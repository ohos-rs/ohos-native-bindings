//! The system callback only attempts a bounded queue send. User work runs on the
//! dispatcher (or synchronously for a cached subscription replay), never under
//! registry/record locks. The trampoline is registered once for process lifetime.
#![cfg_attr(not(target_env = "ohos"), allow(dead_code))]

use std::{
    collections::{BTreeMap, VecDeque},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, Weak,
    },
    time::{Duration, Instant},
};

use crate::{contain_panic, ChildProcessId, NativeChildProcessError as Error};

const MAX_RECORDS: usize = 64;
const MAX_SUBSCRIBERS: usize = 64;
const MAX_EARLY: usize = 64;
const STALE_WINDOW: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChildExitOutcome {
    /// Exact SDK signal (including zero), not a fabricated POSIX wait status.
    Signal(i32),
    /// Queue overflow or PID-reuse ordering is ambiguous. Fall back to control
    /// EOF/handshake observation; this does NOT assert that the child exited.
    ObservationLost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildProcessExitEvent {
    pub pid: ChildProcessId,
    pub generation: u64,
    pub outcome: ChildExitOutcome,
}

type Callback = dyn Fn(ChildProcessExitEvent) + Send + Sync + 'static;
pub(crate) struct Subscriber {
    active: AtomicBool,
    callback: Arc<Callback>,
}

impl Subscriber {
    fn invoke(&self, event: ChildProcessExitEvent) {
        if self
            .active
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            let _ = contain_panic(|| (self.callback)(event));
        }
    }
}

struct RecordState {
    terminal: Option<ChildProcessExitEvent>,
    next_subscriber: u64,
    subscribers: BTreeMap<u64, Arc<Subscriber>>,
}

pub(crate) struct Record {
    pub(crate) pid: ChildProcessId,
    pub(crate) generation: u64,
    pub(crate) ambiguous: AtomicBool,
    born: Instant,
    state: Mutex<RecordState>,
}

impl Record {
    fn new(pid: ChildProcessId, generation: u64, ambiguous: bool) -> Self {
        Self {
            pid,
            generation,
            ambiguous: AtomicBool::new(ambiguous),
            born: Instant::now(),
            state: Mutex::new(RecordState {
                terminal: None,
                next_subscriber: 0,
                subscribers: BTreeMap::new(),
            }),
        }
    }
    pub(crate) fn subscribe(
        self: &Arc<Self>,
        callback: impl Fn(ChildProcessExitEvent) + Send + Sync + 'static,
    ) -> Result<ChildProcessExitSubscription, Error> {
        let subscriber = Arc::new(Subscriber {
            active: AtomicBool::new(true),
            callback: Arc::new(callback),
        });
        let (id, terminal) = {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if state.subscribers.len() == MAX_SUBSCRIBERS {
                return Err(Error::CapacityExceeded);
            }
            state.next_subscriber = state
                .next_subscriber
                .checked_add(1)
                .ok_or(Error::CapacityExceeded)?;
            let id = state.next_subscriber;
            let terminal = state.terminal;
            if terminal.is_none() {
                state.subscribers.insert(id, subscriber.clone());
            }
            (id, terminal)
        };
        // Replay must also invoke user code outside all locks.
        if let Some(event) = terminal {
            subscriber.invoke(event);
        }
        Ok(ChildProcessExitSubscription {
            record: Arc::downgrade(self),
            id,
            subscriber,
        })
    }
    fn observe(&self, outcome: ChildExitOutcome) -> Vec<Delivery> {
        let event = ChildProcessExitEvent {
            pid: self.pid,
            generation: self.generation,
            outcome,
        };
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if state.terminal.is_some() {
            return Vec::new();
        }
        state.terminal = Some(event);
        std::mem::take(&mut state.subscribers)
            .into_values()
            .map(|subscriber| (subscriber, event))
            .collect()
    }
    #[cfg(any(feature = "api-22", test))]
    pub(crate) fn terminal(&self) -> Option<ChildProcessExitEvent> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .terminal
    }
}

/// One-shot subscriber. Drop removes it exactly once, without killing/joining
/// the child or blocking for user code. A callback already in flight may finish.
/// User callbacks should enqueue short work; slow work delays other dispatches,
/// but never blocks the system callback. Panics are contained.
pub struct ChildProcessExitSubscription {
    record: Weak<Record>,
    id: u64,
    subscriber: Arc<Subscriber>,
}

impl Drop for ChildProcessExitSubscription {
    fn drop(&mut self) {
        self.subscriber.active.store(false, Ordering::Release);
        if let Some(record) = self.record.upgrade() {
            record
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .subscribers
                .remove(&self.id);
        }
    }
}

pub(crate) type Delivery = (Arc<Subscriber>, ChildProcessExitEvent);
pub(crate) fn deliver(deliveries: Vec<Delivery>) {
    for (subscriber, event) in deliveries {
        subscriber.invoke(event);
    }
}

#[derive(Clone, Copy)]
pub(crate) struct QueuedExit {
    pub(crate) pid: i32,
    pub(crate) signal: i32,
    pub(crate) launching: u64,
    pub(crate) time: Instant,
}

pub(crate) struct Registry {
    records: BTreeMap<i32, Weak<Record>>,
    recent_pids: VecDeque<(i32, Instant)>,
    early: VecDeque<QueuedExit>,
    launching: Option<u64>,
    launch_lost: bool,
    launch_started: Instant,
}

impl Registry {
    fn new() -> Self {
        Self {
            records: BTreeMap::new(),
            recent_pids: VecDeque::new(),
            early: VecDeque::new(),
            launching: None,
            launch_lost: false,
            launch_started: Instant::now(),
        }
    }
    fn purge(&mut self, now: Instant) {
        self.records.retain(|_, record| record.strong_count() > 0);
        self.recent_pids
            .retain(|(_, time)| now.saturating_duration_since(*time) <= STALE_WINDOW);
        self.early
            .retain(|event| now.saturating_duration_since(event.time) <= STALE_WINDOW);
    }
    fn begin(&mut self, generation: u64, now: Instant) -> Result<(), Error> {
        self.purge(now);
        if self.launching.is_some() {
            return Err(Error::Busy);
        }
        if self.records.len() >= MAX_RECORDS {
            return Err(Error::CapacityExceeded);
        }
        self.early.clear();
        self.launching = Some(generation);
        self.launch_lost = false;
        self.launch_started = now;
        Ok(())
    }
    fn finish(
        &mut self,
        generation: u64,
        result: Result<ChildProcessId, Error>,
        now: Instant,
    ) -> Result<(Arc<Record>, Vec<Delivery>), Error> {
        self.launching = None;
        let pid = match result {
            Ok(pid) => pid,
            Err(error) => {
                self.early.clear();
                return Err(error);
            }
        };
        self.purge(now);
        let ambiguous = self.launch_lost
            || self.recent_pids.iter().any(|(seen, _)| *seen == pid.get())
            || self.records.contains_key(&pid.get());
        let mut record = Record::new(pid, generation, ambiguous);
        record.born = self.launch_started;
        let record = Arc::new(record);
        let mut deliveries = Vec::new();
        // If an old still-held identity is displaced, invalidate it too. It
        // must never authorize a kill against a newly reused PID.
        if let Some(previous) = self
            .records
            .insert(pid.get(), Arc::downgrade(&record))
            .and_then(|record| record.upgrade())
        {
            previous.ambiguous.store(true, Ordering::Release);
            deliveries.extend(previous.observe(ChildExitOutcome::ObservationLost));
        }
        if self.recent_pids.len() == 256 {
            self.recent_pids.pop_front();
        }
        self.recent_pids.push_back((pid.get(), now));
        if ambiguous {
            deliveries.extend(record.observe(ChildExitOutcome::ObservationLost));
        } else if let Some(event) = self
            .early
            .iter()
            .find(|event| event.pid == pid.get() && event.launching == generation)
        {
            deliveries.extend(record.observe(ChildExitOutcome::Signal(event.signal)));
        }
        self.early.clear();
        Ok((record, deliveries))
    }
    fn route(&mut self, event: QueuedExit) -> Vec<Delivery> {
        self.purge(Instant::now());
        if let Some(record) = self.records.get(&event.pid).and_then(Weak::upgrade) {
            // A queue entry captured before this launch cannot become this
            // generation's exit merely because its PID is later reused.
            if event.time < record.born
                || (event.launching != 0 && event.launching < record.generation)
            {
                return Vec::new();
            }
            if Instant::now().saturating_duration_since(event.time) > STALE_WINDOW {
                record.ambiguous.store(true, Ordering::Release);
            }
            if record.ambiguous.load(Ordering::Acquire) {
                return record.observe(ChildExitOutcome::ObservationLost);
            }
            return record.observe(ChildExitOutcome::Signal(event.signal));
        }
        if event.launching != 0 && self.launching == Some(event.launching) {
            if self.early.len() == MAX_EARLY {
                return self.lose_observation();
            }
            self.early.push_back(event);
        }
        Vec::new()
    }
    fn lose_observation(&mut self) -> Vec<Delivery> {
        self.early.clear();
        if self.launching.is_some() {
            self.launch_lost = true;
        }
        self.records
            .values()
            .filter_map(Weak::upgrade)
            .flat_map(|record| {
                record.ambiguous.store(true, Ordering::Release);
                record.observe(ChildExitOutcome::ObservationLost)
            })
            .collect()
    }
    #[cfg(feature = "api-22")]
    pub(crate) fn validate_handle(&self, record: &Arc<Record>) -> Result<(), Error> {
        let current = self
            .records
            .get(&record.pid.get())
            .and_then(Weak::upgrade)
            .ok_or(Error::StaleHandle)?;
        if !Arc::ptr_eq(&current, record) {
            return Err(Error::StaleHandle);
        }
        if record.ambiguous.load(Ordering::Acquire) {
            return Err(Error::ObservationAmbiguous);
        }
        if record.terminal().is_some() {
            return Err(Error::InvalidPid);
        }
        Ok(())
    }
}

#[cfg(target_env = "ohos")]
use std::sync::{
    atomic::AtomicU64,
    mpsc::{sync_channel, SyncSender},
    OnceLock,
};

#[cfg(target_env = "ohos")]
pub(crate) struct Runtime {
    pub(crate) registry: Mutex<Registry>,
    pub(crate) launch_lock: Mutex<()>,
    next_generation: AtomicU64,
    active_generation: AtomicU64,
    tx: SyncSender<QueuedExit>,
    lost: AtomicBool,
}

#[cfg(target_env = "ohos")]
static RUNTIME: OnceLock<Result<Arc<Runtime>, Error>> = OnceLock::new();

#[cfg(target_env = "ohos")]
impl Runtime {
    pub(crate) fn global() -> Result<Arc<Self>, Error> {
        RUNTIME.get_or_init(|| {
            let (tx, rx) = sync_channel(128);
            let runtime = Arc::new(Self { registry: Mutex::new(Registry::new()), launch_lock: Mutex::new(()), next_generation: AtomicU64::new(0), active_generation: AtomicU64::new(0), tx, lost: AtomicBool::new(false) });
            let weak_worker = Arc::downgrade(&runtime);
            std::thread::Builder::new().name("native-child-exit".into()).spawn(move || {
                while let Ok(event) = rx.recv() {
                    let Some(worker) = weak_worker.upgrade() else { break; };
                    let deliveries = {
                        let mut registry = worker.registry.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                        let mut deliveries = registry.route(event);
                        if worker.lost.swap(false, Ordering::AcqRel) { deliveries.extend(registry.lose_observation()); }
                        deliveries
                    };
                    deliver(deliveries);
                }
            }).map_err(|error| Error::Io { operation: "spawn exit dispatcher", code: error.raw_os_error().unwrap_or(0) })?;
            // SAFETY: One static trampoline is installed before any manager
            // launch. Its runtime and worker intentionally live for the process.
            Error::check(unsafe { ohos_native_child_process_sys::OH_Ability_RegisterNativeChildProcessExitCallback(Some(exit_trampoline)) })?;
            Ok(runtime)
        }).clone()
    }

    pub(crate) fn launch(
        &self,
        launch: impl FnOnce() -> Result<ChildProcessId, Error>,
    ) -> Result<Arc<Record>, Error> {
        let _launch = self.launch_lock.try_lock().map_err(|_| Error::Busy)?;
        let generation = self
            .next_generation
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |generation| {
                generation.checked_add(1)
            })
            .map_err(|_| Error::CapacityExceeded)?
            + 1;
        self.registry
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .begin(generation, Instant::now())?;
        self.active_generation.store(generation, Ordering::Release);
        let result = launch();
        let result = self
            .registry
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .finish(generation, result, Instant::now());
        // Keep the capture token until PID association finishes. Otherwise an
        // exit between FFI return and registry insertion could appear unlabeled
        // and be discarded as an unknown event.
        self.active_generation.store(0, Ordering::Release);
        let (record, deliveries) = result?;
        deliver(deliveries);
        Ok(record)
    }
}

#[cfg(target_env = "ohos")]
unsafe extern "C" fn exit_trampoline(pid: i32, signal: i32) {
    let _ = contain_panic(|| {
        if let Some(Ok(runtime)) = RUNTIME.get() {
            let event = QueuedExit {
                pid,
                signal,
                launching: runtime.active_generation.load(Ordering::Acquire),
                time: Instant::now(),
            };
            // Bounded, non-blocking attempt: no user work, no registry lock.
            if runtime.tx.try_send(event).is_err() {
                runtime.lost.store(true, Ordering::Release);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    fn event(pid: i32, generation: u64) -> QueuedExit {
        QueuedExit {
            pid,
            signal: 9,
            launching: generation,
            time: Instant::now(),
        }
    }
    #[test]
    fn early_exit_is_replayed_to_its_generation() {
        let mut registry = Registry::new();
        registry.begin(1, Instant::now()).unwrap();
        assert!(registry.route(event(100, 1)).is_empty());
        let (record, deliveries) = registry
            .finish(1, ChildProcessId::new(100), Instant::now())
            .unwrap();
        deliver(deliveries);
        let result = Arc::new(Mutex::new(None));
        let output = result.clone();
        let _subscription = record
            .subscribe(move |event| *output.lock().unwrap() = Some(event))
            .unwrap();
        let result = result.lock().unwrap().unwrap();
        assert_eq!(result.generation, 1);
        assert_eq!(result.outcome, ChildExitOutcome::Signal(9));
    }
    #[test]
    fn callback_runs_outside_lock_and_panic_does_not_break_delivery() {
        let registry = Arc::new(Mutex::new(Registry::new()));
        registry.lock().unwrap().begin(1, Instant::now()).unwrap();
        let (record, _) = registry
            .lock()
            .unwrap()
            .finish(1, ChildProcessId::new(101), Instant::now())
            .unwrap();
        let check_lock = registry.clone();
        let check_record = record.clone();
        let _panic = record
            .subscribe(move |_| {
                assert!(check_lock.try_lock().is_ok());
                assert!(check_record.state.try_lock().is_ok());
                panic!("user callback");
            })
            .unwrap();
        let count = Arc::new(AtomicUsize::new(0));
        let output = count.clone();
        let _good = record
            .subscribe(move |_| {
                output.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();
        let deliveries = registry.lock().unwrap().route(event(101, 0));
        deliver(deliveries);
        assert_eq!(count.load(Ordering::Relaxed), 1);
        deliver(registry.lock().unwrap().route(event(101, 0)));
        assert_eq!(count.load(Ordering::Relaxed), 1);
    }
    #[test]
    fn subscription_drop_cancels_and_removes_once() {
        let record = Arc::new(Record::new(ChildProcessId::new(102).unwrap(), 1, false));
        let subscription = record.subscribe(|_| panic!("cancelled callback")).unwrap();
        assert_eq!(record.state.lock().unwrap().subscribers.len(), 1);
        drop(subscription);
        assert!(record.state.lock().unwrap().subscribers.is_empty());
        deliver(record.observe(ChildExitOutcome::Signal(0)));
    }
    #[test]
    fn pid_reuse_is_ambiguous_not_a_new_generation_exit() {
        let mut registry = Registry::new();
        registry.begin(1, Instant::now()).unwrap();
        let (old, _) = registry
            .finish(1, ChildProcessId::new(103), Instant::now())
            .unwrap();
        registry.begin(2, Instant::now()).unwrap();
        let (new, deliveries) = registry
            .finish(2, ChildProcessId::new(103), Instant::now())
            .unwrap();
        deliver(deliveries);
        assert!(old.ambiguous.load(Ordering::Acquire));
        assert_eq!(
            new.terminal().unwrap().outcome,
            ChildExitOutcome::ObservationLost
        );
        #[cfg(feature = "api-22")]
        {
            assert_eq!(registry.validate_handle(&old), Err(Error::StaleHandle));
            assert_eq!(
                registry.validate_handle(&new),
                Err(Error::ObservationAmbiguous)
            );
        }
    }
    #[test]
    fn failed_launch_and_expired_early_exits_do_not_cross_generations() {
        let mut registry = Registry::new();
        registry.begin(1, Instant::now()).unwrap();
        registry.route(event(104, 1));
        assert!(registry
            .finish(1, Err(Error::Timeout), Instant::now())
            .is_err());
        registry.begin(2, Instant::now()).unwrap();
        let (record, _) = registry
            .finish(2, ChildProcessId::new(104), Instant::now())
            .unwrap();
        assert_eq!(record.terminal(), None);
        registry.begin(3, Instant::now()).unwrap();
        let mut stale = event(105, 3);
        stale.time -= STALE_WINDOW + Duration::from_secs(1);
        registry.route(stale);
        let (record, _) = registry
            .finish(3, ChildProcessId::new(105), Instant::now())
            .unwrap();
        assert_eq!(record.terminal(), None);
    }
    #[test]
    fn overflow_is_observation_loss_not_a_fabricated_signal() {
        let mut registry = Registry::new();
        registry.begin(1, Instant::now()).unwrap();
        let (record, _) = registry
            .finish(1, ChildProcessId::new(106), Instant::now())
            .unwrap();
        deliver(registry.lose_observation());
        assert_eq!(
            record.terminal().unwrap().outcome,
            ChildExitOutcome::ObservationLost
        );
    }
    #[test]
    fn pending_launch_overflow_marks_only_observation_not_fake_exit() {
        let mut registry = Registry::new();
        registry.begin(1, Instant::now()).unwrap();
        deliver(registry.lose_observation());
        let (record, _) = registry
            .finish(1, ChildProcessId::new(107), Instant::now())
            .unwrap();
        assert_eq!(
            record.terminal().unwrap().outcome,
            ChildExitOutcome::ObservationLost
        );
    }
    #[test]
    fn captured_old_event_cannot_poison_new_identity() {
        let mut registry = Registry::new();
        let mut old = event(108, 0);
        old.time -= Duration::from_secs(1);
        registry.begin(2, Instant::now()).unwrap();
        let (record, _) = registry
            .finish(2, ChildProcessId::new(108), Instant::now())
            .unwrap();
        assert!(registry.route(old).is_empty());
        assert_eq!(record.terminal(), None);
        assert!(registry.route(event(108, 1)).is_empty());
        assert_eq!(record.terminal(), None);
    }
    #[test]
    fn bounded_registries_reject_before_launch_or_subscription() {
        let mut registry = Registry::new();
        let mut records = Vec::new();
        for index in 0..MAX_RECORDS {
            registry.begin(index as u64 + 1, Instant::now()).unwrap();
            records.push(
                registry
                    .finish(
                        index as u64 + 1,
                        ChildProcessId::new(1000 + index as i32),
                        Instant::now(),
                    )
                    .unwrap()
                    .0,
            );
        }
        assert_eq!(
            registry.begin(100, Instant::now()),
            Err(Error::CapacityExceeded)
        );
        let mut subscriptions = Vec::new();
        for _ in 0..MAX_SUBSCRIBERS {
            subscriptions.push(records[0].subscribe(|_| {}).unwrap());
        }
        assert!(matches!(
            records[0].subscribe(|_| {}),
            Err(Error::CapacityExceeded)
        ));
        drop(subscriptions);
        assert!(records[0].state.lock().unwrap().subscribers.is_empty());
    }
}

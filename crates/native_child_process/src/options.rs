use crate::sys;

/// Data-sandbox and network sharing for an extended native child.
/// This does not select an independent UIAbility or HAP process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IsolationMode {
    /// Share the parent's data sandbox and network environment.
    #[default]
    Normal,
    /// Use an isolated data sandbox and network environment.
    Isolated,
}

impl IsolationMode {
    pub(crate) const fn raw(self) -> sys::NativeChildProcess_IsolationMode {
        match self {
            Self::Normal => sys::NativeChildProcess_IsolationMode_NCP_ISOLATION_MODE_NORMAL,
            Self::Isolated => sys::NativeChildProcess_IsolationMode_NCP_ISOLATION_MODE_ISOLATED,
        }
    }
}

/// The official launch options. The native reserved field is always zero.
#[derive(Debug, Clone, Copy, Default)]
pub struct ChildProcessOptions {
    pub isolation_mode: IsolationMode,
}

impl ChildProcessOptions {
    pub(crate) const fn raw(self) -> sys::NativeChildProcess_Options {
        sys::NativeChildProcess_Options {
            isolationMode: self.isolation_mode.raw(),
            reserved: 0,
        }
    }
}

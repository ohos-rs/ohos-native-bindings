# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Breaking

- `ElementInfo::set_operation_actions` takes `&[(ActionType, &'static CStr)]`
  instead of string slices. Use static C literals or caller-managed
  process-lifetime storage; the binding does not intern dynamic labels.
- `EventInfo<'element>` retains the `ElementInfo` borrow until event destruction.
  Declare the element first and keep it in stable storage while borrowed.

- Require process-lifetime C strings for action descriptions because native snapshots shallow-copy their pointers.
- Tie `EventInfo` to the borrowed element's lifetime when configuring its native element pointer.

- Supply a process-lifetime no-op completion function when an event sender does not request a callback; ArkUI rejects null callbacks.

- Add safe wrappers for ArkUI native accessibility providers, callbacks, elements, and events.

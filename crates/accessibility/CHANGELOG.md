# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- Require process-lifetime C strings for action descriptions because native snapshots shallow-copy their pointers.
- Tie `EventInfo` to the borrowed element's lifetime when configuring its native element pointer.

- Supply a process-lifetime no-op completion function when an event sender does not request a callback; ArkUI rejects null callbacks.

- Add safe wrappers for ArkUI native accessibility providers, callbacks, elements, and events.

# Changelog

## Unreleased 0.1.0

- Add typed extended-child entry/args/options/configs/identity wrappers.
- Own CLOEXEC launch duplicates and entry-scoped single FD adoption.
- Route API20 exit events through a bounded process-global dispatcher with
  early-exit/generation/PID-reuse safeguards and panic containment.
- Gate current args, kill and support query at API17/API22/API26 respectively.
- Add host pure tests and a normal-mode, two-FD NAPI worker example.
- Independent source and real-device acceptance remain pending; not published.

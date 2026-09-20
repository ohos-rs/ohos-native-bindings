# Changelog

## Unreleased 0.1.0

- Design bindings directly around the official Native child-process API.
- Expose synchronous entry starts returning PID and asynchronous IPC creation.
- Own argument strings, borrow launch FDs, and prepare stable C list storage per call.
- Provide borrowed native argument views and an entry ABI macro with panic containment.
- Own native configs with RAII and forward setter validation to the platform.
- Expose explicit process-wide exit callback registration and unregistration.
- Preserve native error codes without mapping individual values.
- Follow native API availability from API12 through API26; re-export generated sys APIs.

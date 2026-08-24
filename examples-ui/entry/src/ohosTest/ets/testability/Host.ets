/**
 * Reserved for host-surface state. The current toolchain (arkdown) cannot
 * register router pages in the entry_test module, so the ohosTest suite
 * cannot loadContent a host page of its own. Instead the main module's
 * OhosTestHost page provides the XComponent / NodeContent / Web surfaces
 * (start the app with `pnpm start` before `pnpm test`): all abilities of a
 * bundle share one process, so the .so modules loaded by the host page
 * expose the same process-wide native state to the test code.
 */
export class BindingHost {
  pageLoaded: boolean = false;
}

export const bindingHost: BindingHost = new BindingHost();

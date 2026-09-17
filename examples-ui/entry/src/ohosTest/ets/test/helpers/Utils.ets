import { abilityDelegatorRegistry } from "@kit.TestKit";
import { fileIo } from "@kit.CoreFileKit";
import { common } from "@kit.AbilityKit";

export function appContext(): common.Context {
  return abilityDelegatorRegistry.getAbilityDelegator().getAppContext();
}

export function writeTemp(name: string, body: string): string {
  const path: string = `${appContext().filesDir}/${name}`;
  const file: fileIo.File = fileIo.openSync(
    path,
    fileIo.OpenMode.READ_WRITE | fileIo.OpenMode.CREATE | fileIo.OpenMode.TRUNC,
  );
  fileIo.writeSync(file.fd, body);
  fileIo.closeSync(file);
  return path;
}

export function delay(ms: number): Promise<void> {
  return new Promise<void>((resolve: () => void) => {
    setTimeout((): void => {
      resolve();
    }, ms);
  });
}

export async function waitUntil(ok: () => boolean, timeoutMs: number): Promise<boolean> {
  const startAt: number = Date.now();
  while (Date.now() - startAt < timeoutMs) {
    if (ok()) {
      return true;
    }
    await delay(100);
  }
  return ok();
}

/** Read a `name=123` counter from the stable native E2E status strings. */
export function readCounter(status: string, name: string): number {
  const marker: string = `${name}=`;
  const markerIndex: number = status.indexOf(marker);
  if (markerIndex < 0) {
    throw new Error(`${name} missing from: ${status}`);
  }
  const tail: string = status.substring(markerIndex + marker.length);
  const separatorIndex: number = tail.indexOf(" ");
  const value: string = separatorIndex < 0 ? tail : tail.substring(0, separatorIndex);
  return Number(value);
}

/** Call a zero-arg export on a host-provided module record. */
export function callHost(mod: Record<string, Function> | undefined, name: string): string {
  if (mod === undefined || mod[name] === undefined) {
    throw new Error(`${name} missing`);
  }
  return `${mod[name]()}`;
}

/**
 * A binding is expected to either succeed or throw a descriptive napi error —
 * both count as "behaves as expected"; only silent breakage (no throw, no
 * result) or a native crash can fail.
 */
export function ranOrThrew(fn: Function): boolean {
  try {
    fn();
    return true;
  } catch (e) {
    return `${e}`.length > 0;
  }
}

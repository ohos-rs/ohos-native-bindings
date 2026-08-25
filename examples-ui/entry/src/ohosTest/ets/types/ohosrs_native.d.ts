/* Unique libohosrs_*.so aliases so ohosTest ESM does not bind system modules. */

declare module "libohosrs_ability_access_control.so" {
  export function checkPermission(name: string): boolean;
  export function smoke(): string;
}

declare module "libohosrs_qos.so" {
  export function currentQos(): string;
  export function resetQos(): void;
  export function setQos(level: string): void;
  export function smoke(): string;
}

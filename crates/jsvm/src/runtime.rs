use ohos_jsvm_sys as sys;

use crate::error::{check_status, non_null, JsvmError, Result};
use crate::Env;

#[derive(Debug)]
pub struct Runtime {
    raw: std::ptr::NonNull<sys::JSVM_VM__>,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        let init_options = sys::JSVM_InitOptions {
            externalReferences: std::ptr::null(),
            argc: std::ptr::null_mut(),
            argv: std::ptr::null_mut(),
            removeFlags: false,
        };

        let init_status = unsafe { sys::OH_JSVM_Init(&init_options) };
        if init_status != sys::JSVM_Status_JSVM_OK
            && init_status != sys::JSVM_Status_JSVM_GENERIC_FAILURE
        {
            return Err(JsvmError::Status(init_status));
        }

        let create_options = sys::JSVM_CreateVMOptions {
            maxOldGenerationSize: 0,
            maxYoungGenerationSize: 0,
            initialOldGenerationSize: 0,
            initialYoungGenerationSize: 0,
            snapshotBlobData: std::ptr::null(),
            snapshotBlobSize: 0,
            isForSnapshotting: false,
        };

        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_JSVM_CreateVM(&create_options, &mut raw) })?;
        Ok(Self {
            raw: non_null(raw, "JSVM_VM")?,
        })
    }

    pub fn as_raw(&self) -> sys::JSVM_VM {
        self.raw.as_ptr()
    }

    pub fn open_scope(&self) -> Result<VmScope<'_>> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_JSVM_OpenVMScope(self.as_raw(), &mut raw) })?;
        Ok(VmScope {
            runtime: self,
            raw: non_null(raw, "JSVM_VMScope")?,
        })
    }

    pub fn create_env(&self) -> Result<Env> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_JSVM_CreateEnv(self.as_raw(), 0, std::ptr::null(), &mut raw)
        })?;
        Env::from_raw(raw)
    }

    pub fn pump_message_loop(&self) -> Result<bool> {
        let mut result = false;
        check_status(unsafe { sys::OH_JSVM_PumpMessageLoop(self.as_raw(), &mut result) })?;
        Ok(result)
    }

    pub fn perform_microtask_checkpoint(&self) -> Result<()> {
        check_status(unsafe { sys::OH_JSVM_PerformMicrotaskCheckpoint(self.as_raw()) })
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        let _ = unsafe { sys::OH_JSVM_DestroyVM(self.as_raw()) };
    }
}

#[derive(Debug)]
pub struct VmScope<'a> {
    runtime: &'a Runtime,
    raw: std::ptr::NonNull<sys::JSVM_VMScope__>,
}

impl<'a> VmScope<'a> {
    pub fn as_raw(&self) -> sys::JSVM_VMScope {
        self.raw.as_ptr()
    }
}

impl Drop for VmScope<'_> {
    fn drop(&mut self) {
        let _ = unsafe { sys::OH_JSVM_CloseVMScope(self.runtime.as_raw(), self.as_raw()) };
    }
}

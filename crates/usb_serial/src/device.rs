use crate::error::{check, checked_u32, Result, UsbSerialError};
use crate::r#type::{FlowControl, SerialParams, Timeout};
use ohos_usb_serial_sys::*;
use std::ptr;
use std::sync::Mutex;

// `OH_UsbSerial_Init` / `OH_UsbSerial_Release` are process-global with no
// context argument, so live `Ddk` guards are counted here and the native calls
// only happen when the count crosses zero.
static DDK_USERS: Mutex<usize> = Mutex::new(0);

/// A live handle on the USB serial DDK.
///
/// The DDK is a process-global resource; guards are reference counted, and the
/// native release only runs once the last guard is dropped. Every open
/// [`SerialDevice`] holds one, so the DDK stays up while devices are in use.
pub struct Ddk(());

impl Ddk {
    pub fn init() -> Result<Self> {
        let mut users = lock_users();
        if *users == 0 {
            // SAFETY: no arguments; initializes the process-global DDK.
            check(unsafe { OH_UsbSerial_Init() })?;
        }
        *users += 1;
        Ok(Ddk(()))
    }

    /// Open the ACM interface `interface_index` of device `device_id`.
    ///
    /// `device_id` comes from the USB DDK (`OH_Usb_GetDevices`); this crate does
    /// not enumerate devices.
    ///
    /// ```no_run
    /// use ohos_usb_serial_binding::{Ddk, SerialParams, Timeout};
    /// # let device_id: u64 = 0;
    ///
    /// let ddk = Ddk::init()?;
    /// let mut dev = ddk.open(device_id, 0)?;
    /// dev.set_params(SerialParams::new(115_200))?;
    /// dev.set_timeout(Timeout::Millis(500))?;
    ///
    /// dev.write(b"AT\r\n")?;
    /// let mut buf = [0u8; 64];
    /// let n = dev.read(&mut buf)?;
    /// # Ok::<(), ohos_usb_serial_binding::UsbSerialError>(())
    /// ```
    pub fn open(&self, device_id: u64, interface_index: u8) -> Result<SerialDevice> {
        let mut raw = ptr::null_mut();
        // SAFETY: `raw` is a local pointer slot the DDK writes the handle into.
        check(unsafe { OH_UsbSerial_Open(device_id, interface_index, &mut raw) })?;
        if raw.is_null() {
            return Err(UsbSerialError::new(
                UsbSerial_DdkRetCode_USB_SERIAL_DDK_DEVICE_NOT_FOUND as i32,
            ));
        }
        Ok(SerialDevice {
            raw,
            _ddk: self.clone(),
        })
    }
}

impl Clone for Ddk {
    fn clone(&self) -> Self {
        // `self` proves the DDK is already initialized.
        *lock_users() += 1;
        Ddk(())
    }
}

impl Drop for Ddk {
    fn drop(&mut self) {
        let mut users = lock_users();
        *users -= 1;
        if *users == 0 {
            // SAFETY: no arguments; releases the process-global DDK.
            unsafe { OH_UsbSerial_Release() };
        }
    }
}

fn lock_users() -> std::sync::MutexGuard<'static, usize> {
    DDK_USERS.lock().unwrap_or_else(|e| e.into_inner())
}

/// An open USB serial (CDC-ACM) device.
///
/// All methods block the calling thread until the DDK answers; never call them
/// from the UI thread.
///
/// Not `Send`: the DDK does not specify whether a handle may be used from a
/// thread other than the one that opened it, so open the device on the worker
/// thread that will drive it.
pub struct SerialDevice {
    raw: *mut UsbSerial_Device,
    _ddk: Ddk,
}

impl SerialDevice {
    /// Read into `buf`, returning the number of bytes read.
    ///
    /// How long this blocks with no data available is governed by
    /// [`SerialDevice::set_timeout`].
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.is_empty() {
            // The DDK rejects a zero-length buffer.
            return Ok(0);
        }
        let len = checked_u32(buf.len())?;
        let mut read = 0u32;
        // SAFETY: `buf` is valid for `len` bytes, which is what the DDK is told.
        unsafe {
            check(OH_UsbSerial_Read(
                self.raw,
                buf.as_mut_ptr(),
                len,
                &mut read,
            ))?;
        }
        Ok(read as usize)
    }

    /// Write `data`, returning the number of bytes accepted.
    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        if data.is_empty() {
            // The DDK rejects a zero-length buffer.
            return Ok(0);
        }
        let len = checked_u32(data.len())?;
        let mut written = 0u32;
        // SAFETY: the buffer parameter is declared non-const but is the write
        // source, so casting away constness is sound.
        unsafe {
            check(OH_UsbSerial_Write(
                self.raw,
                data.as_ptr().cast_mut(),
                len,
                &mut written,
            ))?;
        }
        Ok(written as usize)
    }

    /// Set the baud rate, keeping the other line parameters.
    ///
    /// On current OpenHarmony backends this also resets the read timeout to
    /// [`Timeout::Immediate`] and the flow control to [`FlowControl::None`];
    /// call [`SerialDevice::set_timeout`] / [`SerialDevice::set_flow_control`]
    /// after (re)configuring line parameters, not before.
    pub fn set_baud_rate(&mut self, baud_rate: u32) -> Result<()> {
        // SAFETY: `self.raw` is a live handle.
        unsafe { check(OH_UsbSerial_SetBaudRate(self.raw, baud_rate)) }
    }

    /// Set baud rate, data bits, stop bits and parity.
    ///
    /// On current OpenHarmony backends this also resets the read timeout to
    /// [`Timeout::Immediate`] and the flow control to [`FlowControl::None`];
    /// call [`SerialDevice::set_timeout`] / [`SerialDevice::set_flow_control`]
    /// after (re)configuring line parameters, not before.
    pub fn set_params(&mut self, params: SerialParams) -> Result<()> {
        let mut raw = params.to_raw();
        // SAFETY: `raw` outlives the call; the DDK only reads it.
        unsafe { check(OH_UsbSerial_SetParams(self.raw, &mut raw)) }
    }

    /// Set the read timeout. Defaults to [`Timeout::Immediate`].
    pub fn set_timeout(&mut self, timeout: Timeout) -> Result<()> {
        // SAFETY: `self.raw` is a live handle.
        unsafe { check(OH_UsbSerial_SetTimeout(self.raw, timeout.as_raw())) }
    }

    /// Set the flow control mode. Defaults to [`FlowControl::None`].
    pub fn set_flow_control(&mut self, flow_control: FlowControl) -> Result<()> {
        // SAFETY: `self.raw` is a live handle.
        unsafe { check(OH_UsbSerial_SetFlowControl(self.raw, flow_control.into())) }
    }

    /// Flush both buffers after a write.
    pub fn flush(&mut self) -> Result<()> {
        // SAFETY: `self.raw` is a live handle.
        unsafe { check(OH_UsbSerial_Flush(self.raw)) }
    }

    /// Discard buffered input.
    pub fn flush_input(&mut self) -> Result<()> {
        // SAFETY: `self.raw` is a live handle.
        unsafe { check(OH_UsbSerial_FlushInput(self.raw)) }
    }

    /// Discard buffered output.
    pub fn flush_output(&mut self) -> Result<()> {
        // SAFETY: `self.raw` is a live handle.
        unsafe { check(OH_UsbSerial_FlushOutput(self.raw)) }
    }

    /// Close the device, reporting failures instead of swallowing them as
    /// [`Drop`] does. The handle is released either way.
    pub fn close(mut self) -> Result<()> {
        // SAFETY: `self.raw` is a live handle and `Close` takes its address.
        let code = unsafe { OH_UsbSerial_Close(&mut self.raw) };
        self.raw = ptr::null_mut();
        check(code)
    }
}

impl Drop for SerialDevice {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }
        // SAFETY: `self.raw` is a live handle and `Close` takes its address.
        unsafe { OH_UsbSerial_Close(&mut self.raw) };
    }
}

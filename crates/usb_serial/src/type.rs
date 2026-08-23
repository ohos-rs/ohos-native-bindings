use ohos_enum_derive::EnumFrom;
use ohos_usb_serial_sys::*;
use std::os::raw::c_int;

/// Flow control mode. Defaults to [`FlowControl::None`] until set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(UsbSerial_FlowControl, "UsbSerial_FlowControl_")]
pub enum FlowControl {
    #[suffix("USB_SERIAL_NO_FLOW_CONTROL")]
    None,
    #[suffix("USB_SERIAL_SOFTWARE_FLOW_CONTROL")]
    Software,
    #[suffix("USB_SERIAL_HARDWARE_FLOW_CONTROL")]
    Hardware,
}

/// Parity used during communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(UsbSerial_Parity, "UsbSerial_Parity_")]
pub enum Parity {
    #[suffix("USB_SERIAL_PARITY_NONE")]
    None,
    #[suffix("USB_SERIAL_PARITY_ODD")]
    Odd,
    #[suffix("USB_SERIAL_PARITY_EVEN")]
    Even,
}

/// Number of stop bits.
///
/// The header comments `UsbSerial_Params::nStopBits` as a half-bit count, but the driver
/// (`usb_serial_linux_adapter`) only accepts `1` or `2` and rejects anything else, so the
/// field carries a literal stop-bit count. 1.5 stop bits has no encoding the backend accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    One,
    Two,
}

impl StopBits {
    fn raw_count(self) -> u8 {
        match self {
            StopBits::One => 1,
            StopBits::Two => 2,
        }
    }
}

/// Line parameters: baud rate, data bits, stop bits and parity.
///
/// [`SerialParams::new`] starts from 8N1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerialParams {
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

impl SerialParams {
    /// 8 data bits, 1 stop bit, no parity at `baud_rate`.
    pub fn new(baud_rate: u32) -> Self {
        SerialParams {
            baud_rate,
            data_bits: 8,
            stop_bits: StopBits::One,
            parity: Parity::None,
        }
    }

    pub fn data_bits(mut self, data_bits: u8) -> Self {
        self.data_bits = data_bits;
        self
    }

    pub fn stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.stop_bits = stop_bits;
        self
    }

    pub fn parity(mut self, parity: Parity) -> Self {
        self.parity = parity;
        self
    }

    pub(crate) fn to_raw(self) -> UsbSerial_Params {
        UsbSerial_Params {
            baudRate: self.baud_rate,
            nDataBits: self.data_bits,
            nStopBits: self.stop_bits.raw_count(),
            parity: UsbSerial_Parity::from(self.parity) as u8,
        }
    }
}

/// Read timeout.
///
/// The driver rounds the value to the nearest 100 ms and rejects anything above
/// 25500 ms with `USB_SERIAL_DDK_INVALID_PARAMETER`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timeout {
    /// Block until data arrives.
    Infinite,
    /// Return at once with whatever is buffered.
    Immediate,
    Millis(u16),
}

impl Timeout {
    pub(crate) fn as_raw(self) -> c_int {
        match self {
            Timeout::Infinite => -1,
            Timeout::Immediate => 0,
            Timeout::Millis(ms) => c_int::from(ms),
        }
    }
}

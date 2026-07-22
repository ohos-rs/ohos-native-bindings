use crate::error::{Result, TeeError};
use ohos_tee_client_sys as sys;
use std::fmt;
use std::str::FromStr;

/// The direction data flows for a parameter or a shared memory block.
///
/// The same three directions are used for shared memory flags
/// (`TEEC_SharedMemCtl`) and for the value / memory reference parameter types
/// (`TEEC_ParamType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Data flows from the client application to the trusted application.
    Input,
    /// Data flows from the trusted application to the client application.
    Output,
    /// Data flows both ways.
    InOut,
}

impl Direction {
    /// The matching `TEEC_SharedMemCtl` flag.
    pub(crate) fn shared_memory_flag(self) -> u32 {
        match self {
            Direction::Input => sys::TEEC_SharedMemCtl_TEEC_MEM_INPUT,
            Direction::Output => sys::TEEC_SharedMemCtl_TEEC_MEM_OUTPUT,
            Direction::InOut => sys::TEEC_SharedMemCtl_TEEC_MEM_INOUT,
        }
    }

    #[allow(non_upper_case_globals)] // the native constants are mixed case
    pub(crate) fn from_shared_memory_flag(flag: u32) -> Option<Self> {
        match flag {
            sys::TEEC_SharedMemCtl_TEEC_MEM_INPUT => Some(Direction::Input),
            sys::TEEC_SharedMemCtl_TEEC_MEM_OUTPUT => Some(Direction::Output),
            sys::TEEC_SharedMemCtl_TEEC_MEM_INOUT => Some(Direction::InOut),
            _ => None,
        }
    }
}

/// How the client application identifies itself when opening a session.
///
/// The GlobalPlatform login methods that need connection data carry it in the
/// variant, so no untyped pointer reaches the caller: the group methods take
/// the identifier of the group the client wants to be connected as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Login {
    /// No login data is provided.
    Public,
    /// The identity of the user running the client process is provided.
    User,
    /// The identity of the given group is provided.
    Group(u32),
    /// The identity of the running client application is provided.
    Application,
    /// The identities of the user running the client process and of the client
    /// application are provided.
    UserApplication,
    /// The identities of the given group and of the client application are
    /// provided.
    GroupApplication(u32),
    /// Login method reserved for the TEE OS.
    Identify,
}

impl Login {
    /// The raw `TEEC_LoginMethod` plus the connection data it requires.
    pub(crate) fn to_raw(self) -> (u32, Option<u32>) {
        match self {
            Login::Public => (sys::TEEC_LoginMethod_TEEC_LOGIN_PUBLIC, None),
            Login::User => (sys::TEEC_LoginMethod_TEEC_LOGIN_USER, None),
            Login::Group(gid) => (sys::TEEC_LoginMethod_TEEC_LOGIN_GROUP, Some(gid)),
            Login::Application => (sys::TEEC_LoginMethod_TEEC_LOGIN_APPLICATION, None),
            Login::UserApplication => (sys::TEEC_LoginMethod_TEEC_LOGIN_USER_APPLICATION, None),
            Login::GroupApplication(gid) => (
                sys::TEEC_LoginMethod_TEEC_LOGIN_GROUP_APPLICATION,
                Some(gid),
            ),
            Login::Identify => (sys::TEEC_LoginMethod_TEEC_LOGIN_IDENTIFY, None),
        }
    }
}

/// The universally unique identifier of a trusted application, as defined in
/// RFC 4122.
///
/// Build one from its canonical text form, from the RFC 4122 byte order, or
/// from the four fields directly.
///
/// ```no_run
/// use ohos_tee_client_binding::Uuid;
///
/// let uuid: Uuid = "79b77788-9789-4a7a-a2be-b60155eef5f3".parse()?;
/// assert_eq!(uuid.to_string(), "79b77788-9789-4a7a-a2be-b60155eef5f3");
/// # Ok::<(), ohos_tee_client_binding::TeeError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid {
    time_low: u32,
    time_mid: u16,
    time_hi_and_version: u16,
    clock_seq_and_node: [u8; 8],
}

impl Uuid {
    /// Build a UUID from its four RFC 4122 fields.
    pub const fn from_fields(
        time_low: u32,
        time_mid: u16,
        time_hi_and_version: u16,
        clock_seq_and_node: [u8; 8],
    ) -> Self {
        Uuid {
            time_low,
            time_mid,
            time_hi_and_version,
            clock_seq_and_node,
        }
    }

    /// Build a UUID from its 16 bytes in RFC 4122 (big-endian) order.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Uuid {
            time_low: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            time_mid: u16::from_be_bytes([bytes[4], bytes[5]]),
            time_hi_and_version: u16::from_be_bytes([bytes[6], bytes[7]]),
            clock_seq_and_node: [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ],
        }
    }

    /// The 16 bytes of the UUID in RFC 4122 (big-endian) order.
    pub const fn to_bytes(self) -> [u8; 16] {
        let low = self.time_low.to_be_bytes();
        let mid = self.time_mid.to_be_bytes();
        let high = self.time_hi_and_version.to_be_bytes();
        let node = self.clock_seq_and_node;
        [
            low[0], low[1], low[2], low[3], mid[0], mid[1], high[0], high[1], node[0], node[1],
            node[2], node[3], node[4], node[5], node[6], node[7],
        ]
    }

    pub(crate) fn to_raw(self) -> sys::TEEC_UUID {
        sys::TEEC_UUID {
            timeLow: self.time_low,
            timeMid: self.time_mid,
            timeHiAndVersion: self.time_hi_and_version,
            clockSeqAndNode: self.clock_seq_and_node,
        }
    }
}

impl From<[u8; 16]> for Uuid {
    fn from(bytes: [u8; 16]) -> Self {
        Uuid::from_bytes(bytes)
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-",
            self.time_low, self.time_mid, self.time_hi_and_version
        )?;
        for (index, byte) in self.clock_seq_and_node.iter().enumerate() {
            if index == 2 {
                f.write_str("-")?;
            }
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl FromStr for Uuid {
    type Err = TeeError;

    fn from_str(text: &str) -> Result<Self> {
        const HYPHENS: [usize; 4] = [8, 13, 18, 23];
        let bytes = text.as_bytes();
        if bytes.len() != 36 || HYPHENS.iter().any(|&index| bytes[index] != b'-') {
            return Err(TeeError::invalid_argument(
                "a UUID must be 36 characters in 8-4-4-4-12 form",
            ));
        }
        let digits: Vec<u8> = bytes
            .iter()
            .enumerate()
            .filter(|(index, _)| !HYPHENS.contains(index))
            .map(|(_, byte)| *byte)
            .collect();
        let mut raw = [0u8; 16];
        for (target, pair) in raw.iter_mut().zip(digits.chunks_exact(2)) {
            let high = hex_value(pair[0])?;
            let low = hex_value(pair[1])?;
            *target = (high << 4) | low;
        }
        Ok(Uuid::from_bytes(raw))
    }
}

fn hex_value(digit: u8) -> Result<u8> {
    match digit {
        b'0'..=b'9' => Ok(digit - b'0'),
        b'a'..=b'f' => Ok(digit - b'a' + 10),
        b'A'..=b'F' => Ok(digit - b'A' + 10),
        _ => Err(TeeError::invalid_argument(
            "a UUID may only contain hexadecimal digits and hyphens",
        )),
    }
}

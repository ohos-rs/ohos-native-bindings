use std::fmt;
use std::str::FromStr;

/// A 48-bit MAC address.
///
/// The native wifi API hands MAC addresses out as fixed-size text fields in the
/// `AA:BB:CC:DD:EE:FF` form; this type holds the six decoded octets instead and
/// renders them back in the same form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    /// Build an address from its six octets.
    pub const fn from_octets(octets: [u8; 6]) -> Self {
        MacAddress(octets)
    }

    /// The six octets of the address.
    pub const fn octets(&self) -> [u8; 6] {
        self.0
    }

    /// Whether the address is the all-zero address, which the wifi service uses
    /// as a placeholder when it has no address to report.
    pub const fn is_unspecified(&self) -> bool {
        matches!(self.0, [0, 0, 0, 0, 0, 0])
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d, e, g] = self.0;
        write!(f, "{a:02X}:{b:02X}:{c:02X}:{d:02X}:{e:02X}:{g:02X}")
    }
}

impl From<[u8; 6]> for MacAddress {
    fn from(octets: [u8; 6]) -> Self {
        MacAddress(octets)
    }
}

impl From<MacAddress> for [u8; 6] {
    fn from(address: MacAddress) -> Self {
        address.0
    }
}

impl FromStr for MacAddress {
    type Err = MacAddressParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut octets = [0u8; 6];
        let mut parts = text.split(':');
        for octet in octets.iter_mut() {
            let part = parts.next().ok_or(MacAddressParseError)?;
            if part.len() != 2 {
                return Err(MacAddressParseError);
            }
            *octet = u8::from_str_radix(part, 16).map_err(|_| MacAddressParseError)?;
        }
        if parts.next().is_some() {
            return Err(MacAddressParseError);
        }
        Ok(MacAddress(octets))
    }
}

/// The error returned when a string is not a MAC address in the
/// `AA:BB:CC:DD:EE:FF` form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddressParseError;

impl fmt::Display for MacAddressParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("expected a MAC address in the AA:BB:CC:DD:EE:FF form")
    }
}

impl std::error::Error for MacAddressParseError {}

/// Decode a fixed-size C MAC address field.
///
/// An empty field means the wifi service withheld the address, typically
/// because a permission was not granted; it is reported as `None` rather than
/// as a parse error.
pub(crate) fn parse_mac_field(
    field: &[std::os::raw::c_char],
) -> Result<Option<MacAddress>, MacAddressParseError> {
    let text = crate::util::c_field_to_string(field);
    if text.is_empty() {
        return Ok(None);
    }
    text.parse().map(Some)
}

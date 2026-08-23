use crate::error::{ColorSpaceError, Result};
use ohos_native_color_space_manager_sys as sys;

/// A standard color space, as understood by the color manager.
///
/// Several native names are aliases of one another (`DISPLAY_SRGB` is `SRGB`,
/// `LINEAR_BT709` is `LINEAR_SRGB`, ...). Each distinct value is one variant
/// here; the alias spellings are available as associated constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorSpaceName {
    /// An unknown color space.
    None,
    /// Based on Adobe RGB.
    AdobeRgb,
    /// Based on SMPTE RP 431-2-2007 and IEC 61966-2.1:1999.
    DciP3,
    /// Based on SMPTE RP 431-2-2007 and IEC 61966-2.1:1999, full range.
    DisplayP3,
    /// The standard red green blue (sRGB) color space based on IEC 61966-2.1:1999.
    Srgb,
    /// A customized color space.
    Custom,
    /// Based on ITU-R BT.709.
    Bt709,
    /// Based on ITU-R BT.601 (EBU primaries).
    Bt601Ebu,
    /// Based on ITU-R BT.601 (SMPTE C primaries).
    Bt601SmpteC,
    /// Based on ITU-R BT.2020, HLG transfer function.
    Bt2020Hlg,
    /// Based on ITU-R BT.2020, PQ transfer function.
    Bt2020Pq,
    /// P3 D65 primaries, HLG transfer function, full range.
    P3Hlg,
    /// P3 D65 primaries, PQ transfer function, full range.
    P3Pq,
    /// Adobe RGB primaries and transfer function, limited range.
    AdobeRgbLimit,
    /// P3 D65 primaries, sRGB transfer function, limited range.
    DisplayP3Limit,
    /// sRGB primaries and transfer function, limited range.
    SrgbLimit,
    /// BT.709 primaries and transfer function, limited range.
    Bt709Limit,
    /// BT.601 EBU primaries, BT.709 transfer function, limited range.
    Bt601EbuLimit,
    /// BT.601 SMPTE C primaries, BT.709 transfer function, limited range.
    Bt601SmpteCLimit,
    /// BT.2020 primaries, HLG transfer function, limited range.
    Bt2020HlgLimit,
    /// BT.2020 primaries, PQ transfer function, limited range.
    Bt2020PqLimit,
    /// P3 D65 primaries, HLG transfer function, limited range.
    P3HlgLimit,
    /// P3 D65 primaries, PQ transfer function, limited range.
    P3PqLimit,
    /// P3 D65 primaries, linear transfer function.
    LinearP3,
    /// sRGB primaries, linear transfer function. Same value as the native
    /// `LINEAR_BT709`.
    LinearSrgb,
    /// BT.2020 primaries, linear transfer function.
    LinearBt2020,
}

impl ColorSpaceName {
    /// Native alias `DISPLAY_SRGB`.
    pub const DISPLAY_SRGB: Self = Self::Srgb;
    /// Native alias `DISPLAY_P3_SRGB`.
    pub const DISPLAY_P3_SRGB: Self = Self::DisplayP3;
    /// Native alias `DISPLAY_P3_HLG`.
    pub const DISPLAY_P3_HLG: Self = Self::P3Hlg;
    /// Native alias `DISPLAY_P3_PQ`.
    pub const DISPLAY_P3_PQ: Self = Self::P3Pq;
    /// Native alias `LINEAR_BT709`.
    pub const LINEAR_BT709: Self = Self::LinearSrgb;

    /// Build a name from its native value.
    ///
    /// The arms are matched through qualified `sys::` paths so that a constant
    /// missing under the current feature set is a compile error instead of a
    /// catch-all binding pattern.
    pub(crate) fn from_raw(value: u32) -> Result<Self> {
        let name = match value {
            sys::ColorSpaceName_NONE => ColorSpaceName::None,
            sys::ColorSpaceName_ADOBE_RGB => ColorSpaceName::AdobeRgb,
            sys::ColorSpaceName_DCI_P3 => ColorSpaceName::DciP3,
            sys::ColorSpaceName_DISPLAY_P3 => ColorSpaceName::DisplayP3,
            sys::ColorSpaceName_SRGB => ColorSpaceName::Srgb,
            sys::ColorSpaceName_CUSTOM => ColorSpaceName::Custom,
            sys::ColorSpaceName_BT709 => ColorSpaceName::Bt709,
            sys::ColorSpaceName_BT601_EBU => ColorSpaceName::Bt601Ebu,
            sys::ColorSpaceName_BT601_SMPTE_C => ColorSpaceName::Bt601SmpteC,
            sys::ColorSpaceName_BT2020_HLG => ColorSpaceName::Bt2020Hlg,
            sys::ColorSpaceName_BT2020_PQ => ColorSpaceName::Bt2020Pq,
            sys::ColorSpaceName_P3_HLG => ColorSpaceName::P3Hlg,
            sys::ColorSpaceName_P3_PQ => ColorSpaceName::P3Pq,
            sys::ColorSpaceName_ADOBE_RGB_LIMIT => ColorSpaceName::AdobeRgbLimit,
            sys::ColorSpaceName_DISPLAY_P3_LIMIT => ColorSpaceName::DisplayP3Limit,
            sys::ColorSpaceName_SRGB_LIMIT => ColorSpaceName::SrgbLimit,
            sys::ColorSpaceName_BT709_LIMIT => ColorSpaceName::Bt709Limit,
            sys::ColorSpaceName_BT601_EBU_LIMIT => ColorSpaceName::Bt601EbuLimit,
            sys::ColorSpaceName_BT601_SMPTE_C_LIMIT => ColorSpaceName::Bt601SmpteCLimit,
            sys::ColorSpaceName_BT2020_HLG_LIMIT => ColorSpaceName::Bt2020HlgLimit,
            sys::ColorSpaceName_BT2020_PQ_LIMIT => ColorSpaceName::Bt2020PqLimit,
            sys::ColorSpaceName_P3_HLG_LIMIT => ColorSpaceName::P3HlgLimit,
            sys::ColorSpaceName_P3_PQ_LIMIT => ColorSpaceName::P3PqLimit,
            sys::ColorSpaceName_LINEAR_P3 => ColorSpaceName::LinearP3,
            sys::ColorSpaceName_LINEAR_SRGB => ColorSpaceName::LinearSrgb,
            sys::ColorSpaceName_LINEAR_BT2020 => ColorSpaceName::LinearBt2020,
            other => return Err(ColorSpaceError::UnknownName(other)),
        };
        Ok(name)
    }

    /// The native value of this name.
    pub(crate) fn to_raw(self) -> u32 {
        match self {
            ColorSpaceName::None => sys::ColorSpaceName_NONE,
            ColorSpaceName::AdobeRgb => sys::ColorSpaceName_ADOBE_RGB,
            ColorSpaceName::DciP3 => sys::ColorSpaceName_DCI_P3,
            ColorSpaceName::DisplayP3 => sys::ColorSpaceName_DISPLAY_P3,
            ColorSpaceName::Srgb => sys::ColorSpaceName_SRGB,
            ColorSpaceName::Custom => sys::ColorSpaceName_CUSTOM,
            ColorSpaceName::Bt709 => sys::ColorSpaceName_BT709,
            ColorSpaceName::Bt601Ebu => sys::ColorSpaceName_BT601_EBU,
            ColorSpaceName::Bt601SmpteC => sys::ColorSpaceName_BT601_SMPTE_C,
            ColorSpaceName::Bt2020Hlg => sys::ColorSpaceName_BT2020_HLG,
            ColorSpaceName::Bt2020Pq => sys::ColorSpaceName_BT2020_PQ,
            ColorSpaceName::P3Hlg => sys::ColorSpaceName_P3_HLG,
            ColorSpaceName::P3Pq => sys::ColorSpaceName_P3_PQ,
            ColorSpaceName::AdobeRgbLimit => sys::ColorSpaceName_ADOBE_RGB_LIMIT,
            ColorSpaceName::DisplayP3Limit => sys::ColorSpaceName_DISPLAY_P3_LIMIT,
            ColorSpaceName::SrgbLimit => sys::ColorSpaceName_SRGB_LIMIT,
            ColorSpaceName::Bt709Limit => sys::ColorSpaceName_BT709_LIMIT,
            ColorSpaceName::Bt601EbuLimit => sys::ColorSpaceName_BT601_EBU_LIMIT,
            ColorSpaceName::Bt601SmpteCLimit => sys::ColorSpaceName_BT601_SMPTE_C_LIMIT,
            ColorSpaceName::Bt2020HlgLimit => sys::ColorSpaceName_BT2020_HLG_LIMIT,
            ColorSpaceName::Bt2020PqLimit => sys::ColorSpaceName_BT2020_PQ_LIMIT,
            ColorSpaceName::P3HlgLimit => sys::ColorSpaceName_P3_HLG_LIMIT,
            ColorSpaceName::P3PqLimit => sys::ColorSpaceName_P3_PQ_LIMIT,
            ColorSpaceName::LinearP3 => sys::ColorSpaceName_LINEAR_P3,
            ColorSpaceName::LinearSrgb => sys::ColorSpaceName_LINEAR_SRGB,
            ColorSpaceName::LinearBt2020 => sys::ColorSpaceName_LINEAR_BT2020,
        }
    }
}

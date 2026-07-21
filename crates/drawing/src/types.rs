use ohos_enum_derive::EnumFrom;
use ohos_native_drawing_sys::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_BlendMode, "OH_Drawing_BlendMode_BLEND_MODE_")]
pub enum BlendMode {
    Clear,
    Src,
    Dst,
    #[default]
    SrcOver,
    DstOver,
    SrcIn,
    DstIn,
    SrcOut,
    DstOut,
    SrcAtop,
    DstAtop,
    Xor,
    Plus,
    Modulate,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Multiply,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_PathDirection, "OH_Drawing_PathDirection_PATH_DIRECTION_")]
pub enum PathDirection {
    #[default]
    Cw,
    Ccw,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_PathFillType, "OH_Drawing_PathFillType_PATH_FILL_TYPE_")]
pub enum PathFillType {
    #[default]
    Winding,
    EvenOdd,
    InverseWinding,
    InverseEvenOdd,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_PenLineCapStyle, "OH_Drawing_PenLineCapStyle_LINE_")]
pub enum LineCap {
    #[default]
    FlatCap,
    SquareCap,
    RoundCap,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_PenLineJoinStyle, "OH_Drawing_PenLineJoinStyle_LINE_")]
pub enum LineJoin {
    #[default]
    MiterJoin,
    RoundJoin,
    BevelJoin,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_CanvasClipOp, "OH_Drawing_CanvasClipOp_")]
pub enum ClipOperation {
    Difference,
    #[default]
    Intersect,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_TileMode, "OH_Drawing_TileMode_")]
pub enum TileMode {
    #[default]
    Clamp,
    Repeat,
    Mirror,
    Decal,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_FilterMode, "OH_Drawing_FilterMode_FILTER_MODE_")]
pub enum FilterMode {
    Nearest,
    #[default]
    Linear,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_MipmapMode, "OH_Drawing_MipmapMode_MIPMAP_MODE_")]
pub enum MipmapMode {
    #[default]
    None,
    Nearest,
    Linear,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_ColorFormat, "OH_Drawing_ColorFormat_COLOR_FORMAT_")]
pub enum ColorFormat {
    #[default]
    Unknown,
    Alpha8,
    Rgb565,
    Argb4444,
    Rgba8888,
    Bgra8888,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_AlphaFormat, "OH_Drawing_AlphaFormat_ALPHA_FORMAT_")]
pub enum AlphaFormat {
    #[default]
    Unknown,
    Opaque,
    Premul,
    Unpremul,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_CornerPos, "OH_Drawing_CornerPos_CORNER_POS_")]
pub enum CornerPosition {
    #[default]
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_TextEncoding, "OH_Drawing_TextEncoding_TEXT_ENCODING_")]
pub enum TextEncoding {
    #[default]
    #[alias("OH_Drawing_TextEncoding_TEXT_ENCODING_UTF8")]
    Utf8,
    #[alias("OH_Drawing_TextEncoding_TEXT_ENCODING_UTF16")]
    Utf16,
    #[alias("OH_Drawing_TextEncoding_TEXT_ENCODING_UTF32")]
    Utf32,
    GlyphId,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_TextDirection, "OH_Drawing_TextDirection_TEXT_DIRECTION_")]
pub enum TextDirection {
    Rtl,
    #[default]
    Ltr,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_TextAlign, "OH_Drawing_TextAlign_TEXT_ALIGN_")]
pub enum TextAlign {
    #[default]
    Left,
    Right,
    Center,
    Justify,
    Start,
    End,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_FontWeight, "OH_Drawing_FontWeight_FONT_WEIGHT_")]
pub enum FontWeight {
    #[alias("OH_Drawing_FontWeight_FONT_WEIGHT_100")]
    W100,
    #[alias("OH_Drawing_FontWeight_FONT_WEIGHT_200")]
    W200,
    #[alias("OH_Drawing_FontWeight_FONT_WEIGHT_300")]
    W300,
    #[default]
    #[alias("OH_Drawing_FontWeight_FONT_WEIGHT_400")]
    W400,
    #[alias("OH_Drawing_FontWeight_FONT_WEIGHT_500")]
    W500,
    #[alias("OH_Drawing_FontWeight_FONT_WEIGHT_600")]
    W600,
    #[alias("OH_Drawing_FontWeight_FONT_WEIGHT_700")]
    W700,
    #[alias("OH_Drawing_FontWeight_FONT_WEIGHT_800")]
    W800,
    #[alias("OH_Drawing_FontWeight_FONT_WEIGHT_900")]
    W900,
}

impl FontWeight {
    pub const fn from_css(weight: u16) -> Self {
        match weight {
            0..=149 => Self::W100,
            150..=249 => Self::W200,
            250..=349 => Self::W300,
            350..=449 => Self::W400,
            450..=549 => Self::W500,
            550..=649 => Self::W600,
            650..=749 => Self::W700,
            750..=849 => Self::W800,
            _ => Self::W900,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_FontWidth, "OH_Drawing_FontWidth_FONT_WIDTH_")]
pub enum FontWidth {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    #[default]
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_FontStyle, "OH_Drawing_FontStyle_FONT_STYLE_")]
pub enum FontSlant {
    #[default]
    Normal,
    Italic,
    Oblique,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FontStyle {
    pub weight: FontWeight,
    pub width: FontWidth,
    pub slant: FontSlant,
}

impl FontStyle {
    pub const fn new(weight: FontWeight, width: FontWidth, slant: FontSlant) -> Self {
        Self {
            weight,
            width,
            slant,
        }
    }

    pub(crate) fn into_raw(self) -> OH_Drawing_FontStyleStruct {
        OH_Drawing_FontStyleStruct {
            weight: self.weight.into(),
            width: self.width.into(),
            slant: self.slant.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_FontHinting, "OH_Drawing_FontHinting_FONT_HINTING_")]
pub enum FontHinting {
    None,
    Slight,
    #[default]
    Normal,
    Full,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_FontEdging, "OH_Drawing_FontEdging_FONT_EDGING_")]
pub enum FontEdging {
    Alias,
    #[default]
    AntiAlias,
    SubpixelAntiAlias,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitmapFormat {
    pub color: ColorFormat,
    pub alpha: AlphaFormat,
}

impl Default for BitmapFormat {
    fn default() -> Self {
        Self {
            color: ColorFormat::Rgba8888,
            alpha: AlphaFormat::Premul,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    pub width: i32,
    pub height: i32,
    pub color: ColorFormat,
    pub alpha: AlphaFormat,
}

impl ImageInfo {
    pub const fn new(width: i32, height: i32, color: ColorFormat, alpha: AlphaFormat) -> Self {
        Self {
            width,
            height,
            color,
            alpha,
        }
    }

    pub(crate) fn into_raw(self) -> OH_Drawing_Image_Info {
        OH_Drawing_Image_Info {
            width: self.width,
            height: self.height,
            colorType: self.color.into(),
            alphaType: self.alpha.into(),
        }
    }
}

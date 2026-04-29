//! Module animate::options::frame_rate_range wrappers and related types.

use ohos_arkui_sys::ArkUI_ExpectedFrameRateRange;

#[derive(Debug, Clone, Copy)]
/// Wrapper for expected frame-rate range used by animation options.
pub struct AnimationFrameRateRange {
    raw: ArkUI_ExpectedFrameRateRange,
}

impl AnimationFrameRateRange {
    /// Creates a default frame-rate range.
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn raw(&self) -> ArkUI_ExpectedFrameRateRange {
        self.raw
    }

    pub(crate) fn from_raw(raw: ArkUI_ExpectedFrameRateRange) -> Self {
        Self { raw }
    }

    pub fn set_min(&mut self, min: u32) {
        self.raw.min = min;
    }

    pub fn min(&self) -> u32 {
        self.raw.min
    }

    pub fn set_max(&mut self, max: u32) {
        self.raw.max = max;
    }

    pub fn max(&self) -> u32 {
        self.raw.max
    }

    pub fn set_expected(&mut self, expected: u32) {
        self.raw.expected = expected;
    }

    pub fn expected(&self) -> u32 {
        self.raw.expected
    }
}

impl Default for AnimationFrameRateRange {
    fn default() -> Self {
        Self {
            raw: ArkUI_ExpectedFrameRateRange {
                min: 10,
                max: 120,
                expected: 60,
            },
        }
    }
}

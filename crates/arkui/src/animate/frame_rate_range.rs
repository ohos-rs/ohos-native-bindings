use std::{cell::RefCell, rc::Rc};

use ohos_arkui_sys::ArkUI_ExpectedFrameRateRange;

pub struct AnimationFrameRateRange(pub(crate) Rc<RefCell<ArkUI_ExpectedFrameRateRange>>);

impl AnimationFrameRateRange {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(ArkUI_ExpectedFrameRateRange {
            min: 10,
            max: 120,
            expected: 60,
        })))
    }

    pub fn raw(&self) -> ArkUI_ExpectedFrameRateRange {
        let inner = self.0.borrow();
        inner.to_owned()
    }

    pub fn min(&self, min: u32) {
        let mut inner = self.0.borrow_mut();
        inner.min = min;
    }

    pub fn max(&self, max: u32) {
        let mut inner = self.0.borrow_mut();
        inner.max = max;
    }

    pub fn expected(&self, expected: u32) {
        let mut inner = self.0.borrow_mut();
        inner.expected = expected;
    }
}

impl Default for AnimationFrameRateRange {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(ArkUI_ExpectedFrameRateRange {
            min: 10,
            max: 120,
            expected: 60,
        })))
    }
}

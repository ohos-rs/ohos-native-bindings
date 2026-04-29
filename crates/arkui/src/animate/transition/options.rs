//! Module animate::transition::options wrappers and related types.

use ohos_arkui_sys::{ArkUI_RotationOptions, ArkUI_ScaleOptions, ArkUI_TranslationOptions};

#[derive(Debug, Clone, Copy)]
/// Translation transition parameters.
pub struct TranslationOptions {
    raw: ArkUI_TranslationOptions,
}

impl TranslationOptions {
    /// Creates translation options from xyz offsets.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            raw: ArkUI_TranslationOptions { x, y, z },
        }
    }

    pub fn x(&mut self, value: f32) -> &mut Self {
        self.raw.x = value;
        self
    }

    pub fn y(&mut self, value: f32) -> &mut Self {
        self.raw.y = value;
        self
    }

    pub fn z(&mut self, value: f32) -> &mut Self {
        self.raw.z = value;
        self
    }

    pub fn get_x(&self) -> f32 {
        self.raw.x
    }

    pub fn get_y(&self) -> f32 {
        self.raw.y
    }

    pub fn get_z(&self) -> f32 {
        self.raw.z
    }

    pub(crate) fn raw_mut(&mut self) -> *mut ArkUI_TranslationOptions {
        &mut self.raw
    }
}

impl Default for TranslationOptions {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy)]
/// Scale transition parameters.
pub struct ScaleOptions {
    raw: ArkUI_ScaleOptions,
}

impl ScaleOptions {
    /// Creates scale options with center point.
    pub fn new(x: f32, y: f32, z: f32, center_x: f32, center_y: f32) -> Self {
        Self {
            raw: ArkUI_ScaleOptions {
                x,
                y,
                z,
                centerX: center_x,
                centerY: center_y,
            },
        }
    }

    pub fn x(&mut self, value: f32) -> &mut Self {
        self.raw.x = value;
        self
    }

    pub fn y(&mut self, value: f32) -> &mut Self {
        self.raw.y = value;
        self
    }

    pub fn z(&mut self, value: f32) -> &mut Self {
        self.raw.z = value;
        self
    }

    pub fn center_x(&mut self, value: f32) -> &mut Self {
        self.raw.centerX = value;
        self
    }

    pub fn center_y(&mut self, value: f32) -> &mut Self {
        self.raw.centerY = value;
        self
    }

    pub fn get_x(&self) -> f32 {
        self.raw.x
    }

    pub fn get_y(&self) -> f32 {
        self.raw.y
    }

    pub fn get_z(&self) -> f32 {
        self.raw.z
    }

    pub fn get_center_x(&self) -> f32 {
        self.raw.centerX
    }

    pub fn get_center_y(&self) -> f32 {
        self.raw.centerY
    }

    pub(crate) fn raw_mut(&mut self) -> *mut ArkUI_ScaleOptions {
        &mut self.raw
    }
}

impl Default for ScaleOptions {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy)]
/// Rotation transition parameters.
pub struct RotationOptions {
    raw: ArkUI_RotationOptions,
}

impl RotationOptions {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        angle: f32,
        center_x: f32,
        center_y: f32,
        center_z: f32,
        perspective: f32,
    ) -> Self {
        Self {
            raw: ArkUI_RotationOptions {
                x,
                y,
                z,
                angle,
                centerX: center_x,
                centerY: center_y,
                centerZ: center_z,
                perspective,
            },
        }
    }

    pub fn axis_x(&mut self, value: f32) -> &mut Self {
        self.raw.x = value;
        self
    }

    pub fn axis_y(&mut self, value: f32) -> &mut Self {
        self.raw.y = value;
        self
    }

    pub fn axis_z(&mut self, value: f32) -> &mut Self {
        self.raw.z = value;
        self
    }

    pub fn angle(&mut self, value: f32) -> &mut Self {
        self.raw.angle = value;
        self
    }

    pub fn center_x(&mut self, value: f32) -> &mut Self {
        self.raw.centerX = value;
        self
    }

    pub fn center_y(&mut self, value: f32) -> &mut Self {
        self.raw.centerY = value;
        self
    }

    pub fn center_z(&mut self, value: f32) -> &mut Self {
        self.raw.centerZ = value;
        self
    }

    pub fn perspective(&mut self, value: f32) -> &mut Self {
        self.raw.perspective = value;
        self
    }

    pub fn get_axis_x(&self) -> f32 {
        self.raw.x
    }

    pub fn get_axis_y(&self) -> f32 {
        self.raw.y
    }

    pub fn get_axis_z(&self) -> f32 {
        self.raw.z
    }

    pub fn get_angle(&self) -> f32 {
        self.raw.angle
    }

    pub fn get_center_x(&self) -> f32 {
        self.raw.centerX
    }

    pub fn get_center_y(&self) -> f32 {
        self.raw.centerY
    }

    pub fn get_center_z(&self) -> f32 {
        self.raw.centerZ
    }

    pub fn get_perspective(&self) -> f32 {
        self.raw.perspective
    }

    pub(crate) fn raw_mut(&mut self) -> *mut ArkUI_RotationOptions {
        &mut self.raw
    }
}

impl Default for RotationOptions {
    fn default() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }
}

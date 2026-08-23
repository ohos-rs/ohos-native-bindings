use ohos_dlp_permission_sys as sys;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

/// Access permission granted on a DLP file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileAccess {
    /// No permission.
    NoPermission,
    /// Read-only.
    ReadOnly,
    /// Content may be edited.
    ContentEdit,
    /// Full control.
    FullControl,
}

impl FileAccess {
    /// Convert a raw `DLP_FileAccess` value, returning `None` for values
    /// outside the documented enumeration.
    ///
    /// The constants are matched through qualified `sys::` paths so that an arm
    /// whose constant is absent under the current feature set is a compile
    /// error instead of a catch-all binding pattern.
    pub(crate) fn from_raw(raw: sys::DLP_FileAccess) -> Option<Self> {
        match raw {
            sys::DLP_FileAccess_NO_PERMISSION => Some(FileAccess::NoPermission),
            sys::DLP_FileAccess_READ_ONLY => Some(FileAccess::ReadOnly),
            sys::DLP_FileAccess_CONTENT_EDIT => Some(FileAccess::ContentEdit),
            sys::DLP_FileAccess_FULL_CONTROL => Some(FileAccess::FullControl),
            _ => None,
        }
    }

    /// The numeric value the native API uses for this permission.
    pub fn bits(self) -> u32 {
        match self {
            FileAccess::NoPermission => 0,
            FileAccess::ReadOnly => 1,
            FileAccess::ContentEdit => 2,
            FileAccess::FullControl => 3,
        }
    }
}

/// Set of actions allowed on a DLP file.
///
/// This is a bit set; combine members with `|` and test them with
/// [`Actions::contains`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Actions(u32);

impl Actions {
    /// No action is allowed.
    pub const NONE: Actions = Actions(0x0000_0000);
    /// The file may be viewed.
    pub const VIEW: Actions = Actions(0x0000_0001);
    /// The file may be saved.
    pub const SAVE: Actions = Actions(0x0000_0002);
    /// The file may be saved under another name.
    pub const SAVE_AS: Actions = Actions(0x0000_0004);
    /// The file may be edited.
    pub const EDIT: Actions = Actions(0x0000_0008);
    /// The file may be captured in a screenshot.
    pub const SCREEN_CAPTURE: Actions = Actions(0x0000_0010);
    /// The file may be shown while sharing the screen.
    pub const SCREEN_SHARE: Actions = Actions(0x0000_0020);
    /// The file may be captured in a screen recording.
    pub const SCREEN_RECORD: Actions = Actions(0x0000_0040);
    /// The file content may be copied.
    pub const COPY: Actions = Actions(0x0000_0080);
    /// The file may be printed.
    pub const PRINT: Actions = Actions(0x0000_0100);
    /// The file may be exported.
    pub const EXPORT: Actions = Actions(0x0000_0200);
    /// The file permissions may be modified.
    pub const PERMISSION_CHANGE: Actions = Actions(0x0000_0400);

    const NAMED: [(Actions, &'static str); 11] = [
        (Actions::VIEW, "VIEW"),
        (Actions::SAVE, "SAVE"),
        (Actions::SAVE_AS, "SAVE_AS"),
        (Actions::EDIT, "EDIT"),
        (Actions::SCREEN_CAPTURE, "SCREEN_CAPTURE"),
        (Actions::SCREEN_SHARE, "SCREEN_SHARE"),
        (Actions::SCREEN_RECORD, "SCREEN_RECORD"),
        (Actions::COPY, "COPY"),
        (Actions::PRINT, "PRINT"),
        (Actions::EXPORT, "EXPORT"),
        (Actions::PERMISSION_CHANGE, "PERMISSION_CHANGE"),
    ];

    /// Wrap a raw flag word, keeping bits that this crate does not know about.
    ///
    /// The native API may grow new action bits in later versions; they are
    /// preserved rather than dropped.
    pub fn from_bits_retain(bits: u32) -> Self {
        Actions(bits)
    }

    /// The raw flag word.
    pub fn bits(self) -> u32 {
        self.0
    }

    /// Whether no action at all is allowed.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether every action in `other` is allowed.
    pub fn contains(self, other: Actions) -> bool {
        self.0 & other.0 == other.0
    }

    /// Whether at least one action in `other` is allowed.
    pub fn intersects(self, other: Actions) -> bool {
        self.0 & other.0 != 0
    }
}

impl BitOr for Actions {
    type Output = Actions;

    fn bitor(self, rhs: Actions) -> Actions {
        Actions(self.0 | rhs.0)
    }
}

impl BitOrAssign for Actions {
    fn bitor_assign(&mut self, rhs: Actions) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Actions {
    type Output = Actions;

    fn bitand(self, rhs: Actions) -> Actions {
        Actions(self.0 & rhs.0)
    }
}

impl BitAndAssign for Actions {
    fn bitand_assign(&mut self, rhs: Actions) {
        self.0 &= rhs.0;
    }
}

impl fmt::Debug for Actions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "Actions(NONE)");
        }
        write!(f, "Actions(")?;
        let mut first = true;
        let mut rest = self.0;
        for (flag, name) in Actions::NAMED {
            if self.contains(flag) {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "{name}")?;
                first = false;
                rest &= !flag.0;
            }
        }
        if rest != 0 {
            if !first {
                write!(f, " | ")?;
            }
            write!(f, "{rest:#010x}")?;
        }
        write!(f, ")")
    }
}

/// Permission information of the DLP file opened by this sandbox application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PermissionInfo {
    /// Access permission granted on the file.
    pub access: FileAccess,
    /// Actions the current application is allowed to perform on the file.
    pub actions: Actions,
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::{fmt, fmt::Display};

/// Errors that occur when trying to capture OS events.
///
/// <div class="warning">
/// On Mac, not setting accessibility does not cause an error,
/// it justs ignores events.
/// </div>
#[derive(Debug)]
#[non_exhaustive]
pub enum ListenError {
    /// MacOS
    EventTapError,
    /// MacOS
    LoopSourceError,
    /// Linux
    MissingDisplayError,
    /// Linux
    KeyboardError,
    /// Linux
    RecordContextEnablingError,
    /// Linux
    RecordContextError,
    /// Linux
    XRecordExtensionError,
    /// Windows
    KeyHookError(u32),
    /// Windows
    MouseHookError(u32),
}

/// Errors that occur when trying to grab OS events.
///
/// <div class="warning">
/// On Mac, not setting accessibility does not cause an error,
/// it justs ignores events.
/// </div>
#[derive(Debug)]
#[non_exhaustive]
pub enum GrabError {
    /// MacOS
    EventTapError,
    /// MacOS
    LoopSourceError,
    /// Linux
    MissingDisplayError,
    /// Linux
    MissingScreenError,
    /// Linux
    InvalidFileDescriptor,
    /// Linux
    KeyboardError,
    /// Windows
    KeyHookError(u32),
    /// Windows
    MouseHookError(u32),
    /// All
    SimulateError,
    /// All
    ExitGrabError(String),
    /// All
    ListenError,
    /// All
    IoError(std::io::Error),
}

// impl From<std::io::Error> for GrabError {
//     fn from(err: std::io::Error) -> GrabError {
//         GrabError::IoError(err)
//     }
// }

/// Errors that occur when trying to get display size.
#[non_exhaustive]
#[derive(Debug)]
pub enum DisplayError {
    NoDisplay,
    ConversionError,
}

impl From<SimulateError> for GrabError {
    fn from(_: SimulateError) -> GrabError {
        GrabError::SimulateError
    }
}

/// Marking an error when we tried to simulate and event
#[derive(Debug)]
pub struct SimulateError;

impl Display for SimulateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not simulate event")
    }
}

impl std::error::Error for SimulateError {}

// Some keys from https://github.com/chromium/chromium/blob/main/ui/events/keycodes/dom/dom_code_data.inc

use strum_macros::EnumIter; // 0.17.1

/// Key names here assume a QWERTY layout. If you want to detect what actual character was created
/// by a keypress, use [`Event.unicode`](Event::unicode) instead.
///
/// **Warning**: on Windows, [`KpReturn`](Key::KpReturn) does not exist; it' s strictly equivalent to [`Return`](Key::Return). Also, keypad keys
/// get modified if NumLock is off, directly outputting their associated function (e.g. PageDown).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Key {
    /// Alt key on Linux and Windows (option key on macOS)
    Alt,
    AltGr,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    /// Also known as "windows", "super", and "command"
    MetaLeft,
    /// Also known as "windows", "super", and "command"
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    IntlRo,   // Brazilian /? and Japanese _ 'ro'
    IntlYen,  // Japanese Henkan (Convert) key.
    KanaMode, // Japanese Hiragana/Katakana key.
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    KpDecimal,
    KpEqual,
    KpComma,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    Lang1, // Korean Hangul/English toggle key, and as the Kana key on the Apple Japanese keyboard.
    Lang2, // Korean Hanja conversion key, and as the Eisu key on the Apple Japanese keyboard.
    Lang3, // Japanese Katakana key.
    Lang4, // Japanese Hiragana key.
    Lang5, // Japanese Zenkaku/Hankaku (Fullwidth/halfwidth) key.
    Function,
    Apps,
    Cancel,
    Clear,
    Kana,
    Hangul,
    Junja,
    Final,
    Hanja,
    Hanji,
    Print,
    Select,
    Execute,
    Help,
    Sleep,
    Separator,
    Unknown(u32),
    // TODO: Under what circumstances does RawKey get sent?
    RawKey(RawKey),
}

#[cfg(not(target_os = "macos"))]
pub type KeyCode = u32;
#[cfg(target_os = "macos")]
pub type KeyCode = crate::CGKeyCode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RawKey {
    ScanCode(KeyCode),
    WinVirtualKeycode(KeyCode),
    LinuxXorgKeycode(KeyCode),
    LinuxConsoleKeycode(KeyCode),
    MacVirtualKeycode(KeyCode),
}

impl Default for RawKey {
    fn default() -> Self {
        Self::ScanCode(0)
    }
}

/// Standard mouse buttons
/// Some mice have more than 3 buttons. These are not defined, and different
/// OSs will give different `Button::Unknown` values.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Button {
    Left,
    Right,
    Middle,
    Unknown(u8),
}

/// The actual input from an input event. Can either be received from the OS or constructed in
/// code.
///
/// "Button" refers to a mouse button.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EventType {
    KeyPress(Key),
    KeyRelease(Key),
    ButtonPress(Button),
    ButtonRelease(Button),
    /// Contains the cursor's position in pixels with the origin at the top left of the screen.
    MouseMove {
        x: f64,
        y: f64,
    },
    /// Positive delta is up and right.
    Wheel {
        delta_x: i64,
        delta_y: i64,
    },
}

/// The Unicode information of input.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnicodeInfo {
    pub name: Option<String>,
    pub unicode: Vec<u16>,
    pub is_dead: bool,
}

/// An input event received from the OS.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Event {
    /// The time at which the event was received.
    pub time: SystemTime,
    /// For keyboard events, information about the input Unicode character.
    pub unicode: Option<UnicodeInfo>,
    /// Rust-encoded representation of the input.
    ///
    /// Keyboard keys are assumed to be QWERTY layout.
    pub event_type: EventType,
    // TODO: doc the following fields
    // Linux: keysym
    // WIndows: vkcod
    pub platform_code: u32,
    pub position_code: u32,
    pub usb_hid: u32,
    #[cfg(target_os = "windows")]
    pub extra_data: winapi::shared::basetsd::ULONG_PTR,
    #[cfg(target_os = "macos")]
    pub extra_data: i64,
}

// TODO: doc
/// We can define a dummy Keyboard, that we will use to detect
/// what kind of EventType trigger some String. We get the currently used
/// layout for now !
/// Caveat : This is layout dependent. If your app needs to support
/// layout switching don't use this !
/// Caveat: On Linux, the dead keys mechanism is not implemented.
/// Caveat: Only shift and dead keys are implemented, Alt+unicode code on windows
/// won't work.
///
/// ```no_run
/// use rdev::{Keyboard, EventType, Key, KeyboardState};
///
/// let mut keyboard = Keyboard::new().unwrap();
/// let string = keyboard.add(&EventType::KeyPress(Key::KeyS)).unwrap().name.unwrap();
/// // string == Some("s")
/// ```
pub trait KeyboardState {
    /// Changes the keyboard state as if this event happened. we don't
    /// really hit the OS here, which might come handy to test what should happen
    /// if we were to hit said key.
    fn add(&mut self, event_type: &EventType) -> Option<UnicodeInfo>;

    // Resets the keyboard state as if we never touched it (no shift, caps_lock and so on)
    // fn reset(&mut self);
}

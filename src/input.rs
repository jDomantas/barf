#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Key {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
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

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    // TODO: rename
    Back,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    /// The "Compose" key on Linux.
    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,

    AbntC1,
    AbntC2,
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Mute,
    MyComputer,
    // also called "Next"
    NavigateForward,
    // also called "Prior"
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

impl From<winit::event::VirtualKeyCode> for Key {
    fn from(key: winit::event::VirtualKeyCode) -> Key {
        match key {
            winit::event::VirtualKeyCode::Key1 => Key::Key1,
            winit::event::VirtualKeyCode::Key2 => Key::Key2,
            winit::event::VirtualKeyCode::Key3 => Key::Key3,
            winit::event::VirtualKeyCode::Key4 => Key::Key4,
            winit::event::VirtualKeyCode::Key5 => Key::Key5,
            winit::event::VirtualKeyCode::Key6 => Key::Key6,
            winit::event::VirtualKeyCode::Key7 => Key::Key7,
            winit::event::VirtualKeyCode::Key8 => Key::Key8,
            winit::event::VirtualKeyCode::Key9 => Key::Key9,
            winit::event::VirtualKeyCode::Key0 => Key::Key0,
            winit::event::VirtualKeyCode::A => Key::A,
            winit::event::VirtualKeyCode::B => Key::B,
            winit::event::VirtualKeyCode::C => Key::C,
            winit::event::VirtualKeyCode::D => Key::D,
            winit::event::VirtualKeyCode::E => Key::E,
            winit::event::VirtualKeyCode::F => Key::F,
            winit::event::VirtualKeyCode::G => Key::G,
            winit::event::VirtualKeyCode::H => Key::H,
            winit::event::VirtualKeyCode::I => Key::I,
            winit::event::VirtualKeyCode::J => Key::J,
            winit::event::VirtualKeyCode::K => Key::K,
            winit::event::VirtualKeyCode::L => Key::L,
            winit::event::VirtualKeyCode::M => Key::M,
            winit::event::VirtualKeyCode::N => Key::N,
            winit::event::VirtualKeyCode::O => Key::O,
            winit::event::VirtualKeyCode::P => Key::P,
            winit::event::VirtualKeyCode::Q => Key::Q,
            winit::event::VirtualKeyCode::R => Key::R,
            winit::event::VirtualKeyCode::S => Key::S,
            winit::event::VirtualKeyCode::T => Key::T,
            winit::event::VirtualKeyCode::U => Key::U,
            winit::event::VirtualKeyCode::V => Key::V,
            winit::event::VirtualKeyCode::W => Key::W,
            winit::event::VirtualKeyCode::X => Key::X,
            winit::event::VirtualKeyCode::Y => Key::Y,
            winit::event::VirtualKeyCode::Z => Key::Z,
            winit::event::VirtualKeyCode::Escape => Key::Escape,
            winit::event::VirtualKeyCode::F1 => Key::F1,
            winit::event::VirtualKeyCode::F2 => Key::F2,
            winit::event::VirtualKeyCode::F3 => Key::F3,
            winit::event::VirtualKeyCode::F4 => Key::F4,
            winit::event::VirtualKeyCode::F5 => Key::F5,
            winit::event::VirtualKeyCode::F6 => Key::F6,
            winit::event::VirtualKeyCode::F7 => Key::F7,
            winit::event::VirtualKeyCode::F8 => Key::F8,
            winit::event::VirtualKeyCode::F9 => Key::F9,
            winit::event::VirtualKeyCode::F10 => Key::F10,
            winit::event::VirtualKeyCode::F11 => Key::F11,
            winit::event::VirtualKeyCode::F12 => Key::F12,
            winit::event::VirtualKeyCode::F13 => Key::F13,
            winit::event::VirtualKeyCode::F14 => Key::F14,
            winit::event::VirtualKeyCode::F15 => Key::F15,
            winit::event::VirtualKeyCode::F16 => Key::F16,
            winit::event::VirtualKeyCode::F17 => Key::F17,
            winit::event::VirtualKeyCode::F18 => Key::F18,
            winit::event::VirtualKeyCode::F19 => Key::F19,
            winit::event::VirtualKeyCode::F20 => Key::F20,
            winit::event::VirtualKeyCode::F21 => Key::F21,
            winit::event::VirtualKeyCode::F22 => Key::F22,
            winit::event::VirtualKeyCode::F23 => Key::F23,
            winit::event::VirtualKeyCode::F24 => Key::F24,
            winit::event::VirtualKeyCode::Snapshot => Key::Snapshot,
            winit::event::VirtualKeyCode::Scroll => Key::Scroll,
            winit::event::VirtualKeyCode::Pause => Key::Pause,
            winit::event::VirtualKeyCode::Insert => Key::Insert,
            winit::event::VirtualKeyCode::Home => Key::Home,
            winit::event::VirtualKeyCode::Delete => Key::Delete,
            winit::event::VirtualKeyCode::End => Key::End,
            winit::event::VirtualKeyCode::PageDown => Key::PageDown,
            winit::event::VirtualKeyCode::PageUp => Key::PageUp,
            winit::event::VirtualKeyCode::Left => Key::Left,
            winit::event::VirtualKeyCode::Up => Key::Up,
            winit::event::VirtualKeyCode::Right => Key::Right,
            winit::event::VirtualKeyCode::Down => Key::Down,
            winit::event::VirtualKeyCode::Back => Key::Back,
            winit::event::VirtualKeyCode::Return => Key::Return,
            winit::event::VirtualKeyCode::Space => Key::Space,
            winit::event::VirtualKeyCode::Compose => Key::Compose,
            winit::event::VirtualKeyCode::Caret => Key::Caret,
            winit::event::VirtualKeyCode::Numlock => Key::Numlock,
            winit::event::VirtualKeyCode::Numpad0 => Key::Numpad0,
            winit::event::VirtualKeyCode::Numpad1 => Key::Numpad1,
            winit::event::VirtualKeyCode::Numpad2 => Key::Numpad2,
            winit::event::VirtualKeyCode::Numpad3 => Key::Numpad3,
            winit::event::VirtualKeyCode::Numpad4 => Key::Numpad4,
            winit::event::VirtualKeyCode::Numpad5 => Key::Numpad5,
            winit::event::VirtualKeyCode::Numpad6 => Key::Numpad6,
            winit::event::VirtualKeyCode::Numpad7 => Key::Numpad7,
            winit::event::VirtualKeyCode::Numpad8 => Key::Numpad8,
            winit::event::VirtualKeyCode::Numpad9 => Key::Numpad9,
            winit::event::VirtualKeyCode::NumpadAdd => Key::NumpadAdd,
            winit::event::VirtualKeyCode::NumpadDivide => Key::NumpadDivide,
            winit::event::VirtualKeyCode::NumpadDecimal => Key::NumpadDecimal,
            winit::event::VirtualKeyCode::NumpadComma => Key::NumpadComma,
            winit::event::VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
            winit::event::VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
            winit::event::VirtualKeyCode::NumpadMultiply => Key::NumpadMultiply,
            winit::event::VirtualKeyCode::NumpadSubtract => Key::NumpadSubtract,
            winit::event::VirtualKeyCode::AbntC1 => Key::AbntC1,
            winit::event::VirtualKeyCode::AbntC2 => Key::AbntC2,
            winit::event::VirtualKeyCode::Apostrophe => Key::Apostrophe,
            winit::event::VirtualKeyCode::Apps => Key::Apps,
            winit::event::VirtualKeyCode::Asterisk => Key::Asterisk,
            winit::event::VirtualKeyCode::At => Key::At,
            winit::event::VirtualKeyCode::Ax => Key::Ax,
            winit::event::VirtualKeyCode::Backslash => Key::Backslash,
            winit::event::VirtualKeyCode::Calculator => Key::Calculator,
            winit::event::VirtualKeyCode::Capital => Key::Capital,
            winit::event::VirtualKeyCode::Colon => Key::Colon,
            winit::event::VirtualKeyCode::Comma => Key::Comma,
            winit::event::VirtualKeyCode::Convert => Key::Convert,
            winit::event::VirtualKeyCode::Equals => Key::Equals,
            winit::event::VirtualKeyCode::Grave => Key::Grave,
            winit::event::VirtualKeyCode::Kana => Key::Kana,
            winit::event::VirtualKeyCode::Kanji => Key::Kanji,
            winit::event::VirtualKeyCode::LAlt => Key::LAlt,
            winit::event::VirtualKeyCode::LBracket => Key::LBracket,
            winit::event::VirtualKeyCode::LControl => Key::LControl,
            winit::event::VirtualKeyCode::LShift => Key::LShift,
            winit::event::VirtualKeyCode::LWin => Key::LWin,
            winit::event::VirtualKeyCode::Mail => Key::Mail,
            winit::event::VirtualKeyCode::MediaSelect => Key::MediaSelect,
            winit::event::VirtualKeyCode::MediaStop => Key::MediaStop,
            winit::event::VirtualKeyCode::Minus => Key::Minus,
            winit::event::VirtualKeyCode::Mute => Key::Mute,
            winit::event::VirtualKeyCode::MyComputer => Key::MyComputer,
            winit::event::VirtualKeyCode::NavigateForward => Key::NavigateForward,
            winit::event::VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
            winit::event::VirtualKeyCode::NextTrack => Key::NextTrack,
            winit::event::VirtualKeyCode::NoConvert => Key::NoConvert,
            winit::event::VirtualKeyCode::OEM102 => Key::OEM102,
            winit::event::VirtualKeyCode::Period => Key::Period,
            winit::event::VirtualKeyCode::PlayPause => Key::PlayPause,
            winit::event::VirtualKeyCode::Plus => Key::Plus,
            winit::event::VirtualKeyCode::Power => Key::Power,
            winit::event::VirtualKeyCode::PrevTrack => Key::PrevTrack,
            winit::event::VirtualKeyCode::RAlt => Key::RAlt,
            winit::event::VirtualKeyCode::RBracket => Key::RBracket,
            winit::event::VirtualKeyCode::RControl => Key::RControl,
            winit::event::VirtualKeyCode::RShift => Key::RShift,
            winit::event::VirtualKeyCode::RWin => Key::RWin,
            winit::event::VirtualKeyCode::Semicolon => Key::Semicolon,
            winit::event::VirtualKeyCode::Slash => Key::Slash,
            winit::event::VirtualKeyCode::Sleep => Key::Sleep,
            winit::event::VirtualKeyCode::Stop => Key::Stop,
            winit::event::VirtualKeyCode::Sysrq => Key::Sysrq,
            winit::event::VirtualKeyCode::Tab => Key::Tab,
            winit::event::VirtualKeyCode::Underline => Key::Underline,
            winit::event::VirtualKeyCode::Unlabeled => Key::Unlabeled,
            winit::event::VirtualKeyCode::VolumeDown => Key::VolumeDown,
            winit::event::VirtualKeyCode::VolumeUp => Key::VolumeUp,
            winit::event::VirtualKeyCode::Wake => Key::Wake,
            winit::event::VirtualKeyCode::WebBack => Key::WebBack,
            winit::event::VirtualKeyCode::WebFavorites => Key::WebFavorites,
            winit::event::VirtualKeyCode::WebForward => Key::WebForward,
            winit::event::VirtualKeyCode::WebHome => Key::WebHome,
            winit::event::VirtualKeyCode::WebRefresh => Key::WebRefresh,
            winit::event::VirtualKeyCode::WebSearch => Key::WebSearch,
            winit::event::VirtualKeyCode::WebStop => Key::WebStop,
            winit::event::VirtualKeyCode::Yen => Key::Yen,
            winit::event::VirtualKeyCode::Copy => Key::Copy,
            winit::event::VirtualKeyCode::Paste => Key::Paste,
            winit::event::VirtualKeyCode::Cut => Key::Cut,
        }
    }
}

#[derive(Default)]
pub struct Input {
    key_state: [u64; 4],
    pressed_list: Vec<Key>,
    mouse_pos: Option<(u32, u32)>,
    mouse_left_pressed: bool,
    mouse_right_pressed: bool,
    scroll_pos: f32,
}

impl Input {
    pub fn is_pressed(&self, key: Key) -> bool {
        self.get_bit(key)
    }

    pub fn is_mouse_left_pressed(&self) -> bool {
        self.mouse_left_pressed
    }

    pub fn is_mouse_right_pressed(&self) -> bool {
        self.mouse_right_pressed
    }

    pub fn mouse_pos(&self) -> Option<(u32, u32)> {
        self.mouse_pos
    }

    pub fn scroll_pos(&self) -> f32 {
        self.scroll_pos
    }

    pub(crate) fn press_key(&mut self, key: Key) {
        if !self.get_bit(key) {
            self.set_bit(key, true);
            self.pressed_list.push(key);
        }
    }

    pub(crate) fn release_key(&mut self, key: Key) {
        if self.get_bit(key) {
            self.set_bit(key, false);
            self.pressed_list.retain(|&k| k != key);
        }
    }

    pub(crate) fn release_all(&mut self) {
        self.key_state.copy_from_slice(&[0, 0, 0, 0]);
        self.mouse_left_pressed = false;
        self.mouse_right_pressed = false;
        self.mouse_pos = None;
    }

    pub(crate) fn set_mouse_left_press(&mut self, pressed: bool) {
        self.mouse_left_pressed = pressed;
    }

    pub(crate) fn set_mouse_right_press(&mut self, pressed: bool) {
        self.mouse_right_pressed = pressed;
    }

    pub(crate) fn set_mouse_pos(&mut self, pos: Option<(u32, u32)>) {
        self.mouse_pos = pos;
    }

    pub(crate) fn add_mouse_scroll(&mut self, scroll: f32) {
        self.scroll_pos += scroll;
    }

    fn get_bit(&self, idx: Key) -> bool {
        let idx = idx as u32;
        let entry = self.key_state[(idx / 64) as usize];
        let bit = idx % 64;
        (entry >> bit) & 1 != 0
    }

    fn set_bit(&mut self, idx: Key, state: bool) {
        let idx = idx as u32;
        let entry = &mut self.key_state[(idx / 64) as usize];
        let bit = idx % 64;
        let mask = 0xffffffffffffffff ^ (1u64 << bit);
        let state = u64::from(state) << bit;
        *entry = (*entry & mask) | state;
    }
}

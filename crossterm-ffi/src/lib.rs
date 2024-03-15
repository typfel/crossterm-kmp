use std::io::{self, Write};
use std::sync::Arc;
use crossterm::{cursor, event, ExecutableCommand, QueueableCommand, terminal};
use crate::Event::{FocusGained, FocusLost, Key, Mouse, Paste, Resize};

#[derive(uniffi::Record)]
pub struct TerminalSize {
    width: u16,
    height: u16
}

#[derive(uniffi::Enum)]
pub enum ClearType {
    /// All cells.
    All,
    /// All plus history
    Purge,
    /// All cells from the cursor position downwards.
    FromCursorDown,
    /// All cells from the cursor position upwards.
    FromCursorUp,
    /// All cells at the cursor row.
    CurrentLine,
    /// All cells from the cursor position until the new line.
    UntilNewLine,
}

impl ClearType {
    fn as_foo(&self) -> terminal::ClearType {
        match *self {
            ClearType::All => terminal::ClearType::All,
            ClearType::Purge => terminal::ClearType::Purge,
            ClearType::FromCursorDown => terminal::ClearType::FromCursorDown,
            ClearType::FromCursorUp => terminal::ClearType::FromCursorUp,
            ClearType::CurrentLine => terminal::ClearType::CurrentLine,
            ClearType::UntilNewLine => terminal::ClearType::UntilNewLine
        }
    }
}

#[derive(uniffi::Enum)]
pub enum MediaKeyCode {
    Play,
    Pause,
    PlayPause,
    Reverse,
    Stop,
    FastForward,
    Rewind,
    TrackNext,
    TrackPrevious,
    Record,
    LowerVolume,
    RaiseVolume,
    MuteVolume,
}

impl From<event::MediaKeyCode> for MediaKeyCode {
    fn from(value: event::MediaKeyCode) -> Self {
        match value {
            event::MediaKeyCode::Play => MediaKeyCode::Play,
            event::MediaKeyCode::Pause => MediaKeyCode::Pause,
            event::MediaKeyCode::PlayPause => MediaKeyCode::PlayPause,
            event::MediaKeyCode::Reverse => MediaKeyCode::Reverse,
            event::MediaKeyCode::Stop => MediaKeyCode::Stop,
            event::MediaKeyCode::FastForward => MediaKeyCode::FastForward,
            event::MediaKeyCode::Rewind => MediaKeyCode::Rewind,
            event::MediaKeyCode::TrackNext => MediaKeyCode::TrackNext,
            event::MediaKeyCode::TrackPrevious => MediaKeyCode::TrackPrevious,
            event::MediaKeyCode::Record => MediaKeyCode::Record,
            event::MediaKeyCode::LowerVolume => MediaKeyCode::LowerVolume,
            event::MediaKeyCode::RaiseVolume => MediaKeyCode::RaiseVolume,
            event::MediaKeyCode::MuteVolume => MediaKeyCode::MuteVolume
        }
    }
}

#[derive(uniffi::Enum)]
pub enum ModifierKeyCode {
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    LeftHyper,
    LeftMeta,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    RightHyper,
    RightMeta,
    IsoLevel3Shift,
    IsoLevel5Shift,
}

impl From<event::ModifierKeyCode> for ModifierKeyCode {
    fn from(value: event::ModifierKeyCode) -> Self {
        match value {
            event::ModifierKeyCode::LeftShift => ModifierKeyCode::LeftShift,
            event::ModifierKeyCode::LeftControl => ModifierKeyCode::LeftControl,
            event::ModifierKeyCode::LeftAlt => ModifierKeyCode::LeftAlt,
            event::ModifierKeyCode::LeftSuper => ModifierKeyCode::LeftSuper,
            event::ModifierKeyCode::LeftHyper => ModifierKeyCode::LeftHyper,
            event::ModifierKeyCode::LeftMeta => ModifierKeyCode::LeftMeta,
            event::ModifierKeyCode::RightShift => ModifierKeyCode::RightShift,
            event::ModifierKeyCode::RightControl => ModifierKeyCode::RightControl,
            event::ModifierKeyCode::RightAlt => ModifierKeyCode::RightAlt,
            event::ModifierKeyCode::RightSuper => ModifierKeyCode::RightSuper,
            event::ModifierKeyCode::RightHyper => ModifierKeyCode::RightHyper,
            event::ModifierKeyCode::RightMeta => ModifierKeyCode::RightMeta,
            event::ModifierKeyCode::IsoLevel3Shift => ModifierKeyCode::IsoLevel3Shift,
            event::ModifierKeyCode::IsoLevel5Shift => ModifierKeyCode::IsoLevel5Shift
        }
    }
}

#[derive(uniffi::Enum)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F { key: u8 },
    Char { char: String },
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
    Media { code: MediaKeyCode },
    Modifier { code: ModifierKeyCode },
}

impl From<event::KeyCode> for KeyCode {
    fn from(value: event::KeyCode) -> Self {
        match value {
            event::KeyCode::Backspace => KeyCode::Backspace,
            event::KeyCode::Enter => KeyCode::Enter,
            event::KeyCode::Left => KeyCode::Left,
            event::KeyCode::Right => KeyCode::Right,
            event::KeyCode::Up => KeyCode::Up,
            event::KeyCode::Down => KeyCode::Down,
            event::KeyCode::Home => KeyCode::Home,
            event::KeyCode::End => KeyCode::End,
            event::KeyCode::PageUp => KeyCode::PageUp,
            event::KeyCode::PageDown => KeyCode::PageDown,
            event::KeyCode::Tab => KeyCode::Tab,
            event::KeyCode::BackTab => KeyCode::BackTab,
            event::KeyCode::Delete => KeyCode::Delete,
            event::KeyCode::Insert => KeyCode::Insert,
            event::KeyCode::F(key) => KeyCode::F { key },
            event::KeyCode::Char(char) => KeyCode::Char { char: char.to_string() },
            event::KeyCode::Null => KeyCode::Null,
            event::KeyCode::Esc => KeyCode::Esc,
            event::KeyCode::CapsLock => KeyCode::CapsLock,
            event::KeyCode::ScrollLock => KeyCode::ScrollLock,
            event::KeyCode::NumLock => KeyCode::NumLock,
            event::KeyCode::PrintScreen => KeyCode::PrintScreen,
            event::KeyCode::Pause => KeyCode::Pause,
            event::KeyCode::Menu => KeyCode::Menu,
            event::KeyCode::KeypadBegin => KeyCode::KeypadBegin,
            event::KeyCode::Media(key) => KeyCode::Media { code: MediaKeyCode::from(key) },
            event::KeyCode::Modifier(key) => KeyCode::Modifier { code: ModifierKeyCode::from(key) }
        }
    }
}

#[derive(uniffi::Enum)]
pub enum KeyEventKind {
    Press,
    Repeat,
    Release,
}

impl From<event::KeyEventKind> for KeyEventKind {
    fn from(value: event::KeyEventKind) -> Self {
        match value {
            event::KeyEventKind::Press => KeyEventKind::Press,
            event::KeyEventKind::Repeat => KeyEventKind::Repeat,
            event::KeyEventKind::Release => KeyEventKind::Release
        }
    }
}

#[derive(uniffi::Record)]
pub struct KeyEvent {
    pub code: KeyCode,
    // pub modifiers: KeyModifiers, TODO add KeyModifiers support
    pub kind: KeyEventKind,
    // pub state: KeyEventState, TODO add KeyEventState support
}

impl From<crossterm::event::KeyEvent> for KeyEvent {
    fn from(value: crossterm::event::KeyEvent) -> Self {
        KeyEvent {
            code: KeyCode::from(value.code),
            kind: KeyEventKind::from(value.kind)
        }
    }
}

#[derive(uniffi::Enum)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl From<event::MouseButton> for MouseButton {
    fn from(value: event::MouseButton) -> Self {
        match value {
            event::MouseButton::Left => MouseButton::Left,
            event::MouseButton::Middle => MouseButton::Middle,
            event::MouseButton::Right => MouseButton::Right
        }
    }
}

#[derive(uniffi::Enum)]
pub enum MouseEventKind {
    Down { button: MouseButton },
    Up { button: MouseButton },
    Drag { button: MouseButton },
    Moved,
    ScrollDown,
    ScrollUp,
    ScrollLeft,
    ScrollRight,
}

impl From<event::MouseEventKind> for MouseEventKind {
    fn from(value: event::MouseEventKind) -> Self {
        match value {
            event::MouseEventKind::Down(button) => MouseEventKind::Down { button: MouseButton::from(button) },
            event::MouseEventKind::Up(button) => MouseEventKind::Up { button: MouseButton::from(button) },
            event::MouseEventKind::Drag(button) => MouseEventKind::Drag { button: MouseButton::from(button) },
            event::MouseEventKind::Moved => MouseEventKind::Moved,
            event::MouseEventKind::ScrollDown => MouseEventKind::ScrollDown,
            event::MouseEventKind::ScrollUp => MouseEventKind::ScrollUp,
            event::MouseEventKind::ScrollLeft => MouseEventKind::ScrollLeft,
            event::MouseEventKind::ScrollRight => MouseEventKind::ScrollRight,
        }
    }
}

#[derive(uniffi::Record)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    // pub modifiers: KeyModifiers, TODO add KeyModifiers support
}

impl From<crossterm::event::MouseEvent> for MouseEvent {
    fn from(value: event::MouseEvent) -> Self {
        MouseEvent {
            kind: MouseEventKind::from(value.kind),
            column: value.column,
            row: value.row
        }
    }
}

#[derive(uniffi::Enum)]
pub enum Event {
    FocusGained,
    FocusLost,
    Key { event: KeyEvent },
    Mouse { event: MouseEvent },
    Paste,
    Resize { width: u16, height: u16 }
}

impl From<event::Event> for Event {
    fn from(value: event::Event) -> Self {
        match value {
            event::Event::FocusGained => FocusGained,
            event::Event::FocusLost => FocusLost,
            event::Event::Key(event) => Key { event: KeyEvent::from(event) },
            event::Event::Mouse(event) => Mouse { event: MouseEvent::from(event) },
            event::Event::Paste(_) => Paste,
            event::Event::Resize(width, height) => Resize { width, height }
        }
    }
}

#[derive(uniffi::Enum)]
pub enum Command {
    MoveTo { column: u16, row: u16 },
    Clear { clear_type: ClearType },
    SetTitle { title: String },
    EnableMouseCapture,
    DisableMouseCapture,
    EnterAlternateScreen,
    LeaveAlternateScreen
}

#[derive(uniffi::Object)]
pub struct Terminal;

#[uniffi::export]
impl Terminal {

    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub fn size(&self) -> TerminalSize {
        let (width, height) = terminal::size().unwrap();
        TerminalSize { width, height }
    }

    pub fn is_raw_mode_enabled(&self) -> bool {
        terminal::is_raw_mode_enabled().unwrap()
    }

    pub fn enable_raw_mode(&self) {
        terminal::enable_raw_mode().unwrap();
    }

    pub fn disable_raw_mode(&self) {
        terminal::disable_raw_mode().unwrap();
    }

    pub fn queue(&self, commands: Vec<Command>) {
        for command in &commands {
            self.queue_command(command);
        }
    }

    pub fn flush(&self) {
        io::stdout().flush().unwrap();
    }

    pub fn execute(&self, commands: Vec<Command>) {
        for command in &commands {
            self.execute_command(command);
        }
    }

    pub fn read(&self) -> Event {
        Event::from(crossterm::event::read().unwrap())
    }
}

impl Terminal {
    fn queue_command(&self, command: &Command) {
        let mut stdout = io::stdout();
        match command {
            Command::MoveTo { column, row } => stdout.queue(cursor::MoveTo(column.clone(), row.clone())),
            Command::Clear { clear_type } => stdout.queue(terminal::Clear(clear_type.as_foo())),
            Command::SetTitle { title } => stdout.queue(terminal::SetTitle(title)),
            Command::EnableMouseCapture => stdout.queue(event::EnableMouseCapture),
            Command::DisableMouseCapture => stdout.queue(event::DisableMouseCapture),
            Command::EnterAlternateScreen => stdout.queue(terminal::EnterAlternateScreen),
            Command::LeaveAlternateScreen => stdout.queue(terminal::LeaveAlternateScreen)
        }.unwrap();
    }

    fn execute_command(&self, command: &Command) {
        let mut stdout = io::stdout();
        match command {
            Command::MoveTo { column, row } => stdout.execute(cursor::MoveTo(column.clone(), row.clone())),
            Command::Clear { clear_type } => stdout.execute(terminal::Clear(clear_type.as_foo())),
            Command::SetTitle { title } => stdout.execute(terminal::SetTitle(title)),
            Command::EnableMouseCapture => stdout.execute(event::EnableMouseCapture),
            Command::DisableMouseCapture => stdout.execute(event::DisableMouseCapture),
            Command::EnterAlternateScreen => stdout.execute(terminal::EnterAlternateScreen),
            Command::LeaveAlternateScreen => stdout.execute(terminal::LeaveAlternateScreen)
        }.unwrap();
    }
}

uniffi::include_scaffolding!("crossterm");

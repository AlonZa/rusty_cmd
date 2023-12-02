use std::io::{self, stdout};
use std::process::exit;

use crossterm::cursor::{MoveTo, MoveToColumn};
use crossterm::event::{
    poll, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::style::{Print, Color, StyledContent, Stylize};
use crossterm::terminal::{ClearType, Clear};
use crossterm::{
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode, KeyEvent,
    },
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::Duration;

const DEFAULT_PROMPT: &str = "rusty_cmd $ ";

pub struct PrinterTerm {
    cursor: (u16, u16), // (row, col)
    size_limit: (u16, u16), // (col, row)
    line_buffer: String,
    buffer_idx: u16,
}

pub enum Direction {
    Left,
    Right,
    Down(u16),
    Up
}

impl PrinterTerm {
    pub fn new() -> PrinterTerm {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        PrinterTerm { 
            cursor: (0,0),
            size_limit: crossterm::terminal::size().unwrap(),
            line_buffer: String::new(),
            buffer_idx: 0,
        }
    }

    // Resize events can occur in batches.
    // With a simple loop they can be flushed.
    // This function will keep the first and last resize event.
    fn flush_resize_events(&self, first_resize: (u16, u16)) -> ((u16, u16), (u16, u16)) {
        let mut last_resize = first_resize;
        while let Ok(true) = poll(Duration::from_millis(50)) {
            if let Ok(Event::Resize(x, y)) = read() {
                last_resize = (x, y);
            }
        }

        (first_resize, last_resize)
    }

    pub fn read_input(&mut self) -> String {
        
        loop {
            let event: Event = read().unwrap();

            if event == Event::Key(KeyCode::Enter.into()) {
                let line = self.flush_line();
                self.update_cursor(Direction::Down(1));
                return line
                // Just for debugging. Because we print the line
                // self.print_wrapped(&line);
                // End of debug section
            }
            else if event == Event::Key(KeyCode::Tab.into()) {
                let _ = 0; // TODO: Placeholder for `autocomplete_cmd()`
            }
            else if event == Event::Key(KeyCode::Backspace.into()) {
                if self.buffer_idx != 0 {
                    self.buffer_idx -= 1;
                    self.line_buffer.remove(self.buffer_idx as usize);
                    execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
                    execute!(stdout(), MoveToColumn(0)).unwrap();
                    execute!(stdout(), Print(format!("{}", self.line_buffer).bold())).unwrap();
                    self.update_cursor(Direction::Left);
                }
                else {
                    // Cannot delete at index 0;
                    // Do nothing
                }
            }
            else if event == Event::Key(KeyCode::Left.into()) {
                self.buffer_idx -= if self.cursor.0 == 0 {0} else {1};
                self.update_cursor(Direction::Left);
            }
            else if event == Event::Key(KeyCode::Right.into()) {
                if !(self.cursor.0 == self.buffer_idx && self.cursor.0 as usize == self.line_buffer.len()) {
                    // We are at the end of the line
                    self.buffer_idx += if self.cursor.0 as usize == self.line_buffer.len() {0} else {1};
                    self.update_cursor(Direction::Right);
                }
                else {
                    if self.line_buffer.len() > self.cursor.0 as usize {
                        self.buffer_idx += 1;
                        self.update_cursor(Direction::Right);
                    }
                }
            }
            else if event == Event::Key(KeyCode::Esc.into()) {
                exit(0);
            }
            else if let Event::Resize(x, y) = event {
                let (_, new_size) = self.flush_resize_events((x, y));
                self.size_limit = new_size;
            }
            else if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Char(c) => {
                        self.line_buffer.insert(self.buffer_idx as usize, c);
                        self.buffer_idx += 1;
                        execute!(stdout(), Print(format!("{}", &self.line_buffer.as_str()[self.buffer_idx as usize - 1..]).bold())).unwrap();
                        self.update_cursor(Direction::Right);
                    }
                    _ => {}
                }
            }
        };
    }

    pub fn print_wrapped(&mut self, text: &str){
        // self.cursor.1 += 1;
        execute!(stdout(), MoveTo(self.cursor.0, self.cursor.1)).unwrap();
        execute!(stdout(), Print(text)).unwrap();
        self.update_cursor(Direction::Down(text.len() as u16));
    }

    pub fn styled_print_wrapped(&mut self, text: &StyledContent<String>){
        // self.cursor.1 += 1;
        execute!(stdout(), MoveTo(self.cursor.0, self.cursor.1)).unwrap();
        execute!(stdout(), Print(text)).unwrap();
        self.update_cursor(Direction::Down(text.to_string().len() as u16));
    }

    pub fn colored_print(&mut self, text: &str, color: crossterm::style::Color) {
        execute!(stdout(), crossterm::style::SetForegroundColor(color)).unwrap();
        self.print_wrapped(text);
        execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::Reset)).unwrap();
    }

    pub fn colored_styled_print(&mut self, text: &StyledContent<String>, color: crossterm::style::Color) {
        execute!(stdout(), crossterm::style::SetForegroundColor(color)).unwrap();
        self.styled_print_wrapped(text);
        execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::Reset)).unwrap();
    }

    fn update_cursor_no_move(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.cursor.0 == 0 {
                    if self.buffer_idx != 0 {
                        self.cursor.1 -= 1;
                        self.cursor.0 = self.size_limit.0;
                    }
                }
                else {
                    self.cursor.0 -= 1;
                }
            }
            Direction::Right => {
                if self.cursor.0 == self.size_limit.0 {
                    self.cursor.1 += 1;
                    self.cursor.0 = 0;
                }
                else {
                    self.cursor.0 += 1;
                }
            }
            Direction::Down(line_size) => {
                let nr_lines: u16 = line_size / self.size_limit.0;
                if self.cursor.1 + nr_lines + 1 > self.size_limit.1 {
                    // Reached the end of terminal
                    crossterm::execute!(stdout(), crossterm::terminal::ScrollUp(1)).unwrap();
                }
                self.cursor.1 += nr_lines + 1;
                self.cursor.0 = 0;
            }
            _ => {}
        }
    }

    pub fn update_cursor(&mut self, direction: Direction) {
        self.update_cursor_no_move(direction);
        crossterm::execute!(stdout(), crossterm::cursor::MoveTo(self.cursor.0, self.cursor.1)).unwrap();
    }

    pub fn gracefully_quit(&self) {
        queue!(io::stdout(), PopKeyboardEnhancementFlags).unwrap();
    
        execute!(
            io::stdout(),
            DisableBracketedPaste,
            PopKeyboardEnhancementFlags,
            DisableFocusChange,
            DisableMouseCapture
        ).unwrap();
    
        disable_raw_mode().unwrap();

        exit(0);
    }

    pub fn gracefully_start(&self) {
        enable_raw_mode().unwrap();

        let mut stdout = io::stdout();

        let supports_keyboard_enhancement = matches!(
            crossterm::terminal::supports_keyboard_enhancement(),
            Ok(true)
        );

        if supports_keyboard_enhancement {
            queue!(
                stdout,
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                )
            ).unwrap();
        }

        execute!(
            stdout,
            EnableFocusChange,
            EnableMouseCapture,
        ).unwrap();
    }

    fn flush_line(&mut self) -> String{
        let to_ret = self.line_buffer.clone();
        self.line_buffer = String::new();
        self.buffer_idx = 0;

        to_ret
    }
}

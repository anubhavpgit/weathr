use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout, Write};

pub struct TerminalRenderer {
    stdout: Stdout,
    width: u16,
    height: u16,
}

impl TerminalRenderer {
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        let stdout = io::stdout();

        Ok(Self {
            stdout,
            width,
            height,
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen, cursor::Hide)?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        execute!(self.stdout, LeaveAlternateScreen, cursor::Show, ResetColor)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn update_size(&mut self) -> io::Result<()> {
        let (width, height) = terminal::size()?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn clear(&mut self) -> io::Result<()> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    pub fn render_centered(&mut self, lines: &[String], start_row: u16) -> io::Result<()> {
        let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let start_col = if self.width as usize > max_width {
            (self.width as usize - max_width) / 2
        } else {
            0
        };

        for (idx, line) in lines.iter().enumerate() {
            queue!(
                self.stdout,
                cursor::MoveTo(start_col as u16, start_row + idx as u16),
                Print(line)
            )?;
        }

        Ok(())
    }

    pub fn render_line_colored(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        color: Color,
    ) -> io::Result<()> {
        queue!(
            self.stdout,
            cursor::MoveTo(x, y),
            SetForegroundColor(color),
            Print(text),
            ResetColor
        )?;
        Ok(())
    }

    pub fn render_char(&mut self, x: u16, y: u16, ch: char, color: Color) -> io::Result<()> {
        queue!(
            self.stdout,
            cursor::MoveTo(x, y),
            SetForegroundColor(color),
            Print(ch),
            ResetColor
        )?;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

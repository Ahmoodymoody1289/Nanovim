use std::io;
use std::fs;
use crossterm::cursor::MoveTo;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;

pub struct Editor {
    lines: Vec<String>,
    filepath: String,
    cursor_x: usize,
    cursor_y: usize,
    scroll_offset_y: usize,
}

impl Editor {
    pub fn new(filepath: String) -> Self {
        let mut lines = Vec::new();
        
        // Try to read existing file
        if let Ok(content) = fs::read_to_string(&filepath) {
            if content.is_empty() {
                lines.push(String::new());
            } else {
                for line in content.lines() {
                    lines.push(line.to_string());
                }
            }
        } else {
            // File doesn't exist or can't be read - create with empty line
            lines.push(String::new());
        }

        Self {
            lines,
            filepath,
            cursor_x: 0,
            cursor_y: 0,
            scroll_offset_y: 0,
        }
    }

    // Save buffer to file
    pub fn save_file(&self) -> std::io::Result<()> {
        let content = self.lines.join("\n");
        fs::write(&self.filepath, &content)
    }

    fn get_visible_lines(&self) -> usize {
        12
    }

    fn ensure_cursor_visible(&mut self) {
        if self.cursor_y >= self.scroll_offset_y + self.get_visible_lines() {
            self.scroll_offset_y = self.cursor_y - self.get_visible_lines() + 1;
        }
        if self.cursor_y < self.scroll_offset_y {
            self.scroll_offset_y = self.cursor_y;
        }
    }

    fn get_screen_row(&self, line_index: usize) -> u16 {
        let relative = line_index - self.scroll_offset_y;
        3u16 + relative as u16
    }

    fn get_column_offset(&self, line_index: usize) -> u16 {
        let prefix = format!("{}: ", line_index);
        prefix.len() as u16
    }

    pub fn render(&self) -> std::io::Result<()> {
        std::io::stdout().execute(Clear(ClearType::All))?;
        std::io::stdout().execute(MoveTo(0, 0))?;
        
        std::io::stdout().execute(Print(format_args!("Nanovim: {}\r\n", self.filepath)))?;
        std::io::stdout().execute(MoveTo(0, 1))?;
        std::io::stdout().execute(Print(format_args!("Lines: {} | View: {}-{}", 
            self.lines.len(), 
            self.scroll_offset_y, 
            self.scroll_offset_y + self.get_visible_lines())))?;
        std::io::stdout().execute(MoveTo(0, 2))?;
        std::io::stdout().execute(Print("--- Buffer Content ---"))?;
        
        for i in 0..self.get_visible_lines() {
            let line_index = self.scroll_offset_y + i;
            std::io::stdout().execute(MoveTo(0, 3 + i as u16))?;
            
            if line_index < self.lines.len() {
                let line = &self.lines[line_index];
                let prefix = format!("{}: ", line_index);
                std::io::stdout().execute(Print(prefix))?;
                
                let max_chars = 70;
                let truncated = if line.len() > max_chars {
                    &line[..max_chars.min(line.len())]
                } else {
                    line.as_str()
                };
                std::io::stdout().execute(Print(truncated))?;
            } else {
                std::io::stdout().execute(Print("   "))?;
            }
        }
        
        let footer_start = 3 + self.get_visible_lines() as u16;
        std::io::stdout().execute(MoveTo(0, footer_start))?;
        std::io::stdout().execute(Print("Arrows: move | Type: insert | ESC: exit | Ctrl+S: save | BS: delete"))?;
        
        std::io::stdout().execute(MoveTo(0, footer_start + 1))?;
        std::io::stdout().execute(Print(format_args!("Cursor:({}, {})     ", self.cursor_x, self.cursor_y)))?;

        let screen_row = self.get_screen_row(self.cursor_y);
        let col_offset = self.get_column_offset(self.cursor_y);
        let edit_col = col_offset + self.cursor_x as u16;
        
        let screen_height = footer_start.saturating_sub(1);
        let clamped_row = screen_row.min(screen_height);
        std::io::stdout().execute(MoveTo(edit_col.min(79), clamped_row))?;

        Ok(())
    }

    pub fn move_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.ensure_cursor_visible();
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor_y < self.lines.len().saturating_sub(1) {
            self.cursor_y += 1;
            self.ensure_cursor_visible();
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if let Some(line) = self.lines.get(self.cursor_y) {
            if self.cursor_x < line.chars().count() {
                self.cursor_x += 1;
            }
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some(line) = self.lines.get_mut(self.cursor_y) {
            let char_pos = self.cursor_x;
            
            if char_pos >= line.chars().count() {
                line.push(c);
            } else {
                let mut new_line = String::new();
                for (i, ch) in line.chars().enumerate() {
                    if i == char_pos {
                        new_line.push(c);
                    }
                    new_line.push(ch);
                }
                *line = new_line;
            }
            
            self.cursor_x += 1;
        }
    }

    pub fn insert_newline(&mut self) {
        if let Some(current_line) = self.lines.get(self.cursor_y).cloned() {
            let chars: Vec<char> = current_line.chars().collect();
            let split_index = self.cursor_x.min(chars.len());

            let new_line_before: String = chars[..split_index].iter().collect();
            let new_line_after: String = chars[split_index..].iter().collect();

            self.lines[self.cursor_y] = new_line_before;
            self.lines.insert(self.cursor_y + 1, new_line_after);

            self.cursor_x = 0;
            self.cursor_y += 1;
            self.ensure_cursor_visible();
        }
    }

    // Delete character before the cursor
    pub fn delete_backward(&mut self) {
        if self.cursor_y == 0 && self.cursor_x == 0 {
            return;
        }

        if self.cursor_x == 0 {
            if self.cursor_y == 0 {
                return;
            }

            let current_line = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;

            if let Some(prev_line) = self.lines.get_mut(self.cursor_y) {
                prev_line.push_str(&current_line);
                self.cursor_x = prev_line.chars().count();
            }
            return;
        }

        if let Some(line) = self.lines.get_mut(self.cursor_y) {
            let chars: Vec<char> = line.chars().collect();
            let mut new_line = String::new();

            for (i, ch) in chars.into_iter().enumerate() {
                if i != self.cursor_x - 1 {
                    new_line.push(ch);
                }
            }

            *line = new_line;
            self.cursor_x -= 1;
        }
    }

    pub fn handle_input(&self) -> std::io::Result<Option<(KeyCode, bool)>> {
        if poll(std::time::Duration::from_millis(100))? {
            match read()? {
                Event::Key(key_event) => Ok(Some((
                    key_event.code,
                    key_event.modifiers.contains(KeyModifiers::CONTROL),
                ))),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        std::io::stdout().execute(EnterAlternateScreen)?;
        std::io::stdout().execute(Clear(ClearType::All))?;

        self.render()?;

        loop {
            if let Some((key, ctrl_pressed)) = self.handle_input()? {
                match key {
                    KeyCode::Esc => break,
                    KeyCode::Up => self.move_up(),
                    KeyCode::Down => self.move_down(),
                    KeyCode::Left => self.move_left(),
                    KeyCode::Right => self.move_right(),
                    KeyCode::Enter => self.insert_newline(),
                    KeyCode::Char('s') if ctrl_pressed => {
                        self.save_file()?;
                    }
                    KeyCode::Char(c) => self.insert_char(c),
                    KeyCode::Backspace => self.delete_backward(),  // ← ADDED BACKSPACE!
                    _ => {}
                }
                self.render()?;
            }
        }

        std::io::stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}

pub fn run_editor() -> io::Result<()> {
    println!("==================================");
    println!("          Nanovim Editor          ");
    println!("==================================");
    println!("Enter the file path to open or create:");
    
    let mut filepath = String::new();
    io::stdin().read_line(&mut filepath)?;
    
    let mut editor = Editor::new(filepath.trim().to_string());
    editor.run()?;
    
    // SAVE BEFORE EXITING (auto-save on exit)
    let _ = editor.save_file();

    Ok(())
}
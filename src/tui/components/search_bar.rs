use ratatui::{
    style::{Style, Stylize},
    text,
    widgets::{Block, Paragraph},
};

#[derive(Default, Debug)]
pub struct State {
    pub input: String,
    pub char_idx: usize,
}

impl State {
    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char_forward(&mut self) {
        if self.char_idx != self.input.len() {
            let current_index = self.char_idx;
            let from_right_to_current_index = current_index + 1;

            let before_char_to_delete = self.input.chars().take(current_index);
            let after_char_to_delete = self.input.chars().skip(from_right_to_current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
        }
    }

    pub fn delete_char_back(&mut self) {
        if self.char_idx != 0 {
            let current_index = self.char_idx;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.char_idx)
            .unwrap_or(self.input.len())
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.char_idx.saturating_sub(1);
        self.char_idx = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.char_idx.saturating_add(1);
        self.char_idx = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
}

pub fn view<'a>(input: &'a str, focused: bool) -> Paragraph<'a> {
    let mut block = Block::bordered().title(text::Line::from(vec![
        " Search ".into(),
        "</> ".blue().bold(),
    ]));
    if focused {
        block = block.border_style(Style::new().yellow());
    }
    Paragraph::new(input).block(block)
}

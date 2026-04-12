use ratatui::{
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};

pub fn view<'a>(title: String, items: &'a [String]) -> Paragraph<'a> {
    let block = Block::bordered().title(title);
    let text: Vec<Line> = items.iter().map(|l| Line::from(l.clone())).collect();
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    paragraph
}

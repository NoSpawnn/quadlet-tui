use ratatui::{
    style::Stylize as _,
    text,
    widgets::{Block, Paragraph},
};

use crate::quadlet::types::QuadletBasicInfo;

pub fn view<'a>(target: &'a QuadletBasicInfo) -> Paragraph<'a> {
    let text = vec![
        text::Line::from(format!("Name: {}", target.name)),
        text::Line::from(format!("File: {}", target.unit_file_path.display())),
        text::Line::from(format!("State: {:?}", target.status)),
        text::Line::from(format!("App: {}", target.app)),
    ];

    let mut instructions = vec![
        "<s>".blue().bold(),
        " Edit ".into(),
        "<e>".blue().bold(),
        " Details ".into(),
        "<space>".blue().bold(),
        " Restart ".into(),
        "<R> ".blue().bold(),
    ];
    if target.running() {
        instructions.insert(0, " Stop ".into());
    } else {
        instructions.insert(0, " Start ".into());
    }
    let instructions = text::Line::from(instructions);

    Paragraph::new(text).block(
        Block::bordered()
            .title(text::Line::from(" Preview "))
            .title_bottom(instructions.centered()),
    )
}

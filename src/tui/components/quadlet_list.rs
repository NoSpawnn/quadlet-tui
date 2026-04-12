use std::collections::BTreeMap;

use ratatui::{
    style::{Style, Styled, Stylize as _},
    text,
    widgets::{Block, Cell, List, ListState, Padding, Paragraph, Row, Table, TableState},
};

use crate::quadlet::types::{ActiveState, QuadletBasicInfo};

pub fn view<'a>(items: &'a [QuadletBasicInfo], focused: bool) -> List<'a> {
    let rows: Vec<_> = items
        .iter()
        .map(|q| text::Line::from(format!("{}", q.name.clone())))
        .collect();

    let mut block = Block::bordered()
        .padding(Padding::symmetric(2, 0))
        .title(text::Line::from(vec![
            format!(" Quadlets ({} total) ", items.len()).into(),
            "<tab> ".blue().bold(),
        ]));
    if focused {
        block = block.border_style(Style::new().yellow());
    }

    let list = List::new(rows)
        .highlight_style(Style::new().on_dark_gray().bold())
        .block(block);

    list
}

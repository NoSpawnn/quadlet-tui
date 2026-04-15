use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Position, Rect},
    style::{Color, Stylize},
    text,
    widgets::{Block, ListState, Padding, Paragraph, StatefulWidget, Widget},
};

use crate::{
    quadlet::{self, types::QuadletDetailedInfo},
    tui::{
        app::{App, FocusableWidget, Screen},
        components::{quadlet_details_preview, quadlet_list, search_bar},
    },
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.screen {
            Screen::Home => self.home_view(area, buf),
            Screen::Details => self.details_view(area, buf),
        }
    }
}

impl App {
    fn home_view(&self, area: Rect, buf: &mut Buffer) {
        let border = Block::new()
            .title("quadlet-tui")
            .title_alignment(Alignment::Center);
        Widget::render(border, area, buf);

        let [search_area, body_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

        let search_focused = self.focused == FocusableWidget::SearchBar;
        let search_bar = search_bar::view(&self.search_state.input, search_focused);
        if search_focused {
            let x = search_area.x + self.search_state.char_idx as u16 + 1;
            let y = search_area.y + 1;
            let cell = buf.cell_mut(Position::new(x, y)).unwrap();
            cell.bg = Color::Gray;
        }
        Widget::render(search_bar, search_area, buf);

        let [list_area, info_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(body_area);

        let list = quadlet_list::view(&self.quadlets, self.focused == FocusableWidget::QuadletList);
        let mut list_state = ListState::default().with_selected(Some(self.selected_quadlet));
        StatefulWidget::render(list, list_area, buf, &mut list_state);

        let [preview_area, logs_area] =
            Layout::vertical([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(info_area);
        let details = quadlet_details_preview::view(&self.quadlets[self.selected_quadlet]);
        Widget::render(details, preview_area, buf);

        let quadlet = &self.quadlets[self.selected_quadlet];
        let (logs, (scroll_x, scroll_y)) = self.get_quadlet_logs(quadlet);
        let lines: Vec<_> = logs.iter().map(|s| text::Line::from(s.as_str())).collect();
        let instructions = text::Line::from(vec![
            " Scroll ".into(),
            "<Ctrl + ↑/↓>".blue().bold(),
            " Scroll to end ".into(),
            "<Ctrl + Shift + ↑/↓> ".blue().bold(),
        ]);
        let logs = Paragraph::new(lines)
            .block(
                Block::bordered()
                    .title(" Logs ")
                    .title_bottom(instructions.centered())
                    .padding(Padding::symmetric(1, 0)),
            )
            .scroll((*scroll_y, *scroll_x));
        Widget::render(logs, logs_area, buf);
    }

    fn details_view(&self, area: Rect, buf: &mut Buffer) {
        let quadlet = &self.quadlets[self.selected_quadlet];
        let title =
            text::Line::from(format!("{} ({}) - details", quadlet.name, quadlet.kind).bold());
        let instructions = text::Line::from(vec![
            " Back ".into(),
            "<Space>".blue().bold(),
            " Restart ".into(),
            "<r>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);
        let outer_border = Block::new()
            .title(title.centered())
            .title_bottom(instructions.centered());
        let inner_area = outer_border.inner(area);
        outer_border.render(area, buf);

        let [details_area, logs_area] =
            Layout::vertical([Constraint::Percentage(60), Constraint::Percentage(40)])
                .margin(1)
                .areas(inner_area);

        let (logs, (scroll_x, scroll_y)) = self.get_quadlet_logs(quadlet);
        let lines: Vec<_> = logs.iter().map(|s| text::Line::from(s.as_str())).collect();
        let logs = Paragraph::new(lines)
            .block(
                Block::bordered()
                    .title("Logs")
                    .padding(Padding::symmetric(1, 0)),
            )
            .scroll((*scroll_y, *scroll_x));

        match quadlet::manager::inspect(&quadlet).unwrap() {
            QuadletDetailedInfo::Pod(pod_info) => detail_views::pod(details_area, buf, &pod_info),
            QuadletDetailedInfo::Network(network_info) => {
                detail_views::network(details_area, buf, &network_info)
            }
            QuadletDetailedInfo::Container(container_info) => {
                detail_views::container(details_area, buf, &container_info)
            }
        };

        Widget::render(logs, logs_area, buf);
    }
}

mod detail_views {
    use ratatui::{
        buffer::Buffer,
        layout::{Constraint, Layout, Rect},
        style::Style,
        text,
        widgets::{Block, Paragraph, Row, Table, Widget},
    };

    use crate::quadlet::types::{ContainerInfo, NetworkInfo, PodInfo};

    pub(super) fn pod<'a>(area: Rect, buf: &mut Buffer, pod: &'a PodInfo) {
        let sections = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(area);

        let network_info = {
            let port_lines = pod.infra_config["PortBindings"].to_string();
            Paragraph::new(port_lines).block(Block::bordered().title("Network info"))
        };

        let containers_table = {
            let rows: Vec<Row> = pod
                .containers
                .iter()
                .map(|c| Row::new(vec![c.name.clone(), c.state.clone()]))
                .collect();

            Table::new(
                rows,
                [Constraint::Percentage(30), Constraint::Percentage(70)],
            )
            .header(Row::new(vec!["Name", "Status"]).style(Style::new().bold()))
            .block(Block::bordered().title("Containers"))
        };

        Widget::render(network_info, sections[0], buf);
        Widget::render(containers_table, sections[1], buf);
    }

    pub(super) fn network<'a>(area: Rect, buf: &mut Buffer, network: &'a NetworkInfo) {
        let sections = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(area);

        let containers_table = {
            let rows: Vec<_> = network
                .containers
                .iter()
                .map(|(k, v)| Row::new(vec![k.clone(), v.to_string().clone()]))
                .collect();

            Table::new(
                rows,
                [Constraint::Percentage(30), Constraint::Percentage(70)],
            )
            .header(Row::new(vec!["Name", "Status"]).style(Style::new().bold()))
            .block(Block::bordered().title("Containers"))
        };

        Widget::render(containers_table, sections[0], buf);
    }

    pub(super) fn container<'a>(area: Rect, buf: &mut Buffer, container: &'a ContainerInfo) {
        let sections = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(area);

        let info = Paragraph::new(text::Line::from(container.name.as_str()));

        Widget::render(info, sections[0], buf);
    }
}

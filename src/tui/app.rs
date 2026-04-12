use std::collections::HashMap;

use crate::{
    quadlet::{self, types::QuadletBasicInfo},
    tui::{
        components::search_bar,
        event::{AppEvent, Event, EventHandler, SearchEvent},
    },
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

/// Application.
pub struct App {
    pub running: bool,
    pub focused: FocusableWidget,
    pub screen: Screen,

    pub events: EventHandler,

    pub quadlets: Vec<QuadletBasicInfo>,
    // unit name -> (logs, scroll)
    pub quadlet_logs: HashMap<String, (Vec<String>, (u16, u16))>,
    pub selected_quadlet: usize,

    pub search_state: search_bar::State,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum FocusableWidget {
    #[default]
    QuadletList,
    SearchBar,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Home,
    Details,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            quadlets: quadlet::manager::list().unwrap(),
            quadlet_logs: Default::default(),
            selected_quadlet: 0,
            search_state: Default::default(),
            focused: Default::default(),
            screen: Default::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.select_quadlet(self.selected_quadlet); // start a log stream for the first selected item

        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            match self.events.next().await? {
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),

                    AppEvent::StartLogStream(target) => {
                        tokio::spawn(quadlet::manager::stream_logs(
                            target.clone(),
                            self.events.cloned_sender(),
                        ));
                    }
                    AppEvent::AppendLog(unit_name, line) => {
                        self.quadlet_logs
                            .entry(unit_name)
                            .and_modify(|(logs, _)| logs.push(line));
                    }

                    AppEvent::SelectNext => self.select_quadlet(self.selected_quadlet + 1),
                    AppEvent::SelectPrev => self.select_quadlet(self.selected_quadlet - 1),

                    AppEvent::Navigate(target) => self.navigate(target),
                    AppEvent::Focus(target) => self.focused = target,

                    AppEvent::RestartQuadlet(target) => {
                        tokio::spawn(quadlet::manager::restart(
                            target.clone(),
                            self.events.cloned_sender(),
                        ));
                    }
                    AppEvent::RefreshQuadletList => {
                        // TODO: drop any logs from self.quadlet_logs if needed
                        self.quadlets = quadlet::manager::list().unwrap();
                    }

                    // search bar input
                    AppEvent::Search(e) => match e {
                        SearchEvent::Input(c) => self.search_state.enter_char(c),
                        SearchEvent::MoveLeft => self.search_state.move_cursor_left(),
                        SearchEvent::MoveRight => self.search_state.move_cursor_right(),
                        SearchEvent::DeleteBackward => self.search_state.delete_char_back(),
                        SearchEvent::DeleteForward => self.search_state.delete_char_forward(),
                    },

                    // logs scrolling
                    AppEvent::ScrollLogsToTop => {
                        self.quadlet_logs
                            .entry(self.quadlets[self.selected_quadlet].unit_name.clone())
                            .and_modify(|(_, (_, scroll_y))| *scroll_y = 0);
                    }
                    AppEvent::ScrollLogsUp => {
                        self.quadlet_logs
                            .entry(self.quadlets[self.selected_quadlet].unit_name.clone())
                            .and_modify(|(_, (_, scroll_y))| {
                                *scroll_y = scroll_y.saturating_sub(1)
                            });
                    }
                    AppEvent::ScrollLogsDown => {
                        self.quadlet_logs
                            .entry(self.quadlets[self.selected_quadlet].unit_name.clone())
                            .and_modify(|(logs, (_, scroll_y))| {
                                *scroll_y = scroll_y.saturating_add(1).min((logs.len() - 1) as u16)
                            });
                    }
                    AppEvent::ScrollLogsToBottom => {
                        self.quadlet_logs
                            .entry(self.quadlets[self.selected_quadlet].unit_name.clone())
                            .and_modify(|(logs, (_, scroll_y))| {
                                *scroll_y = (logs.len() - 1) as u16
                            });
                    }
                    AppEvent::ScrollLogsLeft => {
                        let target = self.quadlets[self.selected_quadlet].unit_name.clone();
                        self.quadlet_logs
                            .entry(target)
                            .and_modify(|(_, (scroll_x, _))| {
                                *scroll_x = scroll_x.saturating_sub(1)
                            });
                    }
                    AppEvent::ScrollLogsRight => {
                        let target = self.quadlets[self.selected_quadlet].unit_name.clone();
                        self.quadlet_logs
                            .entry(target)
                            .and_modify(|(_, (scroll_x, _))| {
                                *scroll_x = scroll_x.saturating_add(1);
                            });
                    }
                },
            }
        }

        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        // global keybinds
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') if self.focused != FocusableWidget::SearchBar => {
                self.events.send(AppEvent::Quit);
                return Ok(());
            }
            KeyCode::Char('c' | 'C') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.events.send(AppEvent::Quit);
                return Ok(());
            }
            _ => {}
        }

        if self.screen == Screen::Home && self.focused == FocusableWidget::SearchBar {
            match key_event.code {
                KeyCode::Right => self.events.send(AppEvent::Search(SearchEvent::MoveRight)),
                KeyCode::Left => self.events.send(AppEvent::Search(SearchEvent::MoveLeft)),
                KeyCode::Backspace => self
                    .events
                    .send(AppEvent::Search(SearchEvent::DeleteBackward)),
                KeyCode::Delete => self
                    .events
                    .send(AppEvent::Search(SearchEvent::DeleteForward)),
                KeyCode::Char(c) => self.events.send(AppEvent::Search(SearchEvent::Input(c))),
                KeyCode::Esc | KeyCode::Tab => self
                    .events
                    .send(AppEvent::Focus(FocusableWidget::QuadletList)),
                _ => {}
            }
        } else {
            match self.screen {
                Screen::Home => match key_event.code {
                    KeyCode::Char('/') => self
                        .events
                        .send(AppEvent::Focus(FocusableWidget::SearchBar)),
                    KeyCode::Char('r') => {
                        let target = &self.quadlets[self.selected_quadlet];
                        self.events
                            .send(AppEvent::RestartQuadlet(target.unit_name.clone()));
                    }
                    KeyCode::Char('j') | KeyCode::Down => match key_event.modifiers {
                        m if m == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                            self.events.send(AppEvent::ScrollLogsToBottom)
                        }
                        m if m == (KeyModifiers::CONTROL) => {
                            self.events.send(AppEvent::ScrollLogsDown)
                        }
                        _ => {
                            if self.selected_quadlet < self.quadlets.len() - 1 {
                                self.events.send(AppEvent::SelectNext)
                            }
                        }
                    },
                    KeyCode::Char('k') | KeyCode::Up => match key_event.modifiers {
                        m if m == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                            self.events.send(AppEvent::ScrollLogsToTop)
                        }
                        m if m == (KeyModifiers::CONTROL) => {
                            self.events.send(AppEvent::ScrollLogsUp)
                        }
                        _ => {
                            if self.selected_quadlet > 0 {
                                self.events.send(AppEvent::SelectPrev)
                            }
                        }
                    },
                    KeyCode::Char('h') | KeyCode::Left => {
                        if key_event.modifiers == (KeyModifiers::CONTROL) {
                            self.events.send(AppEvent::ScrollLogsLeft)
                        }
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        if key_event.modifiers == (KeyModifiers::CONTROL) {
                            self.events.send(AppEvent::ScrollLogsRight)
                        }
                    }
                    KeyCode::Char(' ') => self.events.send(AppEvent::Navigate(Screen::Details)),
                    _ => {}
                },

                Screen::Details => match key_event.code {
                    KeyCode::Char(' ') => self.events.send(AppEvent::Navigate(Screen::Home)),
                    KeyCode::Char('k') | KeyCode::Up => self.events.send(AppEvent::ScrollLogsUp),
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.events.send(AppEvent::ScrollLogsDown)
                    }
                    _ => {}
                },
            }
        }

        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    fn navigate(&mut self, target: Screen) {
        self.screen = target;
        // match target {
        //     Screen::Home => self.screen = target,
        //     Screen::Details => self.screen = target,
        // }
    }

    fn select_quadlet(&mut self, i: usize) {
        let target = &self.quadlets[i];

        let _ = self
            .quadlet_logs
            .entry(target.unit_name.clone())
            .or_insert_with(|| {
                self.events
                    .send(AppEvent::StartLogStream(target.unit_name.clone()));
                (Vec::new(), (0, 0))
            });

        self.selected_quadlet = i;
    }

    pub fn get_quadlet_logs(&self, target: &QuadletBasicInfo) -> &(Vec<String>, (u16, u16)) {
        &self.quadlet_logs[&target.unit_name]
    }
}

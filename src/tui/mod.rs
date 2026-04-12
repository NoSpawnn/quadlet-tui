use crate::tui::app::App;

pub mod app;
mod components;
pub mod event;
pub mod ui;

pub async fn run() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}

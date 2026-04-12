mod command_helpers;
mod quadlet;
mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tui::run().await?;
    Ok(())
}

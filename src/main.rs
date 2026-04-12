mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tui::run().await?;
    Ok(())
}

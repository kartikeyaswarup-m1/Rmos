mod app;
mod system;
mod ui;
//mod widgets;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(app::run().await?)
}

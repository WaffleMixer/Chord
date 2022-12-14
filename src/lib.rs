pub mod bot;
pub mod commands;
pub mod parse;

use bot::start_bot;
use commands::*;

pub async fn initialize() {
    start_bot().await;
}

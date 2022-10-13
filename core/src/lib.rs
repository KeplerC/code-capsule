#[macro_use]
extern crate log;

pub mod commands;
pub mod error;
pub mod worker;

use utils::error::Result;

pub fn start() -> Result<()> {
    // does nothing

    Ok(())
}

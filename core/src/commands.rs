use super::error;
use super::worker;

use utils::app_config::AppConfig;
use utils::error::Result;

/// Show the configuration file
pub fn worker() -> Result<()> {
    // Generate, randomly, True or False
    worker::run_worker()?;

    Ok(())
}

/// Show the configuration file
pub fn config() -> Result<()> {
    let config = AppConfig::fetch()?;
    println!("{:#?}", config);

    Ok(())
}

/// Simulate an error
pub fn simulate_error() -> Result<()> {
    // Log this Error simulation
    info!("We are simulating an error");

    // Simulate an error
    error::simulate_error()?;

    // We should never get here...
    Ok(())
}

use log::{LevelFilter};
use simple_logger::SimpleLogger;

pub fn init_logger() -> anyhow::Result<()> {
    if log::max_level() == LevelFilter::Off {
        SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init()
            .map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))?;
    }
    Ok(())
}

use anyhow::Result;
use log::info;

#[cfg(not(feature = "simulator"))]
use log::error;

mod app;
mod input;
mod stroke;
mod recognition;
mod context;
mod llm;
mod render;

#[cfg(feature = "simulator")]
mod simulator;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Load environment variables
    dotenv::dotenv().ok();

    info!("Jedusor starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Check for API key (optional - only needed for AI responses, not handwriting recognition)
    if std::env::var("ANTHROPIC_API_KEY").is_err() {
        #[cfg(not(feature = "simulator"))]
        {
            use log::warn;
            warn!("ANTHROPIC_API_KEY environment variable not set");
            warn!("Handwriting recognition will work, but AI responses will be disabled");
        }
        #[cfg(feature = "simulator")]
        {
            info!("ANTHROPIC_API_KEY not set (optional for testing)");
        }
    }

    // Create and run the application
    let mut app = app::App::new()?;
    app.run()?;

    info!("Jedusor shutting down...");
    Ok(())
}

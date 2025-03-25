use anyhow::Result;
use clap::Parser;
use n9_core::Substance;
use ui9_mesh::Mesh;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Enable TUI mode
    #[arg(long)]
    tui: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::try_init()?;
    Mesh::activate().await?;
    let mut substance = Substance::arise();
    substance.add_particle::<n9_config_toml::TomlConfigParticle>()?;
    // TODO: Rename to *Model
    substance.add_particle::<n9_model_openai::OpenAIParticle>()?;
    // substance.add_particle::<n9_model_anthropic::AnthropicParticle>()?;

    // TODO: Rename to *Exchange
    substance.add_particle::<n9_exchange_dydx::DyDxParticle>()?;
    // substance.add_particle::<n9_exchange::ExchangeParticle>()?;

    // TODO: Rename to *Control
    // substance.add_particle::<n9_control_chat::ChatParticle>()?;
    substance.add_particle::<n9_control_task::ControlTask>()?;
    substance.add_particle::<n9_control_session::SessionParticle>()?;

    // substance.add_particle::<n9_app_stdio::StdioApp>()?;

    if args.tui {
        substance.add_particle::<n9_app_tui::TuiApp>()?;
    }

    // TODO: Rename to *Chat
    // substance.add_particle::<n9_chat_telegram::TelegramParticle>()?;

    // Stdio is not compatible with tracing and will be replaced with DUI
    // substance.add_particle::<StdioParticle>()?;
    substance.join().await?;
    Mesh::deactivate().await?;
    // Unblocking stdin
    std::process::exit(0);
}

use anyhow::Result;
use n9_core::Substance;
use ui9_mesh::Mesh;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    Mesh::activate().await?;
    let mut substance = Substance::arise();
    // TODO: Rename to *Model
    substance.add_particle::<n9_model_openai::OpenAIParticle>()?;
    // substance.add_particle::<n9_model_anthropic::AnthropicParticle>()?;

    // TODO: Rename to *Exchange
    substance.add_particle::<n9_exchange_dydx::DyDxParticle>()?;
    // substance.add_particle::<n9_exchange::ExchangeParticle>()?;

    // TODO: Rename to *Control
    substance.add_particle::<n9_control_chat::ChatParticle>()?;
    substance.add_particle::<n9_control_task::ControlTask>()?;

    // substance.add_particle::<n9_app_stdio::StdioApp>()?;

    substance.add_particle::<n9_app_tui::TuiApp>()?;

    // TODO: Rename to *Chat
    substance.add_particle::<n9_chat_telegram::TelegramParticle>()?;

    // Stdio is not compatible with tracing and will be replaced with DUI
    // substance.add_particle::<StdioParticle>()?;
    substance.join().await?;
    Mesh::deactivate().await?;
    // Unblocking stdin
    std::process::exit(0);
}

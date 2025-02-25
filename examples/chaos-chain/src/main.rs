use anyhow::Result;
use n9_app_tui::TuiApp;
use n9_control_chat::ChatParticle;
use n9_core::Substance;
use n9_model_openai::OpenAIParticle;
use n9_system_info::SystemInfo;
use ui9_mesh::Mesh;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    Mesh::activate().await?;
    let mut substance = Substance::arise();
    substance.add_particle::<OpenAIParticle>()?;
    substance.add_particle::<ChatParticle>()?;
    substance.add_particle::<TuiApp>()?;
    substance.add_particle::<SystemInfo>()?;
    substance.join().await?;
    Mesh::deactivate().await?;
    Ok(())
}

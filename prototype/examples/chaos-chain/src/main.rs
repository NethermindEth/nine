use anyhow::Result;
use n9_core::Substance;
use ui9_mesh::Mesh;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    Mesh::activate().await?;
    let mut substance = Substance::arise();
    substance.add_particle::<n9_config_toml::TomlConfigParticle>()?;
    substance.add_particle::<n9_model_openai::OpenAIParticle>()?;
    substance.add_particle::<n9_control_chat::ChatParticle>()?;
    substance.add_particle::<n9_app_tui::TuiApp>()?;
    substance.add_particle::<n9_system_info::SystemInfo>()?;
    substance.join().await?;
    Mesh::deactivate().await?;
    Ok(())
}

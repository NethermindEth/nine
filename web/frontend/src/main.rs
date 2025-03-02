use crb::agent::Standalone;
use n9_core::Substance;
use n9_web_frontend::Frontend;

fn main() {
    // TODO: Is that possible to use `tracing` package here?
    let config = wasm_logger::Config::new(log::Level::Trace);
    wasm_logger::init(config);
    let mut substance = Substance::arise();
    log::info!("N9 WEB APP LOADED");
    Frontend::new().spawn();
}

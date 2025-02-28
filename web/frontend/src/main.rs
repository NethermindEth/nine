use crb::agent::Standalone;
use n9_web_frontend::Frontend;

fn main() {
    // TODO: Is that possible to use `tracing` package here?
    let config = wasm_logger::Config::new(log::Level::Warn);
    wasm_logger::init(config);
    log::info!("N9 WEB APP LOADED");
    Frontend::new().spawn();
}

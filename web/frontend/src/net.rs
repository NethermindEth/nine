use std::str::FromStr;
use anyhow::{anyhow, Result};
use libp2p::Multiaddr;
use web_sys::window;

pub fn server_multiaddr() -> Result<Multiaddr> {
    let window = window().ok_or_else(|| anyhow!("No `window` object available"))?;
    let location = window.location();

    let protocol = location.protocol().map_err(|_| anyhow!("No `protocol` part in the location"))?;
    let host = location.hostname().map_err(|_| anyhow!("No `hostname` in the location"))?;
    let port_str = location.port().map_err(|_| anyhow!("No `port` in the location"))?;

    let mut service = "dns4";
    let port = {
        if host == "localhost" {
            "8080"
        } else if host == "127.0.0.1" {
            service = "ip4";
            "8080"
        } else {
            if port_str.is_empty() {
                if protocol == "https:" {
                    "443"
                } else {
                    "80"
                }
            } else {
                &port_str
            }
        }
    };

    let ws_scheme = if protocol == "https:" { "wss" } else { "ws" };

    let addr = format!("/dns4/{host}/tcp/{port}/{ws_scheme}").parse()?;
    Ok(addr)
}

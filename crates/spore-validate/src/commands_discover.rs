// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    cas_push, discovery,
    error::Error,
    nucleus, nucleus_display,
    paths::{ENV_BIOMEOS_SOCKET_DIR, ENV_TRANSPORT_ENDPOINT, ENV_XDG_RUNTIME},
};
use std::path::Path;

#[allow(clippy::unnecessary_wraps)] // uniform handler signature for main.rs dispatch
pub fn discover() -> Result<(), Error> {
    println!("spore-validate: capability discovery");
    println!();

    let self_caps = &discovery::SELF;
    println!("  SELF: {} v{}", self_caps.primal_id, self_caps.version);
    println!("  capabilities:");
    for cap in self_caps.capabilities {
        println!(
            "    [{:>9}] {} — {}",
            cap.category, cap.name, cap.description
        );
    }

    println!();
    println!("  TRANSPORT:");
    if let Ok(json) = std::env::var(ENV_TRANSPORT_ENDPOINT) {
        match serde_json::from_str::<cas_push::TransportEndpoint>(&json) {
            Ok(ep) => println!("    injected: {ep:?}"),
            Err(e) => println!("    {ENV_TRANSPORT_ENDPOINT} parse error: {e}"),
        }
    } else {
        println!("    (no {ENV_TRANSPORT_ENDPOINT} — will use socket discovery)");
    }

    println!();
    println!("  SOCKET_DIRS:");
    if let Ok(dir) = std::env::var(ENV_BIOMEOS_SOCKET_DIR) {
        println!("    {ENV_BIOMEOS_SOCKET_DIR} = {dir}");
    }
    if let Ok(dir) = std::env::var(ENV_XDG_RUNTIME) {
        println!("    {ENV_XDG_RUNTIME} = {dir}");
    }
    let systemd_dir = discovery::systemd_socket_dir();
    if std::path::Path::new(systemd_dir.as_ref()).is_dir() {
        println!("    {systemd_dir} (systemd NUCLEUS)");
    }
    if std::env::var(ENV_BIOMEOS_SOCKET_DIR).is_err()
        && std::env::var(ENV_XDG_RUNTIME).is_err()
        && !std::path::Path::new(systemd_dir.as_ref()).is_dir()
    {
        println!(
            "    (no socket directories found — set BIOMEOS_SOCKET_DIR or use systemd NUCLEUS)"
        );
    }

    println!();
    println!("  PEERS:");

    let peers = discovery::discover_peers();
    if peers.is_empty() {
        println!(
            "    (none discovered — set BIOMEOS_SOCKET_DIR or NESTGATE_SOCKET/PETALTONGUE_SOCKET)"
        );
    } else {
        for peer in &peers {
            println!(
                "    {} ({})",
                peer.primal_id,
                peer.socket_path.as_deref().unwrap_or("?")
            );
            for cap in &peer.capabilities {
                println!("      - {cap}");
            }
        }
    }

    crate::commands_depot::print_discovery();

    Ok(())
}

/// Run NUCLEUS profile validation.
pub fn nucleus(profile_path: &Path, probe: bool, ribocipher: bool) -> Result<(), Error> {
    let profile = nucleus::parse_profile(profile_path)?;
    let result = nucleus::validate_profile(&profile, probe, ribocipher);

    nucleus_display::print_result(&profile, &result, profile_path);

    if result.passed() {
        println!("  RESULT: ✅ NUCLEUS COMPLIANT");
        Ok(())
    } else {
        println!("  RESULT: ❌ NUCLEUS NON-COMPLIANT");
        Err(Error::Config("NUCLEUS validation failed".into()))
    }
}

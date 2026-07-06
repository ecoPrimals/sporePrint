// SPDX-License-Identifier: AGPL-3.0-or-later

//! NUCLEUS validation display — formatted reporting for profile validation results.
//!
//! Separated from the core validation logic in [`nucleus`] to keep the
//! validation engine and CLI presentation as distinct concerns.

use crate::nucleus::{HealthContract, NucleusProfile, PrimalStatus, ProbeResult, ValidationResult};
use std::path::Path;

/// Print NUCLEUS validation results to stdout.
///
/// Displays profile metadata, per-primal health status (with optional probe
/// details), and aggregate compliance summary.
pub fn print_result(profile: &NucleusProfile, result: &ValidationResult, profile_path: &Path) {
    print_header(profile, result, profile_path);
    print_primals(result);
    print_summary(result);
}

fn print_header(profile: &NucleusProfile, result: &ValidationResult, profile_path: &Path) {
    println!("sporePrint: NUCLEUS profile validation");
    println!(
        "  Profile: {} ({})",
        result.profile_name,
        profile_path.display()
    );
    if let Some(desc) = &profile.profile.description {
        println!("  Description: {desc}");
    }
    if let Some(base) = profile.profile.base() {
        println!("  Extends: {base}");
    }
    println!("  Declared primals: {}", result.total_declared);
    if !profile.launch_order().is_empty() {
        println!("  Launch order: {}", profile.launch_order().join(" → "));
    }
    if profile.federation_enabled() {
        println!("  Federation: enabled");
    }
    println!();
}

fn print_primals(result: &ValidationResult) {
    if !result.healthy.is_empty() {
        println!(
            "  HEALTHY ({}/{}):",
            result.healthy.len(),
            result.total_declared
        );
        for p in &result.healthy {
            let probe_info = format_probe_info(p.probe.as_ref());
            println!(
                "    ✅ {} [{}] → {}{}",
                p.name,
                p.role,
                p.socket_path.as_deref().unwrap_or("?"),
                probe_info
            );
        }
    }

    if !result.missing.is_empty() {
        println!();
        println!(
            "  MISSING ({}/{}):",
            result.missing.len(),
            result.total_declared
        );
        for p in &result.missing {
            let marker = if p.required { "❌" } else { "⚠️" };
            let probe_err = format_probe_error(p.probe.as_ref());
            println!(
                "    {marker} {} [{}] (required={}){probe_err}",
                p.name, p.role, p.required
            );
        }
    }

    if result.healthy.iter().any(|p| p.probe.is_some()) {
        let total_probed = result.healthy.len();
        let compliant = count_by_contract(&result.healthy, HealthContract::Compliant);
        let partial = count_by_contract(&result.healthy, HealthContract::Partial);

        println!();
        println!(
            "  Health contract (guideStone): {compliant}/{total_probed} compliant, {partial} partial"
        );

        let all_primals: Vec<_> = result.healthy.iter().chain(result.missing.iter()).collect();
        let ribo_total = all_primals.iter().filter(|p| has_ribo_result(p)).count();

        if ribo_total > 0 {
            let accepted = all_primals.iter().filter(|p| ribo_accepted(p)).count();
            println!("  riboCipher mito-beacon: {accepted}/{ribo_total} accept signal");
        }
    }

    println!();
}

fn print_summary(result: &ValidationResult) {
    println!(
        "  Critical path: {}",
        if result.critical_met {
            "✅ MET"
        } else {
            "❌ FAILED"
        }
    );
    println!(
        "  Min healthy: {}",
        if result.min_healthy_met {
            "✅ MET"
        } else {
            "❌ FAILED"
        }
    );
    println!();
}

fn count_by_contract(primals: &[PrimalStatus], target: HealthContract) -> usize {
    primals
        .iter()
        .filter(|p| {
            p.probe
                .as_ref()
                .is_some_and(|pr| pr.health_contract == target)
        })
        .count()
}

fn has_ribo_result(p: &PrimalStatus) -> bool {
    p.probe
        .as_ref()
        .is_some_and(|pr| pr.ribocipher_accepted.is_some())
}

fn ribo_accepted(p: &PrimalStatus) -> bool {
    p.probe
        .as_ref()
        .is_some_and(|pr| pr.ribocipher_accepted == Some(true))
}

fn format_probe_info(probe: Option<&ProbeResult>) -> String {
    probe.map_or_else(String::new, |pr| {
        let contract_icon = match pr.health_contract {
            HealthContract::Compliant => " [health:✅]",
            HealthContract::Partial => " [health:⚠️]",
            HealthContract::None => "",
        };
        let ribo_icon = match pr.ribocipher_accepted {
            Some(true) => " [mito:✅]",
            Some(false) => " [mito:❌]",
            None => "",
        };
        let version_str = pr
            .version
            .as_deref()
            .map_or(String::new(), |v| format!(", v{v}"));
        let identity_str = pr
            .primal_id
            .as_deref()
            .map_or(String::new(), |id| format!(", id={id}"));
        let status_str = pr
            .status
            .as_deref()
            .map_or(String::new(), |s| format!(", {s}"));
        format!(
            " ({}ms{version_str}{identity_str}{status_str}{contract_icon}{ribo_icon})",
            pr.latency.as_millis()
        )
    })
}

fn format_probe_error(probe: Option<&ProbeResult>) -> String {
    probe.map_or_else(String::new, |pr| {
        let mut info = String::new();
        if let Some(e) = &pr.error {
            info.push_str(" — ");
            info.push_str(e);
        }
        if pr.responsive && pr.health_contract == HealthContract::None {
            info.push_str(" [no health method]");
        }
        info
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn format_probe_info_none_returns_empty() {
        assert_eq!(format_probe_info(None), "");
    }

    #[test]
    fn format_probe_info_shows_latency_and_contract() {
        let probe = ProbeResult {
            responsive: true,
            latency: Duration::from_millis(42),
            version: Some("1.2.3".into()),
            primal_id: Some("beardog".into()),
            status: Some("running".into()),
            health_contract: HealthContract::Compliant,
            ribocipher_accepted: Some(true),
            error: None,
        };
        let s = format_probe_info(Some(&probe));
        assert!(s.contains("42ms"));
        assert!(s.contains("v1.2.3"));
        assert!(s.contains("id=beardog"));
        assert!(s.contains("running"));
        assert!(s.contains("[health:✅]"));
        assert!(s.contains("[mito:✅]"));
    }

    #[test]
    fn format_probe_info_partial_contract() {
        let probe = ProbeResult {
            responsive: true,
            latency: Duration::from_millis(5),
            version: None,
            primal_id: None,
            status: None,
            health_contract: HealthContract::Partial,
            ribocipher_accepted: Some(false),
            error: None,
        };
        let s = format_probe_info(Some(&probe));
        assert!(s.contains("[health:⚠️]"));
        assert!(s.contains("[mito:❌]"));
    }

    #[test]
    fn format_probe_error_none_returns_empty() {
        assert_eq!(format_probe_error(None), "");
    }

    #[test]
    fn format_probe_error_shows_message() {
        let probe = ProbeResult {
            responsive: false,
            latency: Duration::from_millis(0),
            version: None,
            primal_id: None,
            status: None,
            health_contract: HealthContract::None,
            ribocipher_accepted: None,
            error: Some("connection refused".into()),
        };
        let s = format_probe_error(Some(&probe));
        assert!(s.contains("connection refused"));
    }

    #[test]
    fn format_probe_error_no_health_method() {
        let probe = ProbeResult {
            responsive: true,
            latency: Duration::from_millis(1),
            version: None,
            primal_id: None,
            status: None,
            health_contract: HealthContract::None,
            ribocipher_accepted: None,
            error: None,
        };
        let s = format_probe_error(Some(&probe));
        assert!(s.contains("[no health method]"));
    }

    #[test]
    fn count_by_contract_filters_correctly() {
        let primals = vec![
            PrimalStatus {
                name: "a".into(),
                role: "core".into(),
                required: true,
                socket_path: None,
                probe: Some(ProbeResult {
                    responsive: true,
                    latency: Duration::from_millis(1),
                    version: None,
                    primal_id: None,
                    status: None,
                    health_contract: HealthContract::Compliant,
                    ribocipher_accepted: None,
                    error: None,
                }),
            },
            PrimalStatus {
                name: "b".into(),
                role: "aux".into(),
                required: false,
                socket_path: None,
                probe: Some(ProbeResult {
                    responsive: true,
                    latency: Duration::from_millis(2),
                    version: None,
                    primal_id: None,
                    status: None,
                    health_contract: HealthContract::Partial,
                    ribocipher_accepted: None,
                    error: None,
                }),
            },
            PrimalStatus {
                name: "c".into(),
                role: "aux".into(),
                required: false,
                socket_path: None,
                probe: None,
            },
        ];
        assert_eq!(count_by_contract(&primals, HealthContract::Compliant), 1);
        assert_eq!(count_by_contract(&primals, HealthContract::Partial), 1);
        assert_eq!(count_by_contract(&primals, HealthContract::None), 0);
    }

    #[test]
    fn has_ribo_result_detects_presence() {
        let with = PrimalStatus {
            name: "x".into(),
            role: "r".into(),
            required: true,
            socket_path: None,
            probe: Some(ProbeResult {
                responsive: true,
                latency: Duration::from_millis(0),
                version: None,
                primal_id: None,
                status: None,
                health_contract: HealthContract::None,
                ribocipher_accepted: Some(false),
                error: None,
            }),
        };
        let without = PrimalStatus {
            name: "y".into(),
            role: "r".into(),
            required: true,
            socket_path: None,
            probe: Some(ProbeResult {
                responsive: true,
                latency: Duration::from_millis(0),
                version: None,
                primal_id: None,
                status: None,
                health_contract: HealthContract::None,
                ribocipher_accepted: None,
                error: None,
            }),
        };
        assert!(has_ribo_result(&with));
        assert!(!has_ribo_result(&without));
    }

    #[test]
    fn validation_result_passed_logic() {
        let passing = ValidationResult {
            profile_name: "test".into(),
            total_declared: 2,
            healthy: vec![],
            missing: vec![],
            critical_met: true,
            min_healthy_met: true,
        };
        assert!(passing.passed());

        let failing = ValidationResult {
            profile_name: "test".into(),
            total_declared: 2,
            healthy: vec![],
            missing: vec![],
            critical_met: false,
            min_healthy_met: true,
        };
        assert!(!failing.passed());
    }
}

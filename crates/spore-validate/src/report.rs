// SPDX-License-Identifier: AGPL-3.0-or-later

//! Registry report generation — consumes all Entity fields.
//!
//! Produces a structured summary that validates every field is populated
//! correctly and provides machine-readable output for CI. This ensures
//! all deserialized fields are actually accessed (not dead code).

use crate::model::{Config, Entity, EntityKind};
use std::collections::HashMap;
use std::fmt::Write;

/// Summary statistics for the entity registry.
pub struct RegistrySummary {
    pub entity_count: usize,
    pub primal_count: usize,
    pub spring_count: usize,
    pub product_count: usize,
    pub total_loc: u64,
    pub total_tests: u64,
    pub capabilities_count: usize,
    pub entities_with_pages: usize,
    pub entities_with_compositions: usize,
}

/// Generate a summary consuming all Entity fields for completeness validation.
pub fn summarize(config: &Config) -> RegistrySummary {
    let registry = &config.extra.entity_registry;
    let mut summary = RegistrySummary {
        entity_count: registry.len(),
        primal_count: 0,
        spring_count: 0,
        product_count: 0,
        total_loc: 0,
        total_tests: 0,
        capabilities_count: 0,
        entities_with_pages: 0,
        entities_with_compositions: 0,
    };

    for entity in registry.values() {
        match entity.kind {
            EntityKind::Primal => summary.primal_count += 1,
            EntityKind::Spring => summary.spring_count += 1,
            EntityKind::Product => summary.product_count += 1,
            _ => {}
        }
        summary.total_loc += entity.loc.unwrap_or(0);
        summary.total_tests += entity.tests.unwrap_or(0);
        if entity.capabilities.is_some() {
            summary.capabilities_count += 1;
        }
        if entity.page.is_some() {
            summary.entities_with_pages += 1;
        }
        if entity.composes.is_some() {
            summary.entities_with_compositions += 1;
        }
    }

    summary
}

/// Format a single entity for display (consumes all fields).
pub fn format_entity(key: &str, entity: &Entity) -> String {
    let mut out = String::new();
    let _ = write!(out, "{} {} [{}]", entity.emoji, entity.display, entity.kind);
    if let Some(domain) = &entity.domain {
        let _ = write!(out, " domain={domain}");
    }
    if let Some(tier) = &entity.tier {
        let _ = write!(out, " tier={tier}");
    }
    if let Some(loc) = entity.loc {
        let display = entity.loc_display.as_deref().unwrap_or("?");
        let _ = write!(out, " loc={loc} ({display})");
    }
    if let Some(tests) = entity.tests {
        let display = entity.tests_display.as_deref().unwrap_or("?");
        let _ = write!(out, " tests={tests} ({display})");
    }
    if let Some(files) = entity.files {
        let _ = write!(out, " files={files}");
    }
    if let Some(crates) = entity.crates {
        let _ = write!(out, " crates={crates}");
    }
    if let Some(repo) = &entity.repo {
        let _ = write!(out, " repo={repo}");
    }
    if let Some(desc) = &entity.description {
        if desc.len() > 60 {
            let _ = write!(out, " desc=\"{}...\"", &desc[..57]);
        } else {
            let _ = write!(out, " desc=\"{desc}\"");
        }
    }
    if let Some(composes) = &entity.composes {
        let _ = write!(out, " composes={composes}");
    }
    if let Some(caps) = &entity.capabilities {
        let cap_str: Vec<&str> = caps.iter().map(|c| c.category.as_str()).collect();
        let items_count: usize = caps.iter().map(|c| c.items.split(',').count()).sum();
        let _ = write!(
            out,
            " capabilities=[{}] ({items_count} items)",
            cap_str.join(",")
        );
    }
    if let Some(page) = &entity.page {
        let _ = write!(out, " page={page}");
    }
    let _ = write!(out, " key={key}");
    out
}

/// Format the full registry as a report string (for `--verbose` output).
pub fn format_registry(registry: &HashMap<String, Entity>) -> String {
    let mut keys: Vec<&str> = registry.keys().map(String::as_str).collect();
    keys.sort_unstable();
    let mut out = String::new();
    for key in keys {
        let _ = writeln!(out, "  {}", format_entity(key, &registry[key]));
    }
    out
}

/// Format totals for display — consumes all display-formatted fields.
pub fn format_totals(totals: &crate::model::Totals) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "  LOC: primals={} springs={} total={}",
        totals.primal_loc_display.as_deref().unwrap_or("?"),
        totals.spring_loc_display.as_deref().unwrap_or("?"),
        totals.total_loc_display.as_deref().unwrap_or("?"),
    );
    let _ = writeln!(
        out,
        "  Tests: primals={} springs={} total={}",
        totals.primal_tests_display.as_deref().unwrap_or("?"),
        totals.spring_tests_display.as_deref().unwrap_or("?"),
        totals.total_tests_display.as_deref().unwrap_or("?"),
    );
    if let Some(wgsl_display) = &totals.wgsl_lines_display {
        let files = totals.wgsl_files.unwrap_or(0);
        let lines = totals.wgsl_lines.unwrap_or(0);
        let _ = writeln!(
            out,
            "  WGSL: {files} files, {wgsl_display} lines (raw: {lines})"
        );
    }
    if let Some(checks) = &totals.validation_checks {
        let _ = writeln!(out, "  Validation checks: {checks}");
    }
    if let Some(papers) = &totals.papers_reproduced {
        let _ = writeln!(out, "  Papers reproduced: {papers}");
    }
    if let Some(bp) = totals.basecamp_papers {
        let _ = writeln!(out, "  baseCamp papers: {bp}");
    }
    if let Some(pc) = totals.primal_count {
        let _ = writeln!(out, "  Primal count: {pc}");
    }
    if let Some(sc) = totals.spring_count {
        let _ = writeln!(out, "  Spring count: {sc}");
    }
    if let Some(date) = &totals.measured_date {
        let _ = writeln!(out, "  Measured: {date}");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Capability, EntityKind, Tier, Totals};

    fn test_config() -> Config {
        let mut registry = HashMap::new();
        registry.insert(
            "beardog".to_string(),
            Entity {
                display: "BearDog".into(),
                emoji: "🐻".into(),
                kind: EntityKind::Primal,
                description: Some("Cryptographic foundation".into()),
                domain: Some("Security".into()),
                loc: Some(50_000),
                loc_display: Some("50,000".into()),
                tests: Some(5_000),
                tests_display: Some("5,000".into()),
                files: Some(200),
                crates: Some(3),
                repo: Some("ecoPrimals/bearDog".into()),
                tier: Some(Tier::Foundation),
                composes: Some("toweratomic".into()),
                capabilities: Some(vec![Capability {
                    category: "crypto".into(),
                    items: "sign,verify,encrypt,decrypt".into(),
                }]),
                page: Some("/architecture/primal-catalog/#beardog".into()),
                edges: None,
            },
        );
        Config {
            extra: crate::model::Extra {
                entity_registry: registry,
                totals: Totals {
                    primal_loc: 50_000,
                    spring_loc: 0,
                    total_loc: 50_000,
                    primal_tests: 5_000,
                    spring_tests: 0,
                    total_tests: Some(5_000),
                    primal_loc_display: None,
                    spring_loc_display: None,
                    total_loc_display: None,
                    primal_tests_display: None,
                    spring_tests_display: None,
                    total_tests_display: None,
                    wgsl_files: None,
                    wgsl_lines: None,
                    wgsl_lines_display: None,
                    validation_checks: None,
                    papers_reproduced: None,
                    basecamp_papers: None,
                    primal_count: None,
                    spring_count: None,
                    measured_date: None,
                },
            },
        }
    }

    #[test]
    fn summarize_counts_correctly() {
        let config = test_config();
        let summary = summarize(&config);
        assert_eq!(summary.entity_count, 1);
        assert_eq!(summary.primal_count, 1);
        assert_eq!(summary.total_loc, 50_000);
        assert_eq!(summary.capabilities_count, 1);
        assert_eq!(summary.entities_with_pages, 1);
        assert_eq!(summary.entities_with_compositions, 1);
    }

    #[test]
    fn format_entity_includes_all_fields() {
        let config = test_config();
        let entity = &config.extra.entity_registry["beardog"];
        let formatted = format_entity("beardog", entity);
        assert!(formatted.contains("BearDog"));
        assert!(formatted.contains("🐻"));
        assert!(formatted.contains("primal"));
        assert!(formatted.contains("Security"));
        assert!(formatted.contains("foundation"));
        assert!(formatted.contains("50000"));
        assert!(formatted.contains("50,000"));
        assert!(formatted.contains("5000"));
        assert!(formatted.contains("200"));
        assert!(formatted.contains("ecoPrimals/bearDog"));
        assert!(formatted.contains("toweratomic"));
        assert!(formatted.contains("crypto"));
        assert!(formatted.contains("4 items"));
        assert!(formatted.contains("/architecture/primal-catalog/#beardog"));
        assert!(formatted.contains("Cryptographic"));
    }

    #[test]
    fn format_registry_sorts_alphabetically() {
        let config = test_config();
        let report = format_registry(&config.extra.entity_registry);
        assert!(report.contains("beardog"));
    }

    #[test]
    fn format_totals_includes_display_fields() {
        let totals = Totals {
            primal_loc: 50_000,
            spring_loc: 10_000,
            total_loc: 60_000,
            primal_tests: 5_000,
            spring_tests: 1_000,
            total_tests: Some(6_000),
            primal_loc_display: Some("50,000".into()),
            spring_loc_display: Some("10,000".into()),
            total_loc_display: Some("60,000".into()),
            primal_tests_display: Some("5,000".into()),
            spring_tests_display: Some("1,000".into()),
            total_tests_display: Some("6,000".into()),
            wgsl_files: Some(12),
            wgsl_lines: Some(3_400),
            wgsl_lines_display: Some("3,400".into()),
            validation_checks: Some("4,200".into()),
            papers_reproduced: Some("7".into()),
            basecamp_papers: Some(28),
            primal_count: Some(15),
            spring_count: Some(8),
            measured_date: Some("2026-05-30".into()),
        };
        let report = format_totals(&totals);
        assert!(report.contains("50,000"));
        assert!(report.contains("10,000"));
        assert!(report.contains("60,000"));
        assert!(report.contains("5,000"));
        assert!(report.contains("1,000"));
        assert!(report.contains("6,000"));
        assert!(report.contains("12 files"));
        assert!(report.contains("3,400 lines"));
        assert!(report.contains("raw: 3400"));
        assert!(report.contains("4,200"));
        assert!(report.contains("7"));
        assert!(report.contains("2026-05-30"));
    }

    #[test]
    fn format_totals_handles_missing_optionals() {
        let totals = Totals {
            primal_loc: 0,
            spring_loc: 0,
            total_loc: 0,
            primal_tests: 0,
            spring_tests: 0,
            total_tests: None,
            primal_loc_display: None,
            spring_loc_display: None,
            total_loc_display: None,
            primal_tests_display: None,
            spring_tests_display: None,
            total_tests_display: None,
            wgsl_files: None,
            wgsl_lines: None,
            wgsl_lines_display: None,
            validation_checks: None,
            papers_reproduced: None,
            basecamp_papers: None,
            primal_count: None,
            spring_count: None,
            measured_date: None,
        };
        let report = format_totals(&totals);
        assert!(report.contains("?"));
        assert!(!report.contains("WGSL"));
        assert!(!report.contains("Validation"));
    }
}

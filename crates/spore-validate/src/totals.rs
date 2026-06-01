// SPDX-License-Identifier: AGPL-3.0-or-later

//! Aggregate totals verification.
//!
//! Validates that `[extra.totals]` sums match the per-entity fields in the
//! registry. Catches metric drift introduced by partial updates.

use crate::error::Diagnostic;
use crate::model::{Entity, EntityKind, Totals};
use std::collections::HashMap;

/// Verify that stored totals match computed sums from the registry.
pub fn validate(
    registry: &HashMap<String, Entity>,
    totals: &Totals,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let primal_loc: u64 = sum_field(registry, EntityKind::Primal, |e| e.loc);
    let spring_loc: u64 = sum_field(registry, EntityKind::Spring, |e| e.loc);
    let total_loc = primal_loc + spring_loc;

    check(
        totals.primal_loc,
        primal_loc,
        "primal_loc",
        "primals",
        diagnostics,
    );
    check(
        totals.spring_loc,
        spring_loc,
        "spring_loc",
        "springs",
        diagnostics,
    );
    check(
        totals.total_loc,
        total_loc,
        "total_loc",
        "computed total",
        diagnostics,
    );

    let primal_tests: u64 = sum_field(registry, EntityKind::Primal, |e| e.tests);
    let spring_tests: u64 = sum_field(registry, EntityKind::Spring, |e| e.tests);

    check(
        totals.primal_tests,
        primal_tests,
        "primal_tests",
        "sum",
        diagnostics,
    );
    check(
        totals.spring_tests,
        spring_tests,
        "spring_tests",
        "sum",
        diagnostics,
    );

    if let Some(total_tests) = totals.total_tests {
        let computed = primal_tests + spring_tests;
        check(
            total_tests,
            computed,
            "total_tests",
            "computed total",
            diagnostics,
        );
    }
}

fn sum_field(
    registry: &HashMap<String, Entity>,
    kind: EntityKind,
    field: impl Fn(&Entity) -> Option<u64>,
) -> u64 {
    registry
        .values()
        .filter(|e| e.kind == kind)
        .filter_map(field)
        .sum()
}

fn check(stored: u64, computed: u64, field: &str, label: &str, diagnostics: &mut Vec<Diagnostic>) {
    if stored != computed {
        diagnostics.push(Diagnostic::Error(format!(
            "totals.{field}={stored} but {label}={computed}"
        )));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{EntityKind, Tier};

    fn registry_with(entries: Vec<(&str, EntityKind, u64, u64)>) -> HashMap<String, Entity> {
        entries
            .into_iter()
            .map(|(name, kind, loc, tests)| {
                (
                    name.to_string(),
                    Entity {
                        display: name.to_string(),
                        emoji: "🧪".into(),
                        kind,
                        description: Some("d".into()),
                        domain: Some("d".into()),
                        loc: Some(loc),
                        loc_display: Some(loc.to_string()),
                        tests: Some(tests),
                        tests_display: Some(tests.to_string()),
                        files: Some(1),
                        crates: Some(1),
                        repo: Some("r".into()),
                        tier: if kind == EntityKind::Primal {
                            Some(Tier::Foundation)
                        } else {
                            None
                        },
                        composes: None,
                        capabilities: None,
                        page: None,
                    },
                )
            })
            .collect()
    }

    fn make_totals(
        primal_loc: u64,
        spring_loc: u64,
        primal_tests: u64,
        spring_tests: u64,
    ) -> Totals {
        Totals {
            primal_loc,
            spring_loc,
            total_loc: primal_loc + spring_loc,
            primal_tests,
            spring_tests,
            total_tests: Some(primal_tests + spring_tests),
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
        }
    }

    #[test]
    fn correct_totals_pass() {
        let reg = registry_with(vec![
            ("a", EntityKind::Primal, 100, 10),
            ("b", EntityKind::Spring, 50, 5),
        ]);
        let totals = make_totals(100, 50, 10, 5);
        let mut diags = Vec::new();
        validate(&reg, &totals, &mut diags);
        assert!(diags.is_empty(), "expected no errors, got: {diags:?}");
    }

    #[test]
    fn wrong_total_detected() {
        let reg = registry_with(vec![("a", EntityKind::Primal, 100, 10)]);
        let totals = Totals {
            primal_loc: 999,
            spring_loc: 0,
            total_loc: 999,
            primal_tests: 10,
            spring_tests: 0,
            total_tests: None,
            ..make_totals(0, 0, 0, 0)
        };
        let mut diags = Vec::new();
        validate(&reg, &totals, &mut diags);
        assert!(diags.iter().any(|d| d.message().contains("primal_loc")));
    }

    #[test]
    fn multiple_entities_sum_correctly() {
        let reg = registry_with(vec![
            ("a", EntityKind::Primal, 100, 10),
            ("b", EntityKind::Primal, 200, 20),
            ("c", EntityKind::Spring, 50, 5),
            ("d", EntityKind::Spring, 150, 15),
        ]);
        let totals = make_totals(300, 200, 30, 20);
        let mut diags = Vec::new();
        validate(&reg, &totals, &mut diags);
        assert!(diags.is_empty(), "expected no errors, got: {diags:?}");
    }
}

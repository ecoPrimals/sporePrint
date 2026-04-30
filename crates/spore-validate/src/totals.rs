use crate::model::{Entity, EntityKind, Totals};
use std::collections::HashMap;

pub fn validate(
    registry: &HashMap<String, Entity>,
    totals: &Totals,
    errors: &mut Vec<String>,
) {
    let primal_loc: u64 = sum_field(registry, EntityKind::Primal, |e| e.loc);
    let spring_loc: u64 = sum_field(registry, EntityKind::Spring, |e| e.loc);
    let total_loc = primal_loc + spring_loc;

    check(totals.primal_loc, primal_loc, "primal_loc", "primals", errors);
    check(totals.spring_loc, spring_loc, "spring_loc", "springs", errors);
    check(totals.total_loc, total_loc, "total_loc", "computed total", errors);

    let primal_tests: u64 = sum_field(registry, EntityKind::Primal, |e| e.tests);
    let spring_tests: u64 = sum_field(registry, EntityKind::Spring, |e| e.tests);

    check(totals.primal_tests, primal_tests, "primal_tests", "sum", errors);
    check(totals.spring_tests, spring_tests, "spring_tests", "sum", errors);

    if let Some(total_tests) = totals.total_tests {
        let computed = primal_tests + spring_tests;
        check(total_tests, computed, "total_tests", "computed total", errors);
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
        .filter_map(|e| field(e))
        .sum()
}

fn check(stored: u64, computed: u64, field: &str, label: &str, errors: &mut Vec<String>) {
    if stored != computed {
        errors.push(format!("totals.{field}={stored} but {label}={computed}"));
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

    #[test]
    fn correct_totals_pass() {
        let reg = registry_with(vec![
            ("a", EntityKind::Primal, 100, 10),
            ("b", EntityKind::Spring, 50, 5),
        ]);
        let totals = Totals {
            primal_loc: 100,
            spring_loc: 50,
            total_loc: 150,
            primal_tests: 10,
            spring_tests: 5,
            total_tests: Some(15),
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
            measured_date: None,
        };
        let mut errors = Vec::new();
        validate(&reg, &totals, &mut errors);
        assert!(errors.is_empty(), "expected no errors, got: {errors:?}");
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
            measured_date: None,
        };
        let mut errors = Vec::new();
        validate(&reg, &totals, &mut errors);
        assert!(errors.iter().any(|e| e.contains("primal_loc")));
    }
}

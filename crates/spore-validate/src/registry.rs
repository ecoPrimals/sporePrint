// SPDX-License-Identifier: AGPL-3.0-or-later

//! Entity registry schema validation.
//!
//! Validates that each entity has the required fields for its kind,
//! and that tier assignments are consistent.

use crate::error::Diagnostic;
use crate::model::{Entity, EntityKind};
use std::collections::HashMap;

/// Validate all entities in the registry for schema completeness.
pub fn validate(registry: &HashMap<String, Entity>, diagnostics: &mut Vec<Diagnostic>) {
    let mut keys: Vec<&str> = registry.keys().map(String::as_str).collect();
    keys.sort_unstable();

    for key in keys {
        validate_entity(key, &registry[key], diagnostics);
    }
}

fn validate_entity(key: &str, entity: &Entity, diagnostics: &mut Vec<Diagnostic>) {
    let required = required_for_kind(entity.kind);
    let missing: Vec<&str> = required
        .iter()
        .copied()
        .filter(|f| !has_field(entity, f))
        .collect();

    if !missing.is_empty() {
        diagnostics.push(Diagnostic::error(format!(
            "[{key}] kind={} missing required fields: {{{}}}",
            entity.kind,
            missing.join(", ")
        )));
    }

    if entity.tier.is_some() && entity.kind != EntityKind::Primal {
        diagnostics.push(Diagnostic::error(format!(
            "[{key}] has tier but kind={} (tier is only valid for primals)",
            entity.kind
        )));
    }
}

const fn required_for_kind(kind: EntityKind) -> &'static [&'static str] {
    match kind {
        EntityKind::Primal => &[
            "domain",
            "loc",
            "loc_display",
            "tests",
            "tests_display",
            "files",
            "crates",
            "repo",
            "tier",
        ],
        EntityKind::Spring => &[
            "domain",
            "loc",
            "loc_display",
            "tests",
            "tests_display",
            "files",
            "crates",
            "repo",
        ],
        EntityKind::Product => &["domain"],
        EntityKind::Composition | EntityKind::Concept | EntityKind::Infra | EntityKind::Org => {
            &["description"]
        }
    }
}

fn has_field(entity: &Entity, field: &str) -> bool {
    match field {
        "description" => entity.description.is_some(),
        "domain" => entity.domain.is_some(),
        "loc" => entity.loc.is_some(),
        "loc_display" => entity.loc_display.is_some(),
        "tests" => entity.tests.is_some(),
        "tests_display" => entity.tests_display.is_some(),
        "files" => entity.files.is_some(),
        "crates" => entity.crates.is_some(),
        "repo" => entity.repo.is_some(),
        "tier" => entity.tier.is_some(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{EntityKind, Tier};

    fn make_primal() -> Entity {
        Entity {
            display: "Test".into(),
            emoji: "🧪".into(),
            kind: EntityKind::Primal,
            description: Some("desc".into()),
            domain: Some("Testing".into()),
            loc: Some(100),
            loc_display: Some("100".into()),
            tests: Some(10),
            tests_display: Some("10".into()),
            files: Some(5),
            crates: Some(1),
            repo: Some("org/repo".into()),
            tier: Some(Tier::Foundation),
            composes: None,
            capabilities: None,
            page: None,
            edges: None,
        }
    }

    #[test]
    fn complete_primal_passes() {
        let mut diags = Vec::new();
        validate_entity("test", &make_primal(), &mut diags);
        assert!(diags.is_empty(), "expected no errors, got: {diags:?}");
    }

    #[test]
    fn missing_tier_fails() {
        let mut e = make_primal();
        e.tier = None;
        let mut diags = Vec::new();
        validate_entity("test", &e, &mut diags);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message().contains("tier"));
    }

    #[test]
    fn tier_on_non_primal_fails() {
        let e = Entity {
            kind: EntityKind::Infra,
            tier: Some(Tier::Foundation),
            ..make_primal()
        };
        let mut diags = Vec::new();
        validate_entity("test", &e, &mut diags);
        assert!(diags.iter().any(|d| {
            let msg = d.message();
            msg.contains("tier") && msg.contains("infra")
        }));
    }

    #[test]
    fn product_needs_domain() {
        let e = Entity {
            kind: EntityKind::Product,
            domain: None,
            tier: None,
            ..make_primal()
        };
        let mut diags = Vec::new();
        validate_entity("test", &e, &mut diags);
        assert!(diags.iter().any(|d| d.message().contains("domain")));
    }

    #[test]
    fn concept_needs_description() {
        let e = Entity {
            kind: EntityKind::Concept,
            description: None,
            tier: None,
            ..make_primal()
        };
        let mut diags = Vec::new();
        validate_entity("test", &e, &mut diags);
        assert!(diags.iter().any(|d| d.message().contains("description")));
    }
}

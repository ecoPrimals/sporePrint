#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub extra: Extra,
}

#[derive(Debug, Deserialize)]
pub struct Extra {
    pub entity_registry: HashMap<String, Entity>,
    pub totals: Totals,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Entity {
    pub display: String,
    pub emoji: String,
    pub kind: EntityKind,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub loc: Option<u64>,
    #[serde(default)]
    pub loc_display: Option<String>,
    #[serde(default)]
    pub tests: Option<u64>,
    #[serde(default)]
    pub tests_display: Option<String>,
    #[serde(default)]
    pub files: Option<u32>,
    #[serde(default)]
    pub crates: Option<u32>,
    #[serde(default)]
    pub repo: Option<String>,
    #[serde(default)]
    pub tier: Option<Tier>,
    #[serde(default)]
    pub composes: Option<String>,
    #[serde(default)]
    pub capabilities: Option<Vec<Capability>>,
    #[serde(default)]
    pub page: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EntityKind {
    Primal,
    Spring,
    Product,
    Composition,
    Concept,
    Infra,
    Org,
}

impl fmt::Display for EntityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Primal => "primal",
            Self::Spring => "spring",
            Self::Product => "product",
            Self::Composition => "composition",
            Self::Concept => "concept",
            Self::Infra => "infra",
            Self::Org => "org",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Tier {
    Foundation,
    PostNucleus,
    Meta,
    Tooling,
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Foundation => "foundation",
            Self::PostNucleus => "post-nucleus",
            Self::Meta => "meta",
            Self::Tooling => "tooling",
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Capability {
    pub category: String,
    pub items: String,
}

#[derive(Debug, Deserialize)]
pub struct Totals {
    pub primal_loc: u64,
    pub spring_loc: u64,
    pub total_loc: u64,
    pub primal_tests: u64,
    pub spring_tests: u64,
    #[serde(default)]
    pub total_tests: Option<u64>,
    #[serde(default)]
    pub primal_loc_display: Option<String>,
    #[serde(default)]
    pub spring_loc_display: Option<String>,
    #[serde(default)]
    pub total_loc_display: Option<String>,
    #[serde(default)]
    pub primal_tests_display: Option<String>,
    #[serde(default)]
    pub spring_tests_display: Option<String>,
    #[serde(default)]
    pub total_tests_display: Option<String>,
    #[serde(default)]
    pub wgsl_files: Option<u64>,
    #[serde(default)]
    pub wgsl_lines: Option<u64>,
    #[serde(default)]
    pub wgsl_lines_display: Option<String>,
    #[serde(default)]
    pub validation_checks: Option<String>,
    #[serde(default)]
    pub papers_reproduced: Option<String>,
    #[serde(default)]
    pub measured_date: Option<String>,
}

pub fn parse_config(path: &Path) -> Result<Config, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("failed to parse TOML: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_round_trips() {
        let toml_str = r#"kind = "post-nucleus""#;
        #[derive(Deserialize)]
        struct W {
            kind: Tier,
        }
        let w: W = toml::from_str(toml_str).unwrap();
        assert_eq!(w.kind, Tier::PostNucleus);
        assert_eq!(w.kind.to_string(), "post-nucleus");
    }

    #[test]
    fn entity_kind_display() {
        assert_eq!(EntityKind::Primal.to_string(), "primal");
        assert_eq!(EntityKind::Composition.to_string(), "composition");
    }

    #[test]
    fn minimal_entity_deserializes() {
        let toml_str = r#"
            display = "test"
            emoji = "🧪"
            kind = "concept"
            description = "A test concept"
        "#;
        let e: Entity = toml::from_str(toml_str).unwrap();
        assert_eq!(e.kind, EntityKind::Concept);
        assert!(e.loc.is_none());
    }
}

// SPDX-License-Identifier: AGPL-3.0-or-later

//! Domain model for the sporePrint entity registry.
//!
//! All types here map directly to the `[extra.entity_registry]` and
//! `[extra.totals]` tables in `config.toml`. Deserialization is strict:
//! unknown fields are ignored gracefully, but required fields produce
//! typed parse errors.

use crate::error::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

/// Top-level Zola `config.toml` structure (only fields we consume).
#[derive(Debug, Deserialize)]
pub struct Config {
    pub extra: Extra,
}

#[derive(Debug, Deserialize)]
pub struct Extra {
    pub entity_registry: HashMap<String, Entity>,
    pub totals: Totals,
}

/// A single entity in the registry (primal, spring, product, etc.).
///
/// Fields like `display`, `emoji`, `composes`, `capabilities`, and `page` are
/// consumed by Zola templates at build time, not by this binary directly. They
/// must be deserialized to validate the config schema.
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
    /// Typed edges to other entities — renvois de choses.
    #[serde(default)]
    pub edges: Option<Vec<Edge>>,
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

impl EntityKind {
    /// Kinds that have Zola taxonomy pages (pluralized name → kind mapping).
    /// Derived from the type system, not hardcoded elsewhere.
    pub const fn taxonomy_pairs() -> &'static [(&'static str, Self)] {
        &[("primals", Self::Primal), ("springs", Self::Spring)]
    }
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

/// Capability declaration consumed by Zola templates for entity profile pages.
#[derive(Debug, Clone, Deserialize)]
pub struct Capability {
    pub category: String,
    pub items: String,
}

/// A typed, directed edge from one entity to another — a renvoi de choses.
///
/// Edges represent non-linguistic connections between ideas: structural
/// relationships that exist because of the nature of the things themselves,
/// not because of shared words. Implements the Diderot (1751) → Bush (1945)
/// → provenance trio (2026) lineage.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Edge {
    pub target: String,
    pub relation: EdgeRelation,
    #[serde(default)]
    pub weight: Option<u8>,
}

/// The taxonomy of intellectual relationships between entities.
///
/// Inspired by Diderot's four categories of renvois de choses:
/// analogy, common principle, contrast, and refutation — extended
/// for a scientific computing ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeRelation {
    /// A composes into B (A is a component of B)
    ComposesInto,
    /// A is validated by B (B proves A's correctness)
    ValidatedBy,
    /// A validates B (A proves B's correctness)
    Validates,
    /// A is compiled/built by B
    CompiledBy,
    /// A is derived from B (intellectual lineage)
    DerivedFrom,
    /// A reproduces B (A is a reproduction of B's results)
    Reproduces,
    /// A extends B (A builds upon B)
    Extends,
    /// A was preceded by B (temporal/evolutionary order)
    PrecededBy,
    /// A references B (general citation)
    References,
    /// A dispatches/orchestrates B
    Dispatches,
    /// A stores data for B
    StoresFor,
    /// A discovers/routes to B
    Discovers,
    /// A contradicts B (Diderot's "refutation" renvoi)
    Contradicts,
    /// A is analogous to B (Diderot's "analogy" renvoi)
    AnalogousTo,
}

impl fmt::Display for EdgeRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ComposesInto => "composes into",
            Self::ValidatedBy => "validated by",
            Self::Validates => "validates",
            Self::CompiledBy => "compiled by",
            Self::DerivedFrom => "derived from",
            Self::Reproduces => "reproduces",
            Self::Extends => "extends",
            Self::PrecededBy => "preceded by",
            Self::References => "references",
            Self::Dispatches => "dispatches",
            Self::StoresFor => "stores for",
            Self::Discovers => "discovers",
            Self::Contradicts => "contradicts",
            Self::AnalogousTo => "analogous to",
        })
    }
}

impl EdgeRelation {
    /// The inverse relation — if A has relation R to B, B has inverse(R) to A.
    /// This is what makes all connections bidirectional by construction.
    pub const fn inverse(self) -> Self {
        match self {
            Self::ComposesInto => Self::ComposesInto, // B "composed of" A (same type, reversed)
            Self::ValidatedBy => Self::Validates,
            Self::Validates => Self::ValidatedBy,
            Self::CompiledBy => Self::CompiledBy,
            Self::DerivedFrom => Self::Extends,
            Self::Reproduces => Self::Reproduces,
            Self::Extends => Self::DerivedFrom,
            Self::PrecededBy => Self::PrecededBy,
            Self::References => Self::References,
            Self::Dispatches => Self::Dispatches,
            Self::StoresFor => Self::StoresFor,
            Self::Discovers => Self::Discovers,
            Self::Contradicts => Self::Contradicts,
            Self::AnalogousTo => Self::AnalogousTo,
        }
    }
}

/// Aggregate metrics from `[extra.totals]`.
///
/// Display fields and optional metrics are consumed by Zola templates,
/// not directly by validation logic.
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
    pub basecamp_papers: Option<u32>,
    #[serde(default)]
    pub primal_count: Option<u32>,
    #[serde(default)]
    pub spring_count: Option<u32>,
    #[serde(default)]
    pub measured_date: Option<String>,
}

/// Parse `config.toml` into typed domain model.
pub fn parse_config(path: &Path) -> Result<Config, Error> {
    let text = std::fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_round_trips() {
        #[derive(Deserialize)]
        struct W {
            kind: Tier,
        }
        let toml_str = r#"kind = "post-nucleus""#;
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

    #[test]
    fn parse_config_returns_error_on_missing_file() {
        let result = parse_config(Path::new("/nonexistent/config.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn edge_deserializes() {
        let toml_str = r#"
            display = "test"
            emoji = "🧪"
            kind = "primal"
            edges = [
                { target = "hotspring", relation = "validated_by" },
                { target = "toweratomic", relation = "composes_into", weight = 3 },
            ]
        "#;
        let e: Entity = toml::from_str(toml_str).unwrap();
        let edges = e.edges.unwrap();
        assert_eq!(edges.len(), 2);
        assert_eq!(edges[0].relation, EdgeRelation::ValidatedBy);
        assert_eq!(edges[0].target, "hotspring");
        assert_eq!(edges[1].weight, Some(3));
    }

    #[test]
    fn edge_relation_inverse() {
        assert_eq!(EdgeRelation::ValidatedBy.inverse(), EdgeRelation::Validates);
        assert_eq!(EdgeRelation::Validates.inverse(), EdgeRelation::ValidatedBy);
        assert_eq!(EdgeRelation::DerivedFrom.inverse(), EdgeRelation::Extends);
        assert_eq!(EdgeRelation::Extends.inverse(), EdgeRelation::DerivedFrom);
    }
}

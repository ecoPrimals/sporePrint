// SPDX-License-Identifier: AGPL-3.0-or-later

//! Entity graph — typed bidirectional connections (renvois de choses).
//!
//! Reads edges from the entity registry, validates all targets resolve,
//! computes inverse edges for bidirectionality, and emits a JSON graph
//! suitable for Zola templates and future rhizoCrypt DAG integration.

use crate::error::Diagnostic;
use crate::model::Entity;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

/// A resolved, bidirectional edge in the graph.
#[derive(Debug, Clone, Serialize)]
pub struct ResolvedEdge {
    pub source: String,
    pub target: String,
    pub relation: String,
    pub inverse: bool,
}

/// The full entity graph — all nodes and their bidirectional edges.
#[derive(Debug, Serialize)]
pub struct EntityGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<ResolvedEdge>,
    pub stats: GraphStats,
}

#[derive(Debug, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub display: String,
    pub kind: String,
    pub emoji: String,
}

#[derive(Debug, Serialize)]
pub struct GraphStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub declared_edges: usize,
    pub inverse_edges: usize,
}

/// Build the entity graph from the registry, computing inverse edges.
pub fn build_graph(registry: &HashMap<String, Entity>) -> EntityGraph {
    let mut edges: Vec<ResolvedEdge> = Vec::new();

    for (source_key, entity) in registry {
        if let Some(ref entity_edges) = entity.edges {
            for edge in entity_edges {
                edges.push(ResolvedEdge {
                    source: source_key.clone(),
                    target: edge.target.clone(),
                    relation: edge.relation.to_string(),
                    inverse: false,
                });

                let inv_relation = edge.relation.inverse();
                edges.push(ResolvedEdge {
                    source: edge.target.clone(),
                    target: source_key.clone(),
                    relation: inv_relation.to_string(),
                    inverse: true,
                });
            }
        }
    }

    let declared = edges.iter().filter(|e| !e.inverse).count();
    let inverse = edges.iter().filter(|e| e.inverse).count();

    let nodes: Vec<GraphNode> = registry
        .iter()
        .map(|(key, entity)| GraphNode {
            id: key.clone(),
            display: entity.display.clone(),
            kind: entity.kind.to_string(),
            emoji: entity.emoji.clone(),
        })
        .collect();

    EntityGraph {
        stats: GraphStats {
            node_count: nodes.len(),
            edge_count: edges.len(),
            declared_edges: declared,
            inverse_edges: inverse,
        },
        nodes,
        edges,
    }
}

/// Validate that all edge targets resolve to existing registry keys.
pub fn validate_edges(registry: &HashMap<String, Entity>, diagnostics: &mut Vec<Diagnostic>) {
    for (source_key, entity) in registry {
        if let Some(ref edges) = entity.edges {
            for edge in edges {
                if !registry.contains_key(&edge.target) {
                    diagnostics.push(Diagnostic::error(format!(
                        "edge: [{source_key}] → [{}] ({}) — target not found in registry",
                        edge.target, edge.relation
                    )));
                }
            }
        }
    }
}

/// Emit the entity graph as JSON to the specified path.
pub fn emit_graph_json(graph: &EntityGraph, output_path: &Path) -> std::io::Result<()> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(graph).map_err(std::io::Error::other)?;
    std::fs::write(output_path, json)
}

/// Get edges for a specific entity (both outbound and inbound).
/// Used by Phase 2 template integration and CLI query commands.
#[cfg_attr(not(test), allow(dead_code))]
pub fn edges_for_entity<'a>(
    graph: &'a EntityGraph,
    entity_id: &str,
) -> (Vec<&'a ResolvedEdge>, Vec<&'a ResolvedEdge>) {
    let outbound: Vec<&'a ResolvedEdge> = graph
        .edges
        .iter()
        .filter(|e| e.source == entity_id && !e.inverse)
        .collect();
    let inbound: Vec<&'a ResolvedEdge> = graph
        .edges
        .iter()
        .filter(|e| e.target == entity_id && !e.inverse)
        .collect();
    (outbound, inbound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Edge, EdgeRelation, EntityKind, Tier};

    fn test_registry() -> HashMap<String, Entity> {
        let mut reg = HashMap::new();
        reg.insert(
            "alpha".to_string(),
            Entity {
                display: "Alpha".into(),
                emoji: "A".into(),
                kind: EntityKind::Primal,
                description: None,
                domain: Some("Test".into()),
                loc: None,
                loc_display: None,
                tests: None,
                tests_display: None,
                files: None,
                crates: None,
                repo: None,
                tier: Some(Tier::Foundation),
                composes: None,
                capabilities: None,
                page: None,
                edges: Some(vec![
                    Edge {
                        target: "beta".into(),
                        relation: EdgeRelation::ComposesInto,
                        weight: None,
                    },
                    Edge {
                        target: "gamma".into(),
                        relation: EdgeRelation::ValidatedBy,
                        weight: None,
                    },
                ]),
            },
        );
        reg.insert(
            "beta".to_string(),
            Entity {
                display: "Beta".into(),
                emoji: "B".into(),
                kind: EntityKind::Composition,
                description: Some("comp".into()),
                domain: None,
                loc: None,
                loc_display: None,
                tests: None,
                tests_display: None,
                files: None,
                crates: None,
                repo: None,
                tier: None,
                composes: None,
                capabilities: None,
                page: None,
                edges: None,
            },
        );
        reg.insert(
            "gamma".to_string(),
            Entity {
                display: "Gamma".into(),
                emoji: "G".into(),
                kind: EntityKind::Spring,
                description: None,
                domain: Some("Validation".into()),
                loc: None,
                loc_display: None,
                tests: None,
                tests_display: None,
                files: None,
                crates: None,
                repo: None,
                tier: None,
                composes: None,
                capabilities: None,
                page: None,
                edges: None,
            },
        );
        reg
    }

    #[test]
    fn build_graph_creates_bidirectional_edges() {
        let reg = test_registry();
        let graph = build_graph(&reg);

        assert_eq!(graph.stats.node_count, 3);
        assert_eq!(graph.stats.declared_edges, 2);
        assert_eq!(graph.stats.inverse_edges, 2);
        assert_eq!(graph.stats.edge_count, 4);
    }

    #[test]
    fn validate_catches_missing_target() {
        let mut reg = test_registry();
        reg.get_mut("alpha").unwrap().edges = Some(vec![Edge {
            target: "nonexistent".into(),
            relation: EdgeRelation::References,
            weight: None,
        }]);

        let mut diags = Vec::new();
        validate_edges(&reg, &mut diags);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message().contains("nonexistent"));
    }

    #[test]
    fn edges_for_entity_finds_connections() {
        let reg = test_registry();
        let graph = build_graph(&reg);
        let (outbound, inbound) = edges_for_entity(&graph, "alpha");
        assert_eq!(outbound.len(), 2);
        assert_eq!(inbound.len(), 0);

        let (out_beta, in_beta) = edges_for_entity(&graph, "beta");
        assert_eq!(out_beta.len(), 0);
        assert_eq!(in_beta.len(), 1);
    }
}

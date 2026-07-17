// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    content, error::Diagnostic, error::Error, graph, links, model, paths, registry, report, totals,
};
use std::path::Path;

fn validate_registry(config: &model::Config) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    registry::validate(&config.extra.entity_registry, &mut diags);
    graph::validate_edges(&config.extra.entity_registry, &mut diags);
    totals::validate(
        &config.extra.entity_registry,
        &config.extra.totals,
        &mut diags,
    );
    diags
}

fn validate_content(root: &Path, config: &model::Config, check: bool) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let content_dir = root.join(paths::CONTENT_DIR);
    if content_dir.is_dir() {
        content::validate_taxonomies(
            root,
            &content_dir,
            &config.extra.entity_registry,
            &mut diags,
        );
        content::lint_internal_links(root, &content_dir, &mut diags);

        if check {
            content::check_integrity(
                root,
                &content_dir,
                &config.extra.entity_registry,
                &mut diags,
            );
            content::validate_maturity_levels(&content_dir, &mut diags);
            content::audit_taxonomy_coverage(
                root,
                &content_dir,
                &config.extra.entity_registry,
                &mut diags,
            );
            let link_warnings = links::validate_internal_links(&content_dir);
            diags.extend(link_warnings);
        }
    }
    diags
}

pub fn validate(
    root: &Path,
    config: &model::Config,
    check: bool,
    strict: bool,
    verbose: bool,
) -> Result<(), Error> {
    println!("spore-validate: checking sporePrint entity registry...");

    if verbose {
        print!("{}", report::format_registry(&config.extra.entity_registry));
        print!("{}", report::format_totals(&config.extra.totals));
    }

    let mut diagnostics = validate_registry(config);
    diagnostics.extend(validate_content(root, config, check));

    if strict {
        for diag in &mut diagnostics {
            diag.promote_to_error();
        }
    }

    let infos: Vec<&Diagnostic> = diagnostics.iter().filter(|d| d.is_info()).collect();
    let warnings: Vec<&Diagnostic> = diagnostics.iter().filter(|d| d.is_warning()).collect();
    let errors: Vec<&Diagnostic> = diagnostics.iter().filter(|d| d.is_error()).collect();

    for i in &infos {
        println!("  INFO:  {}", i.message());
    }
    for w in &warnings {
        println!("  WARN:  {}", w.message());
    }
    for e in &errors {
        println!("  ERROR: {}", e.message());
    }

    let summary = report::summarize(config);

    if errors.is_empty() {
        println!(
            "  OK: {} entities ({} primals, {} springs), {} warning(s), 0 errors",
            summary.entity_count,
            summary.primal_count,
            summary.spring_count,
            warnings.len()
        );
        Ok(())
    } else {
        println!(
            "\n  {} error(s), {} warning(s)",
            errors.len(),
            warnings.len()
        );
        Err(Error::ValidationFailed {
            error_count: errors.len(),
            warning_count: warnings.len(),
        })
    }
}

/// Scan content files for `viz_embed` shortcode invocations.
///
/// Returns a sorted, deduplicated list of visualization names found in content.
pub fn scan_viz_embeds(content_dir: &Path) -> Vec<String> {
    use std::sync::LazyLock;

    use regex::Regex;

    static VIZ_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"\{\{[\s]*viz_embed\s*\([^)]*name\s*=\s*"([^"]+)""#)
            .unwrap_or_else(|e| unreachable!("VIZ_RE is a static literal: {e}"))
    });

    let mut names = std::collections::BTreeSet::new();

    for entry in crate::paths::walk_markdown_files(content_dir) {
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            for cap in VIZ_RE.captures_iter(&content) {
                if let Some(m) = cap.get(1) {
                    names.insert(m.as_str().to_string());
                }
            }
        }
    }

    names.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_viz_embeds_finds_names() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("arch");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(
            sub.join("test.md"),
            r#"+++
title = "Test"
+++

{{ viz_embed(name="entity-graph", fallback="static/viz/entity-graph.svg") }}

Some content here.

{{ viz_embed(name="gate-mesh", fallback="static/viz/gate-mesh.svg") }}
"#,
        )
        .unwrap();

        let names = scan_viz_embeds(dir.path());
        assert_eq!(names, vec!["entity-graph", "gate-mesh"]);
    }

    #[test]
    fn scan_viz_embeds_deduplicates() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("a.md"),
            r#"{{ viz_embed(name="entity-graph", fallback="x") }}"#,
        )
        .unwrap();
        std::fs::write(
            dir.path().join("b.md"),
            r#"{{ viz_embed(name="entity-graph", fallback="y") }}"#,
        )
        .unwrap();

        let names = scan_viz_embeds(dir.path());
        assert_eq!(names.len(), 1);
    }

    #[test]
    fn scan_viz_embeds_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let names = scan_viz_embeds(dir.path());
        assert!(names.is_empty());
    }
}

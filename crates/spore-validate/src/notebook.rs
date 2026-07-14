// SPDX-License-Identifier: AGPL-3.0-or-later

//! Jupyter notebook rendering to Zola-compatible markdown.
//!
//! Parses `.ipynb` JSON directly (no nbconvert dependency) and emits
//! Zola-compatible markdown with TOML front matter.

use crate::paths;
use crate::time::today_utc;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct Notebook {
    cells: Vec<Cell>,
    #[serde(default)]
    metadata: Option<NotebookMetadata>,
}

#[derive(Debug, Deserialize)]
struct NotebookMetadata {
    #[serde(default)]
    kernelspec: Option<KernelSpec>,
}

#[derive(Debug, Deserialize)]
struct KernelSpec {
    #[serde(default)]
    language: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Cell {
    #[serde(rename = "cell_type")]
    kind: String,
    source: Vec<String>,
    #[serde(default)]
    outputs: Vec<Output>,
}

#[derive(Debug, Deserialize)]
struct Output {
    #[serde(default, rename = "output_type")]
    kind: String,
    #[serde(default)]
    text: Option<Vec<String>>,
    #[serde(default)]
    data: Option<OutputData>,
}

#[derive(Debug, Deserialize)]
struct OutputData {
    #[serde(rename = "text/html", default)]
    text_html: Option<Vec<String>>,
    #[serde(rename = "text/plain", default)]
    text_plain: Option<Vec<String>>,
}

/// Convert a filename to a URL-safe slug.
fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Extract the first `# Title` from markdown cells, or use fallback.
/// Sanitizes the result for TOML string compatibility (no bare backslashes).
fn extract_title(nb: &Notebook, fallback: &str) -> String {
    for cell in &nb.cells {
        if cell.kind != "markdown" {
            continue;
        }
        for line in &cell.source {
            let trimmed = line.trim();
            if let Some(title) = trimmed.strip_prefix("# ") {
                return sanitize_toml_string(title);
            }
        }
    }
    sanitize_toml_string(fallback)
}

/// Sanitize a string for use in TOML quoted values.
/// Replaces bare backslashes that aren't valid TOML escape sequences.
fn sanitize_toml_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.peek().copied() {
                Some(esc @ ('b' | 'f' | 'n' | 'r' | 't' | 'u' | 'U' | '\\' | '"')) => {
                    result.push('\\');
                    result.push(esc);
                    chars.next();
                }
                _ => result.push(','),
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Detect the notebook's code language from metadata, defaulting to "python".
fn detect_language(nb: &Notebook) -> &str {
    nb.metadata
        .as_ref()
        .and_then(|m| m.kernelspec.as_ref())
        .and_then(|k| k.language.as_deref())
        .unwrap_or("python")
}

/// Render notebook cells into markdown body content.
fn render_cells(nb: &Notebook) -> String {
    let lang = detect_language(nb);
    let mut out = String::new();

    for cell in &nb.cells {
        match cell.kind.as_str() {
            "markdown" => {
                let content: String = cell.source.concat();
                out.push_str(&content);
                out.push_str("\n\n");
            }
            "code" => {
                let source: String = cell.source.concat();
                if !source.trim().is_empty() {
                    out.push_str("```");
                    out.push_str(lang);
                    out.push('\n');
                    out.push_str(&source);
                    if !source.ends_with('\n') {
                        out.push('\n');
                    }
                    out.push_str("```\n\n");
                }
                render_outputs(&cell.outputs, &mut out);
            }
            _ => {}
        }
    }

    out
}

fn render_outputs(outputs: &[Output], out: &mut String) {
    for output in outputs {
        if let Some(data) = &output.data {
            if let Some(html) = &data.text_html {
                out.push_str(&html.concat());
                out.push_str("\n\n");
                continue;
            }
            if let Some(plain) = &data.text_plain {
                out.push_str("```\n");
                out.push_str(&plain.concat());
                out.push_str("\n```\n\n");
                continue;
            }
        }
        if output.kind == "stream" {
            if let Some(text) = &output.text {
                out.push_str("```\n");
                out.push_str(&text.concat());
                out.push_str("\n```\n\n");
            }
        }
    }
}

/// Render a single notebook file to Zola markdown.
fn render_one(nb_path: &Path, output_dir: &Path) -> Result<String, crate::error::Error> {
    let text = std::fs::read_to_string(nb_path).map_err(|e| crate::error::Error::Io {
        path: nb_path.to_path_buf(),
        source: e,
    })?;

    let nb: Notebook = serde_json::from_str(&text).map_err(|e| crate::error::Error::Parse {
        context: format!("notebook {}", nb_path.display()),
        message: e.to_string(),
    })?;

    let stem = nb_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let slug = slugify(&stem);
    let title = extract_title(&nb, &stem);
    let body = render_cells(&nb);

    if body.trim().is_empty() {
        return Err(crate::error::Error::Parse {
            context: format!("notebook {}", nb_path.display()),
            message: "no content cells".into(),
        });
    }

    let date = today_utc();
    let weight = crate::paths::NOTEBOOK_DEFAULT_WEIGHT;
    let domain = crate::paths::NOTEBOOK_DEFAULT_DOMAIN;
    let page = format!(
        "+++\n\
         title = \"{title}\"\n\
         description = \"Rendered from {stem}.ipynb\"\n\
         date = {date}\n\
         weight = {weight}\n\
         \n\
         [extra]\n\
         domain = \"{domain}\"\n\
         rendered_from = \"{stem}.ipynb\"\n\
         +++\n\
         \n\
         <!-- Auto-generated from {stem}.ipynb by spore-validate render-notebooks -->\n\
         \n\
         {body}"
    );

    let out_path = output_dir.join(format!("{slug}.md"));
    std::fs::write(&out_path, &page).map_err(|e| crate::error::Error::Io {
        path: out_path.clone(),
        source: e,
    })?;

    Ok(format!("{} -> {}", nb_path.display(), out_path.display()))
}

/// Render all notebooks from given directories into Zola content.
///
/// Output lands in `{root}/{DEFAULT_NOTEBOOK_OUTPUT}` (configurable via
/// `SPOREPRINT_NOTEBOOK_OUTPUT` env var for non-standard layouts).
pub fn render_notebooks(
    root: &Path,
    notebook_dirs: &[PathBuf],
    springs_root: Option<&Path>,
) -> (u32, Vec<String>) {
    let output_subdir =
        std::env::var(paths::ENV_NOTEBOOK_OUTPUT).unwrap_or_else(|_| paths::NOTEBOOK_OUTPUT.into());
    let output_dir = root.join(&output_subdir);
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        return (
            0,
            vec![format!(
                "cannot create notebook output directory {}: {e}",
                output_dir.display()
            )],
        );
    }

    let mut rendered = 0u32;
    let mut messages = Vec::new();

    let mut dirs: Vec<PathBuf> = notebook_dirs.to_vec();

    if let Some(sr) = springs_root {
        if sr.is_dir() {
            for entry in std::fs::read_dir(sr).into_iter().flatten().flatten() {
                let nb_dir = entry.path().join("notebooks");
                if nb_dir.is_dir() {
                    dirs.push(nb_dir);
                }
            }
        }
    }

    for dir in &dirs {
        if !dir.is_dir() {
            messages.push(format!("SKIP: {} (not a directory)", dir.display()));
            continue;
        }
        for entry in WalkDir::new(dir)
            .max_depth(2)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "ipynb") {
                continue;
            }
            match render_one(path, &output_dir) {
                Ok(msg) => {
                    rendered += 1;
                    messages.push(msg);
                }
                Err(e) => messages.push(format!("SKIP: {e}")),
            }
        }
    }

    (rendered, messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_works() {
        assert_eq!(slugify("My_Cool_Notebook"), "my-cool-notebook");
        assert_eq!(slugify("UPPER CASE"), "upper-case");
        assert_eq!(slugify("a--b__c"), "a-b-c");
    }

    #[test]
    fn slugify_handles_special_chars() {
        assert_eq!(slugify("file (1).ipynb"), "file-1-ipynb");
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn title_extraction() {
        let nb: Notebook = serde_json::from_str(
            r##"{
                "cells": [
                    {"cell_type": "markdown", "source": ["# Hello World\n", "Some text"], "outputs": []},
                    {"cell_type": "code", "source": ["x = 1"], "outputs": []}
                ]
            }"##,
        )
        .unwrap();
        assert_eq!(extract_title(&nb, "fallback"), "Hello World");
    }

    #[test]
    fn title_fallback() {
        let nb: Notebook = serde_json::from_str(
            r#"{"cells": [{"cell_type": "code", "source": ["x = 1"], "outputs": []}]}"#,
        )
        .unwrap();
        assert_eq!(extract_title(&nb, "my_notebook"), "my_notebook");
    }

    #[test]
    fn render_cells_produces_markdown() {
        let json = "{\"cells\": [\
            {\"cell_type\": \"markdown\", \"source\": [\"# Title\\n\"], \"outputs\": []},\
            {\"cell_type\": \"code\", \"source\": [\"print('hi')\"], \"outputs\": [\
            {\"output_type\": \"stream\", \"text\": [\"hi\\n\"]}\
            ]}]}";
        let nb: Notebook = serde_json::from_str(json).unwrap();
        let body = render_cells(&nb);
        assert!(body.contains("# Title"));
        assert!(body.contains("```python"));
        assert!(body.contains("print('hi')"));
    }

    #[test]
    fn render_one_creates_file() {
        let dir = tempfile::tempdir().unwrap();
        let nb_path = dir.path().join("test.ipynb");
        let json = "{\"cells\": [{\"cell_type\": \"markdown\", \
            \"source\": [\"# Test\\n\", \"Content\"], \"outputs\": []}]}";
        std::fs::write(&nb_path, json).unwrap();

        let out_dir = dir.path().join("output");
        std::fs::create_dir_all(&out_dir).unwrap();

        let result = render_one(&nb_path, &out_dir);
        assert!(result.is_ok());
        assert!(out_dir.join("test.md").exists());

        let content = std::fs::read_to_string(out_dir.join("test.md")).unwrap();
        assert!(content.contains("title = \"Test\""));
        assert!(content.contains("Content"));
    }

}

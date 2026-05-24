use serde::Deserialize;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct Notebook {
    cells: Vec<Cell>,
}

#[derive(Debug, Deserialize)]
#[allow(clippy::struct_field_names)]
struct Cell {
    cell_type: String,
    source: Vec<String>,
    #[serde(default)]
    outputs: Vec<Output>,
}

#[derive(Debug, Deserialize)]
#[allow(clippy::struct_field_names)]
struct Output {
    #[serde(default)]
    output_type: String,
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

fn extract_title(nb: &Notebook, fallback: &str) -> String {
    for cell in &nb.cells {
        if cell.cell_type != "markdown" {
            continue;
        }
        for line in &cell.source {
            let trimmed = line.trim();
            if let Some(title) = trimmed.strip_prefix("# ") {
                return title.to_string();
            }
        }
    }
    fallback.to_string()
}

fn render_cells(nb: &Notebook) -> String {
    let mut out = String::new();

    for cell in &nb.cells {
        match cell.cell_type.as_str() {
            "markdown" => {
                let content: String = cell.source.concat();
                out.push_str(&content);
                out.push_str("\n\n");
            }
            "code" => {
                let source: String = cell.source.concat();
                if !source.trim().is_empty() {
                    out.push_str("```python\n");
                    out.push_str(&source);
                    if !source.ends_with('\n') {
                        out.push('\n');
                    }
                    out.push_str("```\n\n");
                }

                for output in &cell.outputs {
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
                    if output.output_type == "stream"
                        && let Some(text) = &output.text
                    {
                        out.push_str("```\n");
                        out.push_str(&text.concat());
                        out.push_str("\n```\n\n");
                    }
                }
            }
            _ => {}
        }
    }

    out
}

fn today() -> String {
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = epoch / 86400;
    let mut y = 1970i32;
    let mut rem = days;
    loop {
        let year_days: u64 = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
            366
        } else {
            365
        };
        if rem < year_days {
            break;
        }
        rem -= year_days;
        y += 1;
    }
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let month_days: &[u64] = if leap {
        &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0;
    for md in month_days {
        if rem < *md {
            break;
        }
        rem -= md;
        m += 1;
    }
    format!("{y}-{:02}-{:02}", m + 1, rem + 1)
}

fn render_one(nb_path: &Path, output_dir: &Path) -> Result<String, String> {
    let text = std::fs::read_to_string(nb_path)
        .map_err(|e| format!("failed to read {}: {e}", nb_path.display()))?;

    let nb: Notebook =
        serde_json::from_str(&text).map_err(|e| format!("failed to parse notebook JSON: {e}"))?;

    let stem = nb_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let slug = slugify(&stem);
    let title = extract_title(&nb, &stem);
    let body = render_cells(&nb);

    if body.trim().is_empty() {
        return Err(format!("SKIP: no content in {}", nb_path.display()));
    }

    let date = today();
    let page = format!(
        "+++\n\
         title = \"{title}\"\n\
         description = \"Rendered from {stem}.ipynb\"\n\
         date = {date}\n\
         weight = 50\n\
         \n\
         [extra]\n\
         domain = \"Lab\"\n\
         rendered_from = \"{stem}.ipynb\"\n\
         +++\n\
         \n\
         <!-- Auto-generated from {stem}.ipynb by spore-validate render-notebooks -->\n\
         \n\
         {body}"
    );

    let out_path = output_dir.join(format!("{slug}.md"));
    std::fs::write(&out_path, &page)
        .map_err(|e| format!("failed to write {}: {e}", out_path.display()))?;

    Ok(format!("{} → {}", nb_path.display(), out_path.display()))
}

pub fn render_notebooks(
    root: &Path,
    notebook_dirs: &[PathBuf],
    springs_root: Option<&Path>,
) -> (u32, Vec<String>) {
    let output_dir = root.join("content/lab/notebooks");
    let _ = std::fs::create_dir_all(&output_dir);

    let mut rendered = 0u32;
    let mut messages = Vec::new();

    let mut dirs: Vec<PathBuf> = notebook_dirs.to_vec();

    if let Some(sr) = springs_root
        && sr.is_dir()
    {
        for entry in std::fs::read_dir(sr).into_iter().flatten().flatten() {
            let nb_dir = entry.path().join("notebooks");
            if nb_dir.is_dir() {
                dirs.push(nb_dir);
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
                Err(msg) => messages.push(msg),
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
}

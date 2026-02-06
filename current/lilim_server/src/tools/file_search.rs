use super::{FileMatch, Tool, ToolArgs, ToolResult};
use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct FileSearchTool {
    allowed_paths: Vec<PathBuf>,
}

impl FileSearchTool {
    pub fn new() -> Self {
        Self {
            allowed_paths: vec![
                PathBuf::from("/home/blanco"),
            ],
        }
    }

    fn is_allowed_path(&self, path: &str) -> bool {
        let path_buf = PathBuf::from(path);
        self.allowed_paths.iter().any(|allowed| {
            path_buf.starts_with(allowed)
        })
    }
}

#[async_trait::async_trait]
impl Tool for FileSearchTool {
    fn name(&self) -> &str {
        "file_search"
    }

    fn description(&self) -> &str {
        "Search for files by name or content in /home/blanco"
    }

    fn requires_confirmation(&self) -> bool {
        false // File search is safe (read-only)
    }

    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult> {
        let query = args.get_str("query")?;
        let path = args.get_str("path").unwrap_or("/home/blanco");
        let content_search = args.get_bool("content", false);

        // Validate path
        if !self.is_allowed_path(path) {
            return Err(anyhow::anyhow!("Path not allowed: {}", path));
        }

        let mut matches = Vec::new();

        if content_search {
            // Content search using grep-like functionality
            matches = search_file_content(path, query, 50)?;
        } else {
            // Filename search
            matches = search_file_names(path, query, 50)?;
        }

        Ok(ToolResult::FileSearch {
            matches,
            query: query.to_string(),
        })
    }
}

fn search_file_names(path: &str, query: &str, limit: usize) -> Result<Vec<FileMatch>> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    for entry in WalkDir::new(path)
        .max_depth(10)
        .into_iter()
        .filter_map(|e| e.ok())
        .take(limit * 2)
    {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.to_lowercase().contains(&query_lower) {
            results.push(FileMatch {
                path: entry.path().display().to_string(),
                matches: vec![file_name.to_string()],
            });

            if results.len() >= limit {
                break;
            }
        }
    }

    Ok(results)
}

fn search_file_content(path: &str, query: &str, limit: usize) -> Result<Vec<FileMatch>> {
    use regex::Regex;
    use std::fs;
    use std::io::{BufRead, BufReader};

    let re = Regex::new(&regex::escape(query))?;
    let mut results = Vec::new();

    for entry in WalkDir::new(path)
        .max_depth(10)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        // Skip binary and large files
        if let Ok(metadata) = entry.metadata() {
            if metadata.len() > 10_000_000 {
                continue; // Skip files > 10MB
            }
        }

        let file_path = entry.path();
        if let Ok(file) = fs::File::open(file_path) {
            let reader = BufReader::new(file);
            let mut file_matches = Vec::new();

            for (line_num, line) in reader.lines().enumerate() {
                if let Ok(line_content) = line {
                    if re.is_match(&line_content) {
                        file_matches.push(format!("L{}: {}", line_num + 1, line_content.trim()));
                        if file_matches.len() >= 5 {
                            break;
                        }
                    }
                }
            }

            if !file_matches.is_empty() {
                results.push(FileMatch {
                    path: file_path.display().to_string(),
                    matches: file_matches,
                });

                if results.len() >= limit {
                    break;
                }
            }
        }
    }

    Ok(results)
}

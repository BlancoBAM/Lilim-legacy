use super::{Tool, ToolArgs, ToolResult, WebSearchResult};
use anyhow::Result;

pub struct WebSearchTool;

impl WebSearchTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "Search the web using DuckDuckGo. Returns top 5 results."
    }

    fn requires_confirmation(&self) -> bool {
        false // Web search is safe
    }

    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult> {
        let query = args.get_str("query")?;
        
        // Use DuckDuckGo HTML API
        let url = format!(
            "https://html.duckduckgo.com/html/?q={}",
            urlencoding::encode(query)
        );

        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
            .build()?;

        let response = client.get(&url).send().await?;
        let html = response.text().await?;

        // Parse results using scraper
        let results = parse_ddg_html(&html)?;

        Ok(ToolResult::WebSearch {
            results,
            query: query.to_string(),
        })
    }
}

fn parse_ddg_html(html: &str) -> Result<Vec<WebSearchResult>> {
    use scraper::{Html, Selector};

    let document = Html::parse_document(html);
    let result_selector = Selector::parse(".result").unwrap();
    let title_selector = Selector::parse(".result__a").unwrap();
    let snippet_selector = Selector::parse(".result__snippet").unwrap();

    let mut results = Vec::new();

    for element in document.select(&result_selector).take(5) {
        let title = element
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        let url = element
            .select(&title_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let snippet = element
            .select(&snippet_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        if !title.is_empty() && !url.is_empty() {
            results.push(WebSearchResult {
                title: title.trim().to_string(),
                url,
                snippet: snippet.trim().to_string(),
            });
        }
    }

    Ok(results)
}

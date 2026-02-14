use anyhow::Result;
use fastembed::{TextEmbedding, InitOptions};
use embedvec::{Distance, EmbedVec, BackendConfig, Quantization};
use tantivy::{schema::*, Index, query::QueryParser};
use std::path::Path;
use std::io::{self, Write};


#[tokio::main]
async fn main() -> Result<()> {
    let db_dir = Path::new("../lilith_rag_builder/rag_ready/rag_index");
    let keyword_dir = Path::new("../lilith_rag_builder/rag_ready/keyword_index");

    println!("[*] Initializing Lilim's hybrid memory...");

    // --- 1. Load Vector Index (embedvec) ---
    let persistence = BackendConfig::new(db_dir.to_string_lossy());
    let mut v_db = EmbedVec::new_internal(
        384,
        Distance::Cosine,
        32,
        200,
        Quantization::None,
        Some(persistence),
    )?;

    // --- 2. Load Keyword Index (tantivy) ---
    let schema = build_schema();
    let index = Index::open_in_dir(keyword_dir)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();

    // --- 3. Init Embedding Model ---
    let mut embedder = TextEmbedding::try_new(InitOptions::default())?;

    println!("[*] Lilim's hybrid brain is online.");
    println!("[*] Ask a Linux or Medical question (or type 'exit' to quit).\n");

    loop {
        print!("User > ");
        io::stdout().flush()?;
        let mut query_str = String::new();
        io::stdin().read_line(&mut query_str)?;
        let query_str = query_str.trim();

        if query_str == "exit" || query_str == "quit" {
            println!("[*] Shutting down Lilim...");
            break;
        }

        if query_str.is_empty() {
            continue;
        }

        println!("\n--- Lilim's Hybrid Search Results ---\n");

        // --- A. Keyword Search (Literal Match) ---
        let text_field = schema.get_field("text").unwrap();
        let category_field = schema.get_field("category").unwrap();
        let authority_field = schema.get_field("authority").unwrap();

        let query_parser = QueryParser::for_index(&index, vec![text_field]);
        let query = query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(3))?;

        if !top_docs.is_empty() {
            println!("🔍 KEYWORD MATCHES (Exact/Literal):");
            for (score, doc_address) in top_docs {
                let retrieved_doc: tantivy::TantivyDocument = searcher.doc(doc_address)?;
                let text = retrieved_doc.get_first(text_field)
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let category = retrieved_doc.get_first(category_field)
                    .and_then(|v| v.as_str())
                    .unwrap_or("General");
                let authority = retrieved_doc.get_first(authority_field)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1);

                println!("  [Score: {:.4}] [Category: {}] [Authority: {}]", score, category, authority);
                println!("  {}\n", text);
            }
        } else {
            println!("🔍 KEYWORD MATCHES: None\n");
        }

        // --- B. Vector Search (Conceptual/Semantic) ---
        let q_vec = embedder.embed(vec![query_str], None)?[0].clone();
        let vector_results = v_db.search(&q_vec, 3, 100, None).await?;

        if !vector_results.is_empty() {
            println!("🧠 VECTOR MATCHES (Semantic/Conceptual):");
            for hit in vector_results {
                let text = hit.payload.get("text")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let category = hit.payload.get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("General");
                let authority = hit.payload.get("authority")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1);

                println!("  [Score: {:.4}] [Category: {}] [Authority: {}]", hit.score, category, authority);
                println!("  {}\n", text);
            }
        } else {
            println!("🧠 VECTOR MATCHES: None\n");
        }

        println!("---\n");
    }

    Ok(())
}

fn build_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("text", TEXT | STORED);
    schema_builder.add_text_field("category", STRING | STORED);
    schema_builder.add_u64_field("authority", STORED);
    schema_builder.add_u64_field("id", STORED);
    schema_builder.build()
}

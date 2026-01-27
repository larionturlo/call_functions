fn main() {
    dotenv::dotenv().ok();
    let ollama_host =
        std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "127.0.0.1:11434".to_string());
    // todo make getting cases_path and report_path from arguments
    call_functions::reqwest::run(&ollama_host, get_models(), None, None);
}

fn get_models() -> &'static [&'static str] {
    &[
        // "llama3-groq-tool-use:latest",
        "granite4:3b",
        "functiongemma:latest",
        // "nemotron-mini:4b",
    ]
}

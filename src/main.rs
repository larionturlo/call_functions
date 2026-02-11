use rig::client::ProviderClient;
use rig::providers::ollama;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let client = ollama::Client::from_env();

    // Run person extraction
    println!("Running person extraction tests...");
    match call_functions::extract::person::run(get_models(), client.clone(), None).await {
        Ok(_) => println!("Person extraction tests completed successfully"),
        Err(err) => {
            eprintln!("Error in person extraction: {}", err);
            return Err(anyhow::Error::msg("An error occurred in person extraction"));
        }
    }

    // Run translation tests
    println!("Running translation tests...");
    match call_functions::translate::run(get_translation_models(), client, None).await {
        Ok(_) => println!("Translation tests completed successfully"),
        Err(err) => {
            eprintln!("Error in translation: {}", err);
            println!("Continuing despite translation error...");
        }
    }

    Ok(())
}

// fn main() {
//     dotenv::dotenv().ok();
//     let ollama_host =
//         std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "127.0.0.1:11434".to_string());
//     // todo make getting cases_path and report_path from arguments
//     call_functions::reqwest::run(&ollama_host, get_models(), None, None);
// }

fn get_models() -> &'static [&'static str] {
    &["functiongemma:latest"]
}

fn get_translation_models() -> &'static [&'static str] {
    &["translategemma:latest"]
}

pub mod cases;
pub mod ollama;

use crate::reqwest::cases::simple;
const CSV_DELIMETR: &str = "|";

pub fn run(ollama_host: &str, models: &[&str]) {
    let ollama_client = ollama::Client::new(&ollama_host);

    println!("`prompt`{CSV_DELIMETR}`calls`{CSV_DELIMETR}`time`");
    for model in models {
        println!("`{}`{CSV_DELIMETR}{CSV_DELIMETR}", model.to_string());
        let simple_cases = simple::Simple::default();
        simple_cases.into_iter().for_each(|case| {
            print!("`{}`{CSV_DELIMETR}", case.prompt.clone());
            let start_time = std::time::Instant::now();
            let response = ollama_client.generate(&case.prompt, &model, case.tools.clone());
            match response {
                Ok(response) => {
                    response.iter().for_each(|chunk| {
                        let json = match chunk.message.tool_calls.clone() {
                            Some(calls) => calls.to_string(),
                            None => "-".to_string(),
                        };
                        print!("`{}`", json);
                    }) // println!(chunk.message.tool_calls.));
                }
                Err(err) => eprint!("`Error: {}`", err),
            };
            println!("{CSV_DELIMETR}`{:}`", start_time.elapsed().as_millis());
        });
    }
}

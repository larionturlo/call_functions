pub mod ollama;

use std::fs;

use crate::{cases::simple, statistic};

pub fn run(
    ollama_host: &str,
    models: &[&str],
    cases_path: Option<&str>,
    report_path: Option<&str>,
) -> statistic::Statistics {
    let ollama_client = ollama::Client::new(ollama_host);

    let mut statistics = statistic::Statistics::new();

    for model in models {
        let mut staistic = statistic::Staistic::default();

        let simple_cases =
            simple::Simple::from_file_or_default(cases_path.unwrap_or("simple_cases.json"));
        let mut sum: u128 = 0;

        simple_cases.into_iter().for_each(|case| {
            let mut result = statistic::Responce {
                prompt: case.prompt.clone(),
                ..Default::default()
            };

            let start_time = std::time::Instant::now();
            let response = ollama_client.generate(&case.prompt, model, case.tools.clone());
            match response {
                Ok(response) => response.iter().for_each(|chunk| {
                    let json = match chunk.message.tool_calls.clone() {
                        Some(calls) => calls.to_string(),
                        None => "-".to_string(),
                    };
                    result.calls = json.clone();
                    result.case_is_valid = true;
                    // todo compare expacted call functions
                }),
                Err(err) => {
                    result.calls = err.to_string().clone();
                    result.case_is_valid = false;
                    eprint!("`Error: {}`", err)
                }
            };
            let elapsed = start_time.elapsed().as_millis();
            result.time = if elapsed > 1_000_000u128 {
                1_000_000u128
            } else {
                elapsed
            };
            sum += result.time;

            staistic.responces.push(result);
        });

        let average_time = sum / staistic.responces.len() as u128;
        staistic.average_time = average_time;

        statistics.push(staistic);
    }

    statistics_report_to_file(&statistics, report_path.unwrap_or("statistics.csv"));

    statistics
}

fn statistics_report_to_file(statistics: &statistic::Statistics, file_path: &str) {
    let csv_string = statistics.to_csv_string();

    fs::write(file_path, csv_string).expect("Failed to write statistics to file");
}

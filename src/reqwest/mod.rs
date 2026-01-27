pub mod cases;
pub mod ollama;

use std::{error::Error, fs};

use serde::Serialize;

use crate::reqwest::cases::simple;
const CSV_DELIMETR: &str = "|";

pub type BoxError = Box<dyn Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, BoxError>;

#[derive(Default, Serialize)]
pub struct Responce {
    pub prompt: String,
    pub calls: String,
    pub case_is_valid: bool,
    pub time: u128,
}

impl Responce {
    pub fn to_csv_string(&self) -> String {
        format!(
            "`{}`{CSV_DELIMETR}`{}`{CSV_DELIMETR}`{}`{CSV_DELIMETR}`{}`\n",
            self.prompt, self.calls, self.case_is_valid, self.time
        )
    }
}

#[derive(Default, Serialize)]
pub struct Staistic {
    pub model: String,
    pub responces: Vec<Responce>,
    pub average_time: u128,
}

impl Staistic {
    pub fn to_csv_string(&self) -> String {
        let mut csv_string = format!(
            "`{}`{CSV_DELIMETR}`{}`{CSV_DELIMETR}`{}`{CSV_DELIMETR}`{}`\n",
            self.model,
            self.responces.len(),
            "",
            self.average_time
        );

        for responce in &self.responces {
            csv_string.push_str(&responce.to_csv_string());
        }

        csv_string
    }
}

pub type Statistics = Vec<Staistic>;

pub fn run(
    ollama_host: &str,
    models: &[&str],
    cases_path: Option<&str>,
    report_path: Option<&str>,
) -> Statistics {
    let ollama_client = ollama::Client::new(&ollama_host);

    let mut statistics: Statistics = Vec::new();

    for model in models {
        let mut staistic = Staistic::default();

        let simple_cases =
            simple::Simple::from_file_or_default(cases_path.unwrap_or("simple_cases.json"));
        let mut sum: u128 = 0;

        simple_cases.into_iter().for_each(|case| {
            let mut result = Responce::default();

            result.prompt = case.prompt.clone();

            let start_time = std::time::Instant::now();
            let response = ollama_client.generate(&case.prompt, &model, case.tools.clone());
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

fn statistics_report_to_file(statistics: &Statistics, file_path: &str) {
    let mut csv_string = String::new();
    for staistic in statistics {
        csv_string.push_str(&staistic.to_csv_string());
    }

    fs::write(file_path, csv_string).expect("Failed to write statistics to file");
}

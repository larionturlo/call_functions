use rig::client::CompletionClient;
use rig::providers::ollama;

use crate::cases::extract;
use crate::{BoxError, statistic};

pub const PERSON_FILE: &str = "extract_persons.json";
#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, PartialEq)]
pub struct Person {
    name: Option<String>,
    age: Option<u8>,
    profession: Option<String>,
}

pub async fn run(
    models: &[&str],
    client: ollama::Client,
    report_path: Option<&str>,
) -> Result<(), BoxError> {
    let mut statistics = statistic::Statistics::new();
    for model in models {
        let mut staistic = statistic::Staistic::default();
        let mut sum: u128 = 0;

        let extractor = client.extractor::<Person>(*model).build();
        let persons = extract::load_async::<Person>(PERSON_FILE).await?;

        dbg!(&persons);

        for case in persons {
            let mut result = statistic::Responce::default();
            let start_time = std::time::Instant::now();

            result.prompt = case.prompt.clone();

            let res = extractor.extract(&case.prompt).await;

            if let Ok(person) = res {
                result.case_is_valid = case.expected_data == person;
                result.calls = serde_json::to_string(&person).unwrap();
            } else {
                result.case_is_valid = false;
                result.calls = res.unwrap_err().to_string();
            }

            let elapsed = start_time.elapsed().as_millis();
            result.time = if elapsed > 1_000_000u128 {
                1_000_000u128
            } else {
                elapsed
            };
            sum += result.time;

            staistic.responces.push(result);
        }
        let average_time = sum / staistic.responces.len() as u128;
        staistic.average_time = average_time;

        statistics.push(staistic);
    }

    statistics_report_to_file_async(&statistics, report_path.unwrap_or("extract_statistics.csv"))
        .await?;

    Ok(())
}

async fn statistics_report_to_file_async(
    statistics: &statistic::Statistics,
    file_path: &str,
) -> Result<(), BoxError> {
    let csv_string = statistics.to_csv_string();

    tokio::fs::write(file_path, csv_string)
        .await
        .map_err(|err| err.into())
}

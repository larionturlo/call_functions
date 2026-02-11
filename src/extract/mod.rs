use rig::client::CompletionClient;
use rig::providers::ollama;

use crate::cases::extract;
use crate::{BoxError, statistic};

pub mod company;
pub mod person;

pub async fn run<T>(
    cases_file_path: &str,
    models: &[&str],
    client: ollama::Client,
    report_path: &str,
) -> Result<(), BoxError>
where
    T: for<'a> serde::Deserialize<'a>
        + serde::Serialize
        + schemars::JsonSchema
        + std::fmt::Debug
        + PartialEq
        + Clone
        + Send
        + Sync
        + 'static,
{
    let mut statistics = statistic::Statistics::new();
    let cases = extract::load_async::<T>(cases_file_path).await?;
    for model in models {
        let mut staistic = statistic::Staistic {
            model: model.to_string(),
            ..Default::default()
        };
        let mut sum: u128 = 0;

        let extractor = client.extractor::<T>(*model).build();

        for case in &cases {
            let mut result = statistic::Responce::default();
            let start_time = std::time::Instant::now();

            result.prompt = case.prompt.clone();

            let res = extractor.extract(&case.prompt).await;

            if let Ok(data) = res {
                result.case_is_valid = case.expected_data == data;
                result.calls = serde_json::to_string(&data).unwrap();
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

    statistics_report_to_file_async(&statistics, report_path).await?;

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

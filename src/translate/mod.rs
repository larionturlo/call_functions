use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::ollama;

use crate::cases::translate::{self, Translate};
use crate::{BoxError, statistic};

pub const TRANSLATE_FILE: &str = "translate_examples.json";

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
pub struct TranslationPair {
    pub source_lang: String,
    pub source_lang_code: String,
    pub target_lang: String,
    pub target_lang_code: String,
    pub reference: Option<Referens>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
pub struct Referens {
    pub source_text: String,
    pub target_text: String,
}

pub async fn run(
    models: &[&str],
    client: ollama::Client,
    report_path: Option<&str>,
) -> Result<(), BoxError> {
    let report_path = report_path.unwrap_or("data/translate/translate_statistics.csv");

    let translations = translate::load_async::<TranslationPair>(TRANSLATE_FILE).await?;

    let mut statistics = statistic::Statistics::new();
    for model in models {
        let mut staistic = statistic::Staistic {
            model: model.to_string(),
            ..Default::default()
        };
        let mut sum: u128 = 0;

        let agent = client.agent(model.to_string()).build();

        for case in &translations {
            let mut result = statistic::Responce::default();
            let start_time = std::time::Instant::now();

            let prompt = generate_prompt(case);

            result.prompt = case.source.clone();

            let res = agent.prompt(prompt).await;

            if let Ok(completion) = res {
                let translated_text = completion.trim().to_string();

                // Check if we have a reference translation to compare against
                if let Some(ref reference) = case.meta.reference {
                    // For now, we'll do a simple string comparison
                    // In a more sophisticated implementation, you might want to use semantic similarity
                    // or other metrics to evaluate translation quality
                    result.case_is_valid =
                        translated_text.to_lowercase() == reference.target_text.to_lowercase();
                } else {
                    // If no reference is available, consider it valid (though this is not ideal)
                    result.case_is_valid = true;
                }

                result.calls = translated_text;
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

    // Create directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(file_path).parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    tokio::fs::write(file_path, csv_string)
        .await
        .map_err(|err| err.into())
}

fn generate_prompt(t: &Translate<TranslationPair>) -> String {
    let source_lang = t.meta.source_lang.clone();
    let source_code = t.meta.source_lang_code.clone();
    let target_lang = t.meta.target_lang.clone();
    let target_code = t.meta.target_lang_code.clone();

    let mut prompt = format!(
        "You are a professional {source_lang} ({source_code}) to {target_lang} ({target_code}) translator. Your goal is to accurately convey the meaning and nuances of the original {source_lang} text while adhering to {target_lang} grammar, vocabulary, and cultural sensitivities.
        Produce only the {target_lang} translation, without any additional explanations or commentary. \n\n"
    );

    if let Some(reference) = &t.meta.reference {
        prompt.push_str(
            &format!(
                "Example: \n\n ( {source_lang} ({source_code}): \n {}, \n {target_lang} ({target_code}: \n {} )\n\n", reference.source_text,
                reference.target_text
            )
        );
    }

    prompt.push_str(&format!(
        "Please translate the following {source_lang} text into {target_lang}: \n\n {} \n\n",
        t.source
    ));

    prompt
}

use rig::providers::ollama;

use crate::BoxError;
use crate::extract;

pub const PERSON_FILE: &str = "extract_persons.json";

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
pub enum Profession {
    #[serde(rename = "doctor")]
    Doctor,
    #[serde(rename = "software engineer")]
    SoftwareEngineer,
    #[serde(rename = "teacher")]
    Teacher,
    #[serde(rename = "chef")]
    Chef,
    #[serde(rename = "nurse")]
    Nurse,
    #[serde(rename = "architect")]
    Architect,
    #[serde(rename = "marketing manager")]
    MarketingManager,
    #[serde(rename = "carpenter")]
    Carpenter,
    #[serde(rename = "graphic designer")]
    GraphicDesigner,
    #[serde(rename = "accountant")]
    Accountant,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
pub struct Person {
    name: Option<String>,
    age: Option<u8>,
    profession: Option<Profession>,
}

pub async fn run(
    models: &[&str],
    client: ollama::Client,
    report_path: Option<&str>,
) -> Result<(), BoxError> {
    let report_path = report_path.unwrap_or("data/extract/person_statistics.csv");
    extract::run::<Person>(PERSON_FILE, models, client, report_path).await
}

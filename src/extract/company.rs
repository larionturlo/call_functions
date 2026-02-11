use rig::providers::ollama;

use crate::BoxError;
use crate::extract;

pub const COMPANY_FILE: &str = "extract_companies.json";

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
pub struct Address {
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    country: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
pub struct Employee {
    name: Option<String>,
    position: Option<String>,
    department: Option<String>,
    salary: Option<f64>,
    contact_info: Option<ContactInfo>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
pub struct ContactInfo {
    email: Option<String>,
    phone: Option<String>,
    linkedin: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
pub struct Company {
    name: Option<String>,
    industry: Option<String>,
    founded_year: Option<u16>,
    // headquarters: Option<Address>,
    employees_count: Option<u32>,
    ceo: Option<String>,
    revenue: Option<f64>,
    // employees: Option<Vec<Employee>>,
}

pub async fn run(
    models: &[&str],
    client: ollama::Client,
    report_path: Option<&str>,
) -> Result<(), BoxError> {
    let report_path = report_path.unwrap_or("data/extract/company_statistics.csv");
    extract::run::<Company>(COMPANY_FILE, models, client, report_path).await
}

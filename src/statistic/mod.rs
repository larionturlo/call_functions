use std::fs;

use serde::Serialize;

// todo refactoring of statistic collection

const CSV_DELIMETR: &str = "|";

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

#[derive(Default, Serialize)]
pub struct Statistics(Vec<Staistic>);

impl Statistics {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, statistic: Staistic) {
        self.0.push(statistic);
    }

    pub fn to_csv_string(&self) -> String {
        let mut csv_string = String::new();

        for statistic in &self.0 {
            csv_string.push_str(&statistic.to_csv_string());
        }

        csv_string
    }

    pub fn report_to_file(&self, file_path: &str) {
        let csv_string = self.to_csv_string();

        fs::write(file_path, csv_string).expect("Failed to write statistics to file");
    }

    pub async fn report_to_file_async(&self, file_path: &str) {
        let csv_string = self.to_csv_string();

        tokio::fs::write(file_path, csv_string)
            .await
            .expect("Failed to write statistics to file");
    }
}

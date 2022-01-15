use std::fmt::{Display, Formatter};
use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
enum Category {
    #[serde(rename(deserialize = "Mat"))]
    Food,
    #[serde(rename(deserialize = "Overføring"))]
    Transfer,
    #[serde(rename(deserialize = "Gave"))]
    Gift,
    #[serde(rename(deserialize = "Oppgjør"))]
    Settlement,
    #[serde(rename(deserialize = "Fondssparing"))]
    FundSaving,
    #[serde(rename(deserialize = "Strøm"))]
    Power,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DnbTransactionCsv {
    #[serde(rename(deserialize = "Dato"))]
    posting_date: String,
    #[serde(rename(deserialize = "Rentedato"))]
    interest_date: String,
    #[serde(rename(deserialize = "Forklaring"))]
    description: String,
    #[serde(rename(deserialize = "Ut fra konto"))]
    withdraw: String,
    #[serde(rename(deserialize = "Inn på konto"))]
    deposit: String,
    #[serde(rename(deserialize = "Kategori"))]
    category: Category
}

impl Display for DnbTransactionCsv {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {:?}: (-{}, {}) {}", self.posting_date, self.category, self.withdraw, self.deposit, self.description)
    }
}

pub fn parse_csv(filename: &str) -> Result<Vec<DnbTransactionCsv>, csv::Error> {

    let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_path(filename)?;
    let mut result = vec![];

    for transaction in reader.deserialize() {
        result.push(transaction?);
    }

    Ok(result)
}

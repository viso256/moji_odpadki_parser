use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Month parsing failed! Found string: '{0}'")]
    MonthName(String),
    #[error("Day of the week parsing failed! Found string: '{0}'")]
    DIAWName(String),
    #[error("Failed to parse {item}. Dind't find '{expr}'.")]
    HTMLParsing {
        item: &'static str,
        expr: &'static str,
    },
    #[error("Failed to parse {item}.")]
    Generic {
        item: &'static str,
    },
}
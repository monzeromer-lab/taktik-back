use serde::Serialize;
use actix_web::http::StatusCode;

#[derive(Debug, Serialize)]
pub struct TakTikResponse<Data> {
    pub error: bool,

    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,

    pub result: ResResult<Data>,
}

#[derive(Debug, Serialize)]
pub enum ResResult<MODEL> {
    Data(MODEL),
    Error(String),
}
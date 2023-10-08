use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TakTikResponse<Data> {
    pub error: bool,

    pub status: i32,

    pub data: ResResult<Data>,
}

#[derive(Debug, Serialize)]
pub enum ResResult<MODEL> {
    DATA(MODEL),
    ERROR(String),
}
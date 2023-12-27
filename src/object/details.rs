use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Details {
    pub error: Option<DetailsErrorType>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub enum DetailsErrorType {
    DeviceNotRegistered,
    InvalidCredentials,
    MessageTooBig,
    MessageRateExceeded,
}

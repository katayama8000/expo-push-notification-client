use std::collections::HashMap;

use serde::Deserialize;

use super::{
    expo_push_error_receipt::ExpoPushErrorReceipt,
    expo_push_success_receipt::ExpoPushSuccessReceipt,
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ExpoPushReceipt {
    Success(HashMap<String, ExpoPushSuccessReceipt>),
    Error(Vec<ExpoPushErrorReceipt>),
}

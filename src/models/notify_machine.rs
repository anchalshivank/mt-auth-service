use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NotifyMachineRequest {
    pub machine_id: String,
    pub user_id: String,
    pub message: String,
}

pub enum NotifyMachineResponse {

    Success,
    Failure,
    Error(String)

}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotifyMachineErrorData {

    pub(crate) code: String,
    pub(crate) message: String

}


use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeployMachineRequest {
    pub machine_id: i32,
}

#[derive(Serialize)]
pub enum MachineResponse {
    DeploySuccess,
    RegisterSuccess,
    TakenForMaintenanceSuccess,
    Error(String)
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MaintenanceRequest {
    pub machine_id: i32,
    pub maintenance_notes: Option<String>,
}


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterMachineRequest {
    pub next_service: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterMachineResponse {
    pub success: bool,
    pub message: String,
}

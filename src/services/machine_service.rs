use std::sync::{Arc, LockResult, Mutex};
use diesel::result::Error;
use crate::models::machine::{
    DeployMachineRequest, MachineResponse,
    MaintenanceRequest,
    RegisterMachineRequest, RegisterMachineResponse
};
use crate::repositories::machine_repository::MachineRepository;
use ntex::web;
use log::info;
use ntex::http::error::BlockingError;

#[derive(Clone)]
pub struct MachineService {
    pub repository: Arc<Mutex<MachineRepository>>,
}

impl MachineService {

    pub fn new(repository: Arc<Mutex<MachineRepository>>) -> MachineService {
        MachineService { repository }
    }

    pub async fn deploy_machine(&self, req: DeployMachineRequest) -> Result<MachineResponse, std::io::Error> {
        info!("Service deploy_machine");
        match self.repository.lock() {
            Ok(repository) => {
                match repository.deploy(req).await {
                    Ok(_) => Ok(MachineResponse::DeploySuccess),
                    Err(error) => Ok(MachineResponse::Error(format!("Failed to deploy machine: {}", error))),
                }
            }
            Err(error) => Ok(MachineResponse::Error(format!("Failed to acquire repository lock: {}", error))),
        }
    }

    pub async fn register_machine(&self, req: RegisterMachineRequest) -> Result<MachineResponse, std::io::Error> {
        info!("Service register_machine");
        match self.repository.lock() {
            Ok(repository) => {
                match repository.register(req).await {
                    Ok(_) => Ok(MachineResponse::RegisterSuccess),
                    Err(error) => Ok(MachineResponse::Error(format!("Failed to register machine: {}", error))),
                }
            }
            Err(error) => Ok(MachineResponse::Error(format!("Failed to acquire repository lock: {}", error))),
        }
    }

    pub async fn take_machine_for_maintenance(&self, req: MaintenanceRequest) -> Result<MachineResponse, std::io::Error> {
        info!("Service take_machine_for_maintenance");
        match self.repository.lock() {
            Ok(repository) => {
                match repository.take_for_maintenance(req).await {
                    Ok(_) => Ok(MachineResponse::TakenForMaintenanceSuccess),
                    Err(error) => Ok(MachineResponse::Error(format!("Failed to take machine for maintenance: {}", error))),
                }
            }
            Err(error) => Ok(MachineResponse::Error(format!("Failed to acquire repository lock: {}", error))),
        }
    }
}

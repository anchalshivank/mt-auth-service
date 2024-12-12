use crate::controllers::response::ApiResponse;
use crate::models::machine::{
    DeployMachineRequest, MachineResponse, MaintenanceRequest,
};
use crate::services::Services;
use ntex::web;
use ntex::web::types::Json;
use ntex::web::{resource, HttpResponse, Responder};
use std::io::Error;
use std::sync::{Arc, Mutex};

#[web::post("/deploy")]
pub async fn deploy_machine(
    services: web::types::State<Arc<Mutex<Services>>>,
    req: Json<DeployMachineRequest>,
) -> impl Responder {
    let services = services.lock().unwrap();
    let service = services.machine_service.lock().unwrap();

    match service.deploy_machine(req.into_inner()).await {
        Ok(response) => match response {
            MachineResponse::DeploySuccess => {
                HttpResponse::Ok().json(&ApiResponse::<MachineResponse, ()>::success(
                    "Machine deployed successfully",
                    Some(MachineResponse::DeploySuccess),
                ))
            }
            MachineResponse::RegisterSuccess => {
                HttpResponse::Ok().json(&ApiResponse::<MachineResponse, ()>::success(
                    "Machine registered successfully",
                    Some(MachineResponse::RegisterSuccess),
                ))
            }
            MachineResponse::TakenForMaintenanceSuccess => {
                HttpResponse::Ok().json(&ApiResponse::<MachineResponse, ()>::success(
                    "Machine taken for maintenance successfully",
                    Some(MachineResponse::TakenForMaintenanceSuccess),
                ))
            }
            MachineResponse::Error(err_msg) => {
                HttpResponse::BadRequest().json(&ApiResponse::<(), String>::error(
                    "Operation failed",
                    err_msg,
                ))
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(&ApiResponse::<(), String>::error(
            "Failed to process the request",
            err.to_string(),
        )),
    }
}

#[web::post("/register")]
pub async fn register_machine(
    services: web::types::State<Arc<Mutex<Services>>>
) -> impl Responder {
    let services = services.lock().unwrap();
    let service = services.machine_service.lock().unwrap();

    match service.register_machine().await {
        Ok(response)=> match response {
            MachineResponse::RegisterSuccess => {
                HttpResponse::Ok().json(&ApiResponse::<MachineResponse, ()>::success(
                    "Machine registered successfully",
                Some(response),
                ))
            }
            MachineResponse::Error(err_msg) => {
                HttpResponse::BadRequest().json(&ApiResponse::<(), String>::error(
                    "Operation failed",
                    err_msg,
                ))
            }
            _ => {
                HttpResponse::BadRequest().json(&ApiResponse::<(), String>::error(
                    "Unknown Error",
                    "Improper response".parse().unwrap(),
                ))
            }
        }
        // Ok(response) =>
        //     HttpResponse::Ok().json(&ApiResponse::<MachineResponse, ()>::success(
        //     "Machine registered successfully",
        //     Some(response),
        // )),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to register machine: {}", err))
        }
    }
}

#[web::post("/maintenance")]
pub async fn take_machine_for_maintenance(
    services: web::types::State<Arc<Mutex<Services>>>,
    req: Json<MaintenanceRequest>,
) -> impl Responder {
    let services = services.lock().unwrap();
    let service = services.machine_service.lock().unwrap();

    match service.take_machine_for_maintenance(req.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(&ApiResponse::<MachineResponse, ()>::success(
            "Machine taken for maintenance successfully",
            Some(response),
        )),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to take machine for maintenance: {}", err)),
    }
}

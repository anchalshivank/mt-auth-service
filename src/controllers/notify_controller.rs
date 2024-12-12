use crate::controllers::response::ApiResponse;
use crate::models::notify_machine::{
    NotifyMachineErrorData, NotifyMachineRequest, NotifyMachineResponse,
};
use crate::services::Services;
use log::info;
use ntex::web;
use ntex::web::types::Json;
use ntex::web::{HttpResponse, Responder};
use std::sync::{Arc, Mutex};

#[web::post("/auth-qr")]
pub async fn handle_qr_code(
    services: web::types::State<Arc<Mutex<Services>>>,
    req: Json<NotifyMachineRequest>,
) -> impl Responder {
    info!("Handling QR code authentication");

    let services = services.lock().unwrap();
    let service = services.notify_service.lock().unwrap();

    // Attempt to notify the machine
    match service.notify_machine(req.clone()).await {
        Ok(NotifyMachineResponse::Success) => {
            info!("Machine notification successful");
            HttpResponse::Ok().json(&ApiResponse::<(), ()>::success(
                "Notification sent successfully",
                None,
            ))
        }
        Ok(NotifyMachineResponse::Failure) => {
            let error_data = NotifyMachineErrorData {
                code: "NOTIFICATION_FAILED".to_string(),
                message: "Notification could not be delivered to the machine.".to_string(),
            };
            HttpResponse::BadRequest().json(&ApiResponse::<(), NotifyMachineErrorData>::error(
                "Notification failed",
                error_data,
            ))
        }
        Ok(NotifyMachineResponse::Error(err)) => {
            let error_data = NotifyMachineErrorData {
                code: "NOTIFICATION_ERROR".to_string(),
                message: format!("An unexpected error occurred: {}", err),
            };
            HttpResponse::InternalServerError().json(
                &ApiResponse::<(), NotifyMachineErrorData>::error(
                    "Notification failed",
                    error_data,
                ),
            )
        }
        Err(err) => {
            let error_data = NotifyMachineErrorData {
                code: "NOTIFICATION_EXCEPTION".to_string(),
                message: format!("An exception occurred: {}", err),
            };
            HttpResponse::InternalServerError().json(
                &ApiResponse::<(), NotifyMachineErrorData>::error(
                    "Notification failed",
                    error_data,
                ),
            )
        }
    }
}

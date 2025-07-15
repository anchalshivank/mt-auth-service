use crate::models::notify_machine::{NotifyMachineRequest, NotifyMachineResponse};
use log::info;
use reqwest::Client;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Clone)]
pub struct NotifyService {
    addr: String,
    client: Arc<Client>,
}

impl NotifyService {
    pub fn new(addr: String) -> Self {
        info!("Notify addr is {:?}",addr );
        Self {
            addr,
            client: Arc::new(Client::new()),
        }
    }

    pub async fn notify_machine(
        &self,
        qr: NotifyMachineRequest,
    ) -> Result<NotifyMachineResponse, String> {
        // Construct the URL for the endpoint
        // let url = format!("http://{}/notify-machine", self.addr);
        let url = self.addr.to_string();


        // Prepare the request payload
        let payload = NotifyMachineRequest {
            machine_id: qr.machine_id.clone(),
            user_id: qr.user_id.clone(),
            message: qr.message.clone(),
        };


        // Send the POST request
        let response = self
            .client
            .post(&url)
            .json(&payload) // Serialize the payload to JSON
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => Ok(NotifyMachineResponse::Success),
            Ok(resp) => {
                let error_message = resp
                    .text()
                    .await
                    .unwrap_or_else(|err| err.to_string());
                Ok(NotifyMachineResponse::Error(error_message))
            }
            Err(err) => Err(format!(
                "Error occurred while sending notification: {:?}",
                err
            )),
        }
    }
}

use std::sync::{Arc, Mutex};

pub mod user_service;
pub mod notify_service;
pub mod machine_service;

pub struct Services{
    pub user_service: Arc<Mutex<user_service::UserService>>,
    pub notify_service: Arc<Mutex<notify_service::NotifyService>>,
    pub machine_service: Arc<Mutex<machine_service::MachineService>>
}
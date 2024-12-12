use crate::database::DbPool;
use crate::models::machine::{DeployMachineRequest, MaintenanceRequest, RegisterMachineRequest};
use diesel::result::Error;
use diesel::{sql_query, QueryableByName, RunQueryDsl, Selectable};
use ntex::http::error::BlockingError;
use ntex::web;
use std::sync::{Arc, Mutex};
use log::info;
use crate::models::CountQueryResult;

#[derive(Clone)]
pub struct MachineRepository {
    pub pool: Arc<Mutex<DbPool>>
}




impl MachineRepository {
    pub fn new(pool: Arc<Mutex<DbPool>>) -> MachineRepository {
        Self { pool }
    }

    pub async fn deploy(&self, req: DeployMachineRequest) -> Result<usize, Error> {
        let db = self.pool.lock().map_err(|_| Error::RollbackTransaction)?;
        let mut conn = db.get().map_err(|_| Error::RollbackTransaction)?;

        // Ensure machine is registered and not under maintenance
        let exists = sql_query("SELECT COUNT(*) FROM machines WHERE id = $1 AND under_maintenance = TRUE")
            .bind::<diesel::sql_types::Integer, _>(req.machine_id)
            .get_result::<CountQueryResult>(&mut conn)?;

        info!("Exists : {}", exists.count);

        if exists.count == 0 {
            return Err(Error::RollbackTransaction);
        }

        // Update machine to be deployed
        sql_query("UPDATE machines SET under_maintenance = FALSE WHERE id = $1")
            .bind::<diesel::sql_types::Integer, _>(req.machine_id)
            .execute(&mut conn)
    }

    pub async fn take_for_maintenance(&self, req: MaintenanceRequest) -> Result<usize, Error> {
        let db = self.pool.lock().map_err(|_| Error::RollbackTransaction)?;
        let mut conn = db.get().map_err(|_| Error::RollbackTransaction)?;

        // Ensure machine is not already under maintenance
        let not_under_maintenance = sql_query("SELECT COUNT(*) FROM machines WHERE id = $1 AND under_maintenance = FALSE")
            .bind::<diesel::sql_types::Integer, _>(req.machine_id)
            .get_result::<CountQueryResult>(&mut conn)?;

        if not_under_maintenance.count == 0 {
            return Err(Error::RollbackTransaction);
        }

        // Mark machine as under maintenance
        sql_query("UPDATE machines SET under_maintenance = TRUE WHERE id = $1")
            .bind::<diesel::sql_types::Integer, _>(req.machine_id)
            .execute(&mut conn)?;

        // Insert into service history
        sql_query("INSERT INTO service_history (machine_id, service_notes) VALUES ($1, $2)")
            .bind::<diesel::sql_types::Integer, _>(req.machine_id)
            .bind::<diesel::sql_types::Text, _>(req.maintenance_notes.unwrap_or_default())
            .execute(&mut conn)
    }


    pub async fn register(&self, req: RegisterMachineRequest) -> Result<usize, BlockingError<Error>> {
        match self.pool.lock() {
            Ok(db) => {
                let db = db.clone();
                let result = web::block(move || {
                    let mut conn = db.get().unwrap();
                    sql_query("INSERT INTO machines (under_maintenance, next_service) VALUES (FALSE, $1)")
                        .bind::<diesel::sql_types::Text, _>(req.next_service)
                        .execute(&mut conn)
                })
                    .await;
                result
            }
            Err(_) => Err(BlockingError::Canceled),
        }
    }

}

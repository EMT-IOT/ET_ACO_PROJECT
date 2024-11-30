use actix_web::{get,Responder, web::{self,Data}, HttpResponse};

use crate::db::db_operations::DB;
use crate::models::device::Device;
use crate::db::db_operations::DeviceError;

#[get("/device/{id}")]
pub async fn create_device(db:Data<DB>,path:web::Path<String>)-> impl Responder {

    let device_id=path.into_inner();

    let device=Device::new(device_id);

    let res=db.insert_device(device).await;

    match res {
       Ok(_) => HttpResponse::Ok().body("device created successfully"),
       Err( DeviceError::DeviceExists) => HttpResponse::Conflict().body("device already exists"),
       Err(_)=> HttpResponse::InternalServerError().body("database erorr"),
    }
}   
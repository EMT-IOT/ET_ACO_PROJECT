mod models;
mod controllers;
mod db;
mod mqtt;
mod hashmap;

use actix_web::{web,HttpServer,App};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use crate::db::db_operations::DB;
use crate::mqtt::mqtt_client::MqttService;
use crate::hashmap::map_storage::HaspMapService;

use crate::controllers::create_device::create_device;
use crate::controllers::update_device_parameters::update_device_parameters;
// use crate::controllers::get_devices::get_devices;


#[actix_web::main]
async fn main()->std::io::Result<()> {
    
    dotenv().ok();

    let host=env::var("HTTP_HOST").expect("failed to read HOST env");
    let port=env::var("HTTP_PORT").expect("failed to read the PORT env").parse().expect("failed to parse PORT env");

    let db_service=DB::connect_to_db().await.unwrap();

     let hash_map_service=HaspMapService::init();

     let hash_map_service_ref=Arc::clone(&hash_map_service.map);

   let mqtt_service=MqttService::init(hash_map_service_ref).await.expect("failed while initializing MQTT service");

    let server=HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_service.clone()))
            .app_data(web::Data::new(mqtt_service.clone()))
            .app_data(web::Data::new(hash_map_service.clone()))
            .service(create_device)
            .service(update_device_parameters)
            // .service(get_devices)
    }).bind((host.clone(),port))?;

    println!("server is running at http://{}:{}",host,port);

    server.run().await

    
}

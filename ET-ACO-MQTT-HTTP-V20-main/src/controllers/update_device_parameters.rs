
use actix_web::{post, web::{self,Data}, HttpResponse, Responder};
use serde_json::Value;
use rumqttc::QoS;
use std::time::Duration;

use crate::mqtt::mqtt_client::MqttService;
use crate::hashmap::map_storage::HaspMapService;

#[post("/device/update/{id}")]
async fn update_device_parameters(mqtt_service:Data<MqttService>,hash_map_service:Data<HaspMapService>,path:web::Path<String>,request_body:String) -> impl Responder{

    let device_id=path.into_inner();
    
    let is_valid_json=serde_json::from_str::<Value>(&request_body).is_ok();

    if is_valid_json {
        
        let publish_topic=format!("{}/RX",device_id.clone());

        mqtt_service.mqtt_client.publish(publish_topic, QoS::AtLeastOnce, false, request_body).await.unwrap();

        let mut time_out_counter=10;
                           
         while time_out_counter > 0 {

            if let Ok(mut map) = hash_map_service.map.lock() {
                if map.get(&device_id).is_some() {
                    map.remove(&device_id);
                        return HttpResponse::Ok().body("device parameters updated sucessfully");
                }
            }

            actix_web::rt::time::sleep(Duration::from_secs(1)).await;
            println!("checking for the device response {}",time_out_counter);

                time_out_counter-=1;
            }

            return HttpResponse::RequestTimeout().body("failed to get the device acknowledgement");

    }else{
        return HttpResponse::BadRequest().body("invalid json request body");
    }


   
}
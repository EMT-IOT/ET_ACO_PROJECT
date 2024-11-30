
use rumqttc::{ AsyncClient, ClientError, Event, Incoming, MqttOptions,QoS};
use std::env;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json::Value;
// use std::time::{Instant,Duration};

#[derive(Clone)]
pub struct MqttService {
    pub mqtt_client:AsyncClient,
}

impl MqttService {
    pub async fn init(hash_map_service: Arc<Mutex<HashMap<String,String>>>) -> Result<Self,ClientError> {
        let client_id=env::var("MQTT_CLIENT_ID").expect("failed to read the MQTT_CLIENT_ID environment variable");
        let host=env::var("MQTT_HOST").expect("failed to read MQTT_HOST env variable");
        let port=env::var("MQTT_PORT").expect("failed to read the MQTT_PORT env variable");

        let port:u16=port.trim().parse().expect("failed to parse the MQTT_PORT env variable");

        let mut mqtt_options=MqttOptions::new(client_id, host, port);
        mqtt_options.set_keep_alive(std::time::Duration::from_secs(10));

        let (mqtt_client,mut event_loop)=AsyncClient::new(mqtt_options, 10);

        mqtt_client.subscribe("+/TX", QoS::AtLeastOnce).await?;


        actix_web::rt::spawn(async move {
            loop {

                let hash_map_service =Arc::clone(&hash_map_service);

                if let Ok(event)=event_loop.poll().await {
                    match event {
                        Event::Incoming(Incoming::ConnAck(_)) => {
                            println!("connected to mqtt broker");
                        },
                        Event::Incoming(Incoming::Publish(publish))=>{
                            let payload=publish.payload.to_vec();
                            
                            if let Ok(payload_str)=String::from_utf8(payload) {
                                if let Ok(message)=serde_json::from_str::<Value>(&payload_str) {
                                    if message.get("RES").is_some() {
                                        let device_id=message.get("ID").and_then(|v| v.as_str()).unwrap_or("0").to_string();

                                        if let Ok(mut map) = hash_map_service.lock() {
                                            map.insert(device_id.clone(), device_id.clone());
                                        }
                                        
                                        let map_clear_timeout=10;
                                        actix_web::rt::spawn( async move {
                                            actix_web::rt::time::sleep(std::time::Duration::from_secs(map_clear_timeout)).await;
                                            if let Ok(mut map) =hash_map_service.lock() {
                                                map.remove(&device_id);
                                            }
                                        });
                                        
                                    }
                                }
                            }
                        },
                        Event::Incoming(Incoming::Disconnect) => {
                            println!("disconnected from the mqtt broker");
                        },
                        _ => continue
                    }
                }
            }
        });

        
        Ok(Self {
            mqtt_client,
        })
    }


}
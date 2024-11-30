mod db;
mod models;
mod mqtt;

use dotenv::dotenv;
use rumqttc::{Event, Incoming};
use std::sync::Arc;

use crate::db::db_operations::DB;
use crate::mqtt::mqtt_operations::MqttService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db = DB::connect_to_db().await?;
    let mqtt_service = MqttService::connect().await?;

    let (client, mut event_loop) = mqtt_service.mqtt_cleint;


    let db = Arc::new(db);
    let mqtt_client = Arc::new(client);
    loop {
        if let Ok(event) = event_loop.poll().await {
            match event {
                Event::Incoming(Incoming::ConnAck(_)) => {
                    println!("conected to broker")
                }
                Event::Incoming(Incoming::Publish(publish)) => {
                    let payload = publish.payload.to_vec();
                    let topic = publish.topic.clone();

                    let db = Arc::clone(&db);
                    let mqtt_client = Arc::clone(&mqtt_client);

                    tokio::task::spawn(async move {
                        let _ = MqttService::message_handler(
                            topic,
                            payload,
                            db,
                            mqtt_client,
                        )
                        .await;
                    });
                }
                Event::Incoming(Incoming::Disconnect) => {
                    println!("disconnected from the broker")
                }
                _ => (),
            }
        }
    }
}

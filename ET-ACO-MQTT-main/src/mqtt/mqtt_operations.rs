use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use crate::db::db_operations::DB;
use crate::models::manifold_refills::ManifoldRefillReq;
use crate::models::manifold_shifts::ManifoldShiftReq;
use crate::models::sensor_alerts::{SensorAlertReq, SensorAlerts};
use crate::models::status_updates::StatusUpdateReq;

use crate::models::manifold_refills::ManifoldRefills;
use crate::models::manifold_shifts::ManifoldShifts;
use crate::models::status_updates::StatusUpdates;

pub struct MqttService {
    pub mqtt_cleint: (AsyncClient, EventLoop),
}

impl MqttService {
    pub async fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let host = env::var("HOST").expect("failed to read the HOST env");
        let port = env::var("PORT").expect("failed to PORT env");

        let port = port.trim().parse().expect("failed to parse the PORT env");

        let mut mqtt_options = MqttOptions::new("server", host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(10));

        let mqtt_cleint = AsyncClient::new(mqtt_options, 10);

        mqtt_cleint.0.subscribe("+/TX", QoS::AtLeastOnce).await?;

        return Ok(Self { mqtt_cleint });
    }

    pub async fn message_handler(
        _topic: String,
        payload: Vec<u8>,
        db: Arc<DB>,
        client: Arc<AsyncClient>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload_str = String::from_utf8(payload)?;
        let payload_value = serde_json::from_str::<Value>(payload_str.as_str())?;

        if payload_value.get("PRV").is_some() {
            let id = payload_value
                .get("ID")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let sg = payload_value
                .get("SG")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let prv = payload_value
                .get("PRV")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let mt = payload_value
                .get("MT")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let bn = payload_value
                .get("BN")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let mf1_st = payload_value
                .get("MF1_ST")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let mf2_st = payload_value
                .get("MF2_ST")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let ws = payload_value
                .get("WS")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let mf_shd = payload_value
                .get("MF_SHD")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let sms = payload_value
                .get("SMS")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let to = payload_value
                .get("TO")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let mfr = payload_value
                .get("MFR")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();
            let gld = payload_value
                .get("GLD")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();
            let aco = payload_value
                .get("ACO")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();
            let hi = payload_value
                .get("HI")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();
            let lw = payload_value
                .get("LW")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();
            let ht = payload_value
                .get("HT")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();
            let sen = payload_value
                .get("SEN")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();
            let stu = payload_value
                .get("STU")
                .and_then(|v| v.as_str())
                .unwrap_or("0")
                .to_string();

            let device_past_data = vec![
                ("sg", sg),
                ("prv", prv),
                ("mt", mt),
                ("bn", bn),
                ("mf1_st", mf1_st),
                ("mf2_st", mf2_st),
                ("ws", ws),
                ("mf_shd", mf_shd),
                ("sms", sms),
                ("to", to),
                ("mfr", mfr),
                ("gld", gld),
                ("aco", aco),
                ("hi", hi),
                ("lw", lw),
                ("ht", ht),
                ("sen", sen),
                ("stu", stu),
            ];

            let device_changed_parameters_vec = db
                .find_device_changed_parameters(id.clone(), device_past_data)
                .await?;
            let device_changed_parameters_map: HashMap<String, String> =
                device_changed_parameters_vec.into_iter().collect();

            let publish_topic = format!("{}/RX", id);
            let publish_payload = serde_json::to_string(&device_changed_parameters_map)?;

            client
                .publish(publish_topic, QoS::AtLeastOnce, false, publish_payload)
                .await?;
        } else if payload_value.get("WT").is_some() {
            let status_update_req = serde_json::from_str::<StatusUpdateReq>(payload_str.as_str())?;

            let status_update = StatusUpdates::new(status_update_req);

            db.insert_status_update(status_update).await?;
        } else if payload_value.get("MF_ON").is_some() {
            let manifold_shift_req =
                serde_json::from_str::<ManifoldShiftReq>(payload_str.as_str())?;

            let manifold_shifts = ManifoldShifts::new(manifold_shift_req);

            db.insert_manifold_shifts(manifold_shifts).await?;
        } else if payload_value.get("LVL").is_some() {
            let sensor_alert_req = serde_json::from_str::<SensorAlertReq>(payload_str.as_str())?;
            let sensor_alert = SensorAlerts::new(sensor_alert_req);
            db.insert_sensor_alert(sensor_alert).await?;
        } else {
            let manifold_refill_req =
                serde_json::from_str::<ManifoldRefillReq>(&payload_str.as_str())?;

            let manifold_refills = ManifoldRefills::new(manifold_refill_req);

            db.insert_manifold_refills(manifold_refills).await?;
        }

        return Ok(());
    }
}

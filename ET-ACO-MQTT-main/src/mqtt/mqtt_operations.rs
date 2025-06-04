use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use crate::db::db_operations::DB;
use crate::models::device_settings::{DeviceSettings, DeviceSettingsRequest};
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
        _client: Arc<AsyncClient>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload_str = String::from_utf8(payload)?;
        let payload_value = serde_json::from_str::<Value>(payload_str.as_str())?;

        if payload_value.get("PRV").is_some() {
            let device_settings_req = serde_json::from_str::<DeviceSettingsRequest>(payload_str.as_str())?;

            let device_settings = DeviceSettings::new(device_settings_req);

            db.insert_device_settings(device_settings).await?;

        } else if payload_value.get("WT").is_some() {

            let status_update_req = serde_json::from_str::<StatusUpdateReq>(payload_str.as_str())?;

            let status_update = StatusUpdates::new(status_update_req);

            db.insert_status_update(status_update).await?;
        } else if (payload_value.get("MF_ON").is_some()) || (payload_value.get("MF").is_some()) {
            let manifold_shift_req =
                serde_json::from_str::<ManifoldShiftReq>(payload_str.as_str())?;

            let manifold_shifts = ManifoldShifts::new(manifold_shift_req);

            db.insert_manifold_shifts(manifold_shifts).await?;
        } else if payload_value.get("LVL").is_some() {
            let sensor_alert_req = serde_json::from_str::<SensorAlertReq>(payload_str.as_str())?;
            let sensor_alert = SensorAlerts::new(sensor_alert_req);
            db.insert_sensor_alert(sensor_alert).await?;
        } else if payload_value.get("BY").is_some() {
            let manifold_refill_req =
                serde_json::from_str::<ManifoldRefillReq>(&payload_str.as_str())?;

            let manifold_refills = ManifoldRefills::new(manifold_refill_req);

            db.insert_manifold_refills(manifold_refills).await?;
        } else {
            let json_value:serde_json::Value= serde_json::from_str(&payload_str.as_str())?;
            db.insert_optional_data(json_value).await?;
        }

        return Ok(());
    }
}

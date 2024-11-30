use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SensorAlertReq {
    pub id: String,
    pub sen: String,
    pub lvl: String,
}

#[derive(Debug, Serialize)]
pub struct SensorAlerts {
    pub id: String,
    pub sen: String,
    pub lvl: String,
    pub datetime: DateTime,
}

impl SensorAlerts {
    pub fn new(sensor_alert_request: SensorAlertReq) -> Self {
        Self {
            id: sensor_alert_request.id,
            sen: sensor_alert_request.sen,
            lvl: sensor_alert_request.lvl,
            datetime: DateTime::now(),
        }
    }
}

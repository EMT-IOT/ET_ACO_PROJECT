use mongodb::bson::{doc, DateTime};
use mongodb::error::Error as MongoError;
use mongodb::{Client, Collection};
use std::env;
use thiserror::Error;

use crate::models::device::Device;
use crate::models::manifold_refills::ManifoldRefills;
use crate::models::manifold_shifts::ManifoldShifts;
use crate::models::sensor_alerts::SensorAlerts;
use crate::models::status_updates::StatusUpdates;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Databse error: {0}")]
    DatabseError(#[from] MongoError),
}

#[derive(Clone)]
pub struct DB {
    pub devices: Collection<Device>,
    pub status_updates: Collection<StatusUpdates>,
    pub manifold_refills: Collection<ManifoldRefills>,
    pub manifold_shifts: Collection<ManifoldShifts>,
    pub sensor_alerts: Collection<SensorAlerts>,
}

impl DB {
    pub async fn connect_to_db() -> Result<Self, DeviceError> {
        let uri = env::var("DB_URI").expect("failed to read the DB_URI env");
        let db_name = env::var("DB_NAME").expect("failed to read the DB_NAME env");
        let client = Client::with_uri_str(uri).await?;

        let database = client.database(&db_name);
        let devices: Collection<Device> = database.collection("devices");
        let status_updates: Collection<StatusUpdates> = database.collection("status_updates");
        let manifold_refills: Collection<ManifoldRefills> = database.collection("manifold_refills");
        let manifold_shifts: Collection<ManifoldShifts> = database.collection("manifold_shifts");
        let sensor_alerts: Collection<SensorAlerts> = database.collection("sensor_alerts");

        client
            .database(&db_name)
            .run_command(doc! {"ping":1})
            .await?;
        println!("conected to mongodb");

        return Ok(Self {
            devices,
            status_updates,
            manifold_refills,
            manifold_shifts,
            sensor_alerts,
        });
    }

    pub async fn find_device_changed_parameters(
        self: &Self,
        device_id: String,
        device_past_data: Vec<(&str, String)>,
    ) -> Result<Vec<(String, String)>, DeviceError> {
        let mut devices = Vec::new();

        println!("{}", device_id);

        let filter_query = doc! {
                "id":&device_id,
        };

        let db_result = self.devices.find_one(filter_query).await?;

        if let Some(device) = db_result {
            for (key, value) in device_past_data {
                match key {
                    "sg" => {
                        if value != device.sg {
                            devices.push((key.to_string(), device.sg.clone()));
                        }
                    }
                    "prv" => {
                        if value != device.prv {
                            devices.push((key.to_string(), device.prv.clone()));
                        }
                    }
                    "mt" => {
                        if value != device.mt {
                            devices.push((key.to_string(), device.mt.clone()));
                        }
                    }
                    "bn" => {
                        if value != device.bn {
                            devices.push((key.to_string(), device.bn.clone()));
                        }
                    }
                    "mf1_st" => {
                        if value != device.mf1_st {
                            devices.push((key.to_string(), device.mf1_st.clone()));
                        }
                    }
                    "mf2_st" => {
                        if value != device.mf2_st {
                            devices.push((key.to_string(), device.mf2_st.clone()));
                        }
                    }
                    "ws" => {
                        if value != device.ws {
                            devices.push((key.to_string(), device.ws.clone()));
                        }
                    }
                    "mf_shd" => {
                        if value != device.mf_shd {
                            devices.push((key.to_string(), device.mf_shd.clone()));
                        }
                    }
                    "sms" => {
                        if value != device.sms {
                            devices.push((key.to_string(), device.sms.clone()));
                        }
                    }
                    "to" => {
                        if value != device.to {
                            devices.push((key.to_string(), device.to.clone()));
                        }
                    }
                    "mfr" => {
                        if value != device.mfr {
                            devices.push((key.to_string(), device.mfr.clone()));
                        }
                    }
                    "gld" => {
                        if value != device.gld {
                            devices.push((key.to_string(), device.gld.clone()));
                        }
                    }
                    "aco" => {
                        if value != device.aco {
                            devices.push((key.to_string(), device.aco.clone()));
                        }
                    }
                    "hi" => {
                        if value != device.hi {
                            devices.push((key.to_string(), device.hi.clone()));
                        }
                    }
                    "lw" => {
                        if value != device.lw {
                            devices.push((key.to_string(), device.lw.clone()));
                        }
                    }
                    "ht" => {
                        if value != device.ht {
                            devices.push((key.to_string(), device.ht.clone()));
                        }
                    }
                    "sen" => {
                        if value != device.sen {
                            devices.push((key.to_string(), device.sen.clone()));
                        }
                    }
                    "stu" => {
                        if value != device.stu {
                            devices.push((key.to_string(), device.stu.clone()));
                        }
                    }
                    _ => (),
                }
            }
        }

        let filter_query = doc! {
            "id":device_id
        };
        let update_query = doc! {
            "$set":{"last_power_on":DateTime::now()},
        };

        self.devices.update_one(filter_query, update_query).await?;

        return Ok(devices);
    }

    pub async fn insert_status_update(
        self: &Self,
        status_update: StatusUpdates,
    ) -> Result<(), DeviceError> {
        self.status_updates.insert_one(status_update).await?;
        return Ok(());
    }

    pub async fn insert_manifold_refills(
        self: &Self,
        manifold_refill: ManifoldRefills,
    ) -> Result<(), DeviceError> {
        self.manifold_refills.insert_one(manifold_refill).await?;
        return Ok(());
    }

    pub async fn insert_manifold_shifts(
        self: &Self,
        manifold_shift: ManifoldShifts,
    ) -> Result<(), DeviceError> {
        self.manifold_shifts.insert_one(manifold_shift).await?;
        return Ok(());
    }

    pub async fn insert_sensor_alert(
        self: &Self,
        sensor_alert: SensorAlerts,
    ) -> Result<(), DeviceError> {
        self.sensor_alerts.insert_one(sensor_alert).await?;
        return Ok(());
    }
}

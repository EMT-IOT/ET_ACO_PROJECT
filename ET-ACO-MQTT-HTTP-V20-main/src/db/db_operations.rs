use mongodb::{bson::doc,Client, Collection};
use mongodb::error::Error as MongoError;
use thiserror::Error;
use std::env;

use crate::models::device::{Device, DeviceUpdateRequest};

#[derive(Error,Debug)]
pub enum DeviceError {
    #[error("device already exists")]
    DeviceExists,
    #[error("device not found")]
    DeviceNotFound,
    #[error("Databse error: {0}")]
    DatabseError(#[from] MongoError)
}

#[derive(Clone)]
pub struct DB{
    pub devices:Collection<Device>,
}

impl DB {
    pub async fn connect_to_db()->Result<Self,DeviceError>{
    let uri=env::var("DB_URI").expect("failed to read the DB_URI env");
    let db_name=env::var("DB_NAME").expect("failed to read the DB_NAME env");
    let client=Client::with_uri_str(uri).await?;

    let database=client.database(&db_name);
    let devices=database.collection("devices");

    client.database(&db_name).run_command(doc!{"ping":1}).await?;
    println!("conected to mongodb");

    return Ok(Self {
        devices,
    });
    }

    pub async fn insert_device(self:&Self,device:Device)->Result<(),DeviceError>{

        let unit_filter_query=doc! {
            "id":device.id.clone()
        };

        let db_result=self.devices.find_one(unit_filter_query).await?;

        if let Some(_) = db_result {
            return Err(DeviceError::DeviceExists);
        }

        self.devices.insert_one(device).await?;
        return Ok(());
    }

    pub async fn update_device_parameters(self:&Self,id:String,device:DeviceUpdateRequest)->Result<(),DeviceError> {
        let filter_query=doc! {"id": id};
        let update_doc=doc! {"$set": {
            "sg": device.sg,
            "prv": device.prv,
            "mt": device.mt,
            "bn": device.bn,
            "mf1_st": device.mf1_st,
            "mf2_st": device.mf2_st,
            "ws": device.ws,
            "mf_shd": device.mf_shd,
            "sms": device.sms,
            "to": device.to,
            "mfr": device.mfr,
            "gld": device.gld,
            "aco": device.aco,
            "hi": device.hi,
            "lw": device.lw,
            "ht": device.ht,
            "sen": device.sen,
            "stu": device.stu,
        }};

        let db_result=self.devices.update_one(filter_query, update_doc).await?;
        
        if db_result.matched_count > 0 {
            return Ok(());
        }else{
            return Err(DeviceError::DeviceNotFound)
        }
        
    }

    pub async fn get_devices_by_date(self:&Self) {
        
    }
}

use serde::{Serialize,Deserialize};
use mongodb::bson::DateTime;


fn default_zero() -> String {
    "0".to_string()
}

#[derive(Debug,Serialize,Deserialize)]
#[derive(Clone)]
pub struct DeviceUpdateRequest {
    pub sg:String,
    pub prv:String,
    //new key added on 11/3/2025
    #[serde(default="default_zero")]
    pub prm: String,
    pub mt:String,
    pub bn:String,
    pub mf1_st:String,
    pub mf2_st:String,
    //new key added on 11/3/2025
    #[serde(default="default_zero")]
    pub psr: String,
    pub ws:String,
    pub mf_shd:String,
    pub sms:String,
    pub to:String,
    pub mfr:String,
    pub gld:String,
    pub aco:String,
    pub hi:String,
    pub lw:String,
    pub ht:String,
    pub sen:String,
    pub stu:String,
    //new key added on 11/3/2025
    #[serde(default="default_zero")]
    pub prlt:String,
     //new key added on 11/3/2025
     #[serde(default="default_zero")]
    pub prht:String,
    //new key added on 11/3/2025
    #[serde(default="default_zero")]
    pub tch:String,
     //new key added on 11/3/2025
    #[serde(default="default_zero")]
    pub tmp:String,
}


#[derive(Debug,Serialize,Deserialize)]
pub struct Device {
    pub id:String,
    pub sg:String,
    pub prv:String,
    pub prm:String,
    pub mt:String,
    pub bn:String,
    pub mf1_st:String,
    pub mf2_st:String,
    pub psr:String,
    pub ws:String,
    pub mf_shd:String,
    pub sms:String,
    pub to:String,
    pub mfr:String,
    pub gld:String,
    pub aco:String,
    pub hi:String,
    pub lw:String,
    pub ht:String,
    pub sen:String,
    pub stu:String,
    pub prlt:String,
    pub prht:String,
    pub tch:String,
    pub tmp:String,
    pub last_power_on:DateTime
}

impl Device {
    pub fn new(id:String)->Self {
        Self {
            id,
            sg:"0".to_string(),
            prv:"0".to_string(),
            prm:"0".to_string(),
            mt:"0".to_string(),
            bn:"0".to_string(),
            mf1_st:"0".to_string(),
            mf2_st:"0".to_string(),
            psr:"0".to_string(),
            ws:"0".to_string(),
            mf_shd:"0".to_string(),
            sms:"0".to_string(),
            to:"0".to_string(),
            mfr:"0".to_string(),
            gld:"0".to_string(),
            aco:"0".to_string(),
            hi:"0".to_string(),
            lw:"0".to_string(),
            ht:"0".to_string(),
            sen:"0".to_string(),
            stu:"0".to_string(),
            prlt:"0".to_string(),
            prht:"0".to_string(),
            tch:"0".to_string(),
            tmp:"0".to_string(),
            last_power_on:DateTime::now()
        }
    }
}


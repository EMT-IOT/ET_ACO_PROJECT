use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Device {
    pub id:String,
    pub sg:String,
    pub prv:String,
    pub mt:String,
    pub bn:String,
    pub mf1_st:String,
    pub mf2_st:String,
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
    pub last_power_on:DateTime
}

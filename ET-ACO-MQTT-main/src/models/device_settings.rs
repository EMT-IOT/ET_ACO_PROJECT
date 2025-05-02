//new collection added on 18/4/2025
use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};

//incoming data format
/*
{
    "ID":"862360073784667",
    "SG":"19",
    "PRV":"0.100",
    "PRM":"16.000",
    "MT":"2",
    "BN":"0",
    "MF1_ST":"0",
    "MF2_ST":"2",
    "MF_OFF":"0",
    "PSR":"0",
    "WS":"0",
    "MF_SHD":"0",
    "SMS":"0",
    "TO":"30",
    "MFR":"5",
    "GLD":"0",
    "ACO":"1",
    "HI":"20",
    "LW":"10",
    "HT":"1",
    "SEN":"6",
    "STU":"30",
    "PRLT":"0.250",
    "PRHT":"2.500",
    "TCH":"1",
    "TMP":"40.5",
    "SWV":"226"
}
 */

 fn default_zero() -> String {
    "0".to_string()
}

 #[derive(Debug,Serialize,Deserialize)]
 #[serde(rename_all="UPPERCASE")]
pub struct DeviceSettingsRequest {
    #[serde(default="default_zero")]
    pub id: String,
    #[serde(default="default_zero")]
    pub sg:String,
    #[serde(default="default_zero")]
    pub prv:String,
    #[serde(default="default_zero")]
    pub prm:String,
    #[serde(default="default_zero")]
    pub mt:String,
    #[serde(default="default_zero")]
    pub bn:String,
    #[serde(default="default_zero")]
    pub mf1_st:String,
    #[serde(default="default_zero")]
    pub mf2_st:String,
    #[serde(default="default_zero")]
    pub mf_off:String,
    #[serde(default="default_zero")]
    pub psr:String,
    #[serde(default="default_zero")]
    pub ws:String,
    #[serde(default="default_zero")]
    pub mf_shd:String,
    #[serde(default="default_zero")]
    pub sms:String,
    #[serde(default="default_zero")]
    pub to:String,
    #[serde(default="default_zero")]
    pub mfr:String,
    #[serde(default="default_zero")]
    pub gld:String,
    #[serde(default="default_zero")]
    pub aco:String,
    #[serde(default="default_zero")]
    pub hi:String,
    #[serde(default="default_zero")]
    pub lw:String,
    #[serde(default="default_zero")]
    pub ht:String,
    #[serde(default="default_zero")]
    pub sen:String,
    #[serde(default="default_zero")]
    pub stu:String,
    #[serde(default="default_zero")]
    pub prlt:String,
    #[serde(default="default_zero")]
    pub prht:String,
    #[serde(default="default_zero")]
    pub tch:String,
    #[serde(default="default_zero")]
    pub tmp:String,
    #[serde(default="default_zero")]
    pub swv:String,
    //new key added on 2/5/2025
    #[serde(default="default_zero")]
    pub al:String,
    //new key added on 2/5/2025
    #[serde(default="default_zero")]
    pub ts:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct DeviceSettings{
    pub id: String,
    pub sg:String,
    pub prv:String,
    pub prm:String,
    pub mt:String,
    pub bn:String,
    pub mf1_st:String,
    pub mf2_st:String,
    pub mf_off:String,
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
    pub swv:String,
    pub al:String,
    pub ts:String,
    pub datetime:DateTime
}


impl DeviceSettings {
    pub fn new(request: DeviceSettingsRequest) -> Self {
        return Self {
          id: request.id,
          sg: request.sg,
          prv: request.prv,
          prm: request.prm,
          mt: request.mt,
          bn: request.bn,
          mf1_st: request.mf1_st,
          mf2_st: request.mf2_st,
          mf_off: request.mf_off,
          psr: request.psr,
          ws: request.ws,
          mf_shd: request.mf_shd,
          sms: request.sms,
          to: request.to,
          mfr: request.mfr,
          gld: request.gld,
          aco: request.aco,
          hi: request.hi,
          lw: request.lw,
          ht: request.ht,
          sen: request.sen,
          stu: request.stu,
          prlt: request.prlt,
          prht: request.prht,
          tch: request.tch,
          tmp: request.tmp,
          swv: request.swv,
          al: request.al,
          ts:request.ts,
          datetime:DateTime::now(),
        }
    }
}
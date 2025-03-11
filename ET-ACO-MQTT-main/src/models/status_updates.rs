use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};


// status update 
// {"ID":"862211073253668","UT":"2032","SG":"26","MF1_ST":"0","MF2_ST":"2","PSR":"0","BR_RN":"0","PR":"1.164","WT":"0.00","GAS":"0,0,0,0,0,0","TMP":"64.64"}

fn default_zero() -> String {
    "0".to_string()
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub struct StatusUpdateReq {
    pub id:String,
    pub ut:String,
    pub sg:String,
    pub mf1_st:String,
    pub mf2_st:String,
    //new key added on 12/3/2025
    #[serde(default="default_zero")]
    pub psr:String,
    pub br_rn:String,
    pub pr:String,
    pub wt:String,
    //new key added on 12/3/2025
    #[serde(default="default_zero")]
    pub gas:String,
    //new key added on 12/3/2025
    #[serde(default="default_zero")]
    pub tmp:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct StatusUpdates {
    pub id:String,
    pub ut:String,
    pub sg:String,
    pub mf1_st:String,
    pub mf2_st:String,
    pub psr:String,
    pub br_rn:String,
    pub pr:String,
    pub wt:String,
    pub gas:String,
    pub tmp:String,
    pub datetime:DateTime
}

impl StatusUpdates {
    pub fn new(status_update_request:StatusUpdateReq) -> Self {
        return Self {
            id: status_update_request.id,
            ut: status_update_request.ut,
            sg: status_update_request.sg,
            mf1_st: status_update_request.mf1_st,
            mf2_st: status_update_request.mf2_st,
            psr: status_update_request.psr,
            br_rn: status_update_request.br_rn,
            pr: status_update_request.pr,
            wt: status_update_request.wt,
            gas: status_update_request.gas,
            tmp: status_update_request.tmp,
            datetime: DateTime::now()
        };
    }
}


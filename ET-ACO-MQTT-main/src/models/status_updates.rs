use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub struct StatusUpdateReq {
    pub id:String,
    pub ut:String,
    pub sg:String,
    pub mf1_st:String,
    pub mf2_st:String,
    pub br_rn:String,
    pub pr:String,
    pub wt:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct StatusUpdates {
    pub id:String,
    pub ut:String,
    pub sg:String,
    pub mf1_st:String,
    pub mf2_st:String,
    pub br_rn:String,
    pub pr:String,
    pub wt:String,
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
            br_rn: status_update_request.br_rn,
            pr: status_update_request.pr,
            wt: status_update_request.wt,
            datetime: DateTime::now()
        };
    }
}


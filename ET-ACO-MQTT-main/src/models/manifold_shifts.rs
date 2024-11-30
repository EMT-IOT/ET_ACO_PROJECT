use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub struct ManifoldShiftReq {
    pub id:String,
    pub mf_on:String,
    pub pr:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ManifoldShifts {
    pub id:String,
    pub mf_on:String,
    pub pr:String,
    pub datetime:DateTime
}

impl ManifoldShifts {
    pub fn new(manifold_shift_request:ManifoldShiftReq) -> Self {
        return Self {
            id:manifold_shift_request.id,
            mf_on:manifold_shift_request.mf_on,
            pr:manifold_shift_request.pr,
            datetime:DateTime::now()
        };
    }
}


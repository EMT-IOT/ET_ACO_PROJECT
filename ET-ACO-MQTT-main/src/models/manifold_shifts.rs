use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};

// shifts
// the following messages will be sent from device, so this has to be handled and stored as before
// {"ID":"862211073253668","MF_ON":"2","PR":"0.000"} 
// {"ID":"862211073253668","MF_ON":"1","PR":"0.000"} 
// {"ID":"862211073253668","MF_ON":"2","PR":"4.718","MFPC":"1"} 
// {"ID":"862211073253668","MF_ON":"1","PR":"0.187","MFPC":"1"} 
// {"ID":"862211073253668","MF":"2","PR":"0.187","MFPC":"1","MFPE":"1"}
// {"ID":"862211073253668","MF":"1","PR":"0.187","MFPC":"1","MFPE":"1"}
// {"ID":"862211073253668","MF_ON":"0","PR":"0.000"} 
fn default_zero() -> String {
    "0".to_string()
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub struct ManifoldShiftReq {
    pub id:String,
    //new key added on 11/3/2024
    #[serde(default="default_zero")]
    pub mf:String,
    pub mf_on:String,
    //new key added on 16/4/2025
    #[serde(default="default_zero")]
    pub mf_off:String,
    pub pr:String,
    //new key added on 11/3/2024
    #[serde(default="default_zero")]
    pub mfpc:String,
    //new key added on 11/3/2024
    #[serde(default="default_zero")]
    pub mfpe:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ManifoldShifts {
    pub id:String,
    pub mf:String,
    pub mf_on:String,
    pub mf_off:String,
    pub pr:String,
    pub mfpc:String,
    pub mfpe:String,
    pub datetime:DateTime
}

impl ManifoldShifts {
    pub fn new(manifold_shift_request:ManifoldShiftReq) -> Self {
        return Self {
            id:manifold_shift_request.id,
            mf:manifold_shift_request.mf,
            mf_on:manifold_shift_request.mf_on,
            mf_off:manifold_shift_request.mf_off,
            pr:manifold_shift_request.pr,
            mfpc:manifold_shift_request.mfpc,
            mfpe:manifold_shift_request.mfpe,
            datetime:DateTime::now()
        };
    }
}


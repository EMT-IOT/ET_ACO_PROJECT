use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};


#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub struct ManifoldRefillReq {
    pub id:String,
    pub mf1_st:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ManifoldRefills {
    pub id:String,
    pub mf1_st:String,
    pub datetime:DateTime
}

impl ManifoldRefills {
    pub fn new(manifold_refills_request:ManifoldRefillReq) -> Self {
        return Self {
            id:manifold_refills_request.id,
            mf1_st:manifold_refills_request.mf1_st,
            datetime:DateTime::now()
        }
    }
}


use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};

fn default_zero() -> String {
    "0".to_string()
}


#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub struct ManifoldRefillReq {
    #[serde(default="default_zero")]
    pub id:String,
    #[serde(default="default_zero")]
    pub mf1_st:String,
    //new key added on 2/5/2025
    #[serde(default="default_zero")]
    pub mf2_st:String,
    //new key added on 2/5/2025
    #[serde(default="default_zero")]
    pub by:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ManifoldRefills {
    pub id:String,
    pub mf1_st:String,
    pub mf2_st:String, 
    pub by:String,
    pub datetime:DateTime
}

impl ManifoldRefills {
    pub fn new(manifold_refills_request:ManifoldRefillReq) -> Self {
        return Self {
            id:manifold_refills_request.id,
            mf1_st:manifold_refills_request.mf1_st,
            mf2_st:manifold_refills_request.mf2_st,
            by:manifold_refills_request.by,
            datetime:DateTime::now()
        }
    }
}


use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Device {
    pub id:String,
    pub sg:String,
    pub prv:String,
    //new key added on 11/3/2025
    pub prm: String,
    pub mt:String,
    pub bn:String,
    pub mf1_st:String,
    pub mf2_st:String,
    //new key added on 11/3/2025
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
    pub prlt:String,
    //new key added on 11/3/2025
    pub prht:String,
    //new key added on 11/3/2025
    pub tch:String,
    //new key added on 11/3/2025
    pub tmp:String,
    pub last_power_on:DateTime
}


// device 
// {"ID":"862211073253668","SG":"26","PRV":"0.100","PRM":"16.000","MT":"1","BN":"0","MF1_ST":"0","MF2_ST":"2","PSR":"0","WS":"0","MF_SHD":"0","SMS":"0","TO":"60","MFR":"10","GLD":"0","ACO":"1","HI":"20","LW":"10","HT":"0","SEN":"6","STU":"30","PRLT":"0.200","PRHT":"3.700","TCH":"1","TMP":":":":"}

// status update 
// {"ID":"862211073253668","UT":"2032","SG":"26","MF1_ST":"0","MF2_ST":"2","PSR":"0","BR_RN":"0","PR":"1.164","WT":"0.00","GAS":"0,0,0,0,0,0","TMP":"64.64"}

// refills
// {
//     "CL1": "1"
//   }
//   {
//     "CL1": "2"
//   }

// shifts
// the following messages will be sent from device, so this has to be handled and stored as before
// {"ID":"862211073253668","MF_ON":"2","PR":"0.000"} 
// {"ID":"862211073253668","MF_ON":"1","PR":"0.000"} 
// {"ID":"862211073253668","MF_ON":"2","PR":"4.718","MFPC":"1"} 
// {"ID":"862211073253668","MF_ON":"1","PR":"0.187","MFPC":"1"} 
// {"ID":"862211073253668","MF":"2","PR":"0.187","MFPC":"1","MFPE":"1"}
// {"ID":"862211073253668","MF":"1","PR":"0.187","MFPC":"1","MFPE":"1"}
// {"ID":"862211073253668","MF_ON":"0","PR":"0.000"} 


// possible kyes in rest api are 

// ACO, GLD, TCH, HI, LW, HT, SEN, TO, PRMN, PRLT, PRHT, PRMX, MFL1, MFL2, SIL, BR, MFSHD, MFPRD, WS, ST, MFR, SMS, STU

// the following messages will be sent from device, so this has to be handled and stored as before

use actix_web::{get, web::{self,Data}, HttpResponse, Responder};
use chrono::{FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use mongodb::bson::DateTime;

use crate::db::db_operations::DB;



#[get("/devices/{date}/{time}")]
pub async fn get_devices(db_service:Data<DB>,path:web::Path<(String,String)>) -> impl Responder {
    
    let (date_str,time_str)=path.into_inner();

    if let Ok(date)=NaiveDate::parse_from_str(&date_str, "%d/%m/%Y") {

        if let Ok(time)=NaiveTime::parse_from_str(&time_str,"%H:%M") {

            let naive_datetime=NaiveDateTime::new(date, time);

            if let Some(ist_offset)=FixedOffset::east_opt(5*3600+30*60) {

                

            
            }else{
                return HttpResponse::Conflict().body("invalid ist time format");
            }
        }else{
            return HttpResponse::Conflict().body("failed to parse time string");
        }

    }else{
        return HttpResponse::Conflict().body("failed to parse the date string");
    }

    HttpResponse::Ok().body("hello")
}
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct DeviceInitialParamsAckReq {
    pub id: String,
    pub res: String,
}

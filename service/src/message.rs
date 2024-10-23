use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub service_name: String,
    pub data_len: u8,
    pub data: String,
}



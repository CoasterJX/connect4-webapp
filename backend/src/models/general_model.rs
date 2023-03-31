use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralStatus {
    pub success: bool,
    pub msg: String,
}

impl GeneralStatus {

    pub fn success() -> Self {
        GeneralStatus {
            success: true,
            msg: "".to_owned()
        }
    }

    pub fn failure(msg: &str) -> Self {
        GeneralStatus {
            success: false,
            msg: msg.to_owned()
        }
    }
}
//Holochain request data types
#[derive(Debug, Deserialize, Serialize)]
pub struct GetEnvRequestData {
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentBitPrefixRequestData {
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBitPrefixRequestData {
    pub bit_prefix: u32
}
extern crate alloc;
use alloc::{string::*, format, vec::*, borrow::*};

pub const SEARCH_URL: &str = "https://www.mojiodpadki.si/api2/";

#[derive(serde::Deserialize, Debug)]
pub struct Response<'a, T> {
    pub id: &'a str,
    pub jsonrpc: &'a str,
    pub result: Vec<T>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Street {
    pub id: u32,
    pub label: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Address {
    pub addition: String,
    pub id: u32,
    pub municipality: String,
    pub number: String,
    pub street: String,
}


pub fn get_street_search_url_and_request(term: Option<&str>) -> (String, String) {
    let request: String = if let Some(term) = term {
        format!("{{\"method\":\"streets\",\"params\":{{\"term\":\"{term}\"}},\"id\":1723033621554,\"jsonrpc\":\"2.0\"}}")
    } else {
        "{\"method\":\"streets\",\"params\":{},\"id\":1723033621554,\"jsonrpc\":\"2.0\"}".to_owned()
    };
    (SEARCH_URL.to_string(), request)
}

pub fn get_address_search_url_and_request(street_id: u32) -> (String, String) {
    let request: String = format!("{{\"method\":\"address\",\"params\":{{\"streetId\":\"{street_id}\"}},\"id\":1723033621554,\"jsonrpc\":\"2.0\"}}");

    (SEARCH_URL.to_string(), request)
}
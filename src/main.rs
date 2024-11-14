use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::{fs::File, io::Read, time::Duration};
use url::Url;
use serde::{Deserializer, Serializer};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    b_day: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: i32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    duration: String,
    at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    req_type: HttpStatus,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Debug, Serialize, Deserialize)]
enum HttpStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "bad_request")]
    BadRequest,
    #[serde(rename = "unprocessable_entity")]
    UnprocessableEntity,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    date: String,
}

fn main() {
    let event = Event {
        name: "Event 1".to_string(),
        date: "2024-11-14".to_string(),
    };
    let json = serde_json::to_string(&event).unwrap();
    println!("\nJSON:\n{}", json);

    let deserialized_event: Event = serde_json::from_str(&json).unwrap();
    println!("\nDeserialized JSON:\n{:?}", deserialized_event);
}

fn serialize_date<S: Serializer>(date: &str, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let data: &str = Deserialize::deserialize(deserializer)?;
    let res = data.replace("Date: ", "");
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_1() {
        let mut file = File::open("request.json").unwrap();
        let mut file_str: String = String::new();
        file.read_to_string(&mut file_str).unwrap();

        let request: Request = serde_json::from_str(&file_str).unwrap();

        assert_eq!(request.stream.user_id, Uuid::from_str("8d234120-0bda-49b2-b7e0-fbd3912f6cbf").unwrap());
        assert_eq!(request.debug.duration, "234ms");
        assert_eq!(request.stream.shard_url, Url::parse("https://n3.example.com/sapi").unwrap());
        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].id, 1);
        assert_eq!(request.gifts[1].id, 2);
    }
}

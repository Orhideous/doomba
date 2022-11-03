mod state;

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub use state::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum InternalRegionType {
    Rid,
    Zid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegionCleaningParams {
    pub no_auto_passes: bool,
    pub two_pass: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Region {
    pub region_id: String,
    pub region_name: String,
    pub region_type: String, // it's actually enum like «kitchen»/«hallway»/…
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<RegionCleaningParams>,
    #[serde(rename = "type")]
    internal_region_type: InternalRegionType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StartRegions {
    pmap_id: String,
    user_pmapv_id: String,
    ordered: i64,
    regions: Vec<Region>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "command")]
pub enum Command {
    Start {
        #[serde(flatten)]
        start_regions: Option<StartRegions>,
    },
    Clean,
    Pause,
    Stop,
    Resume,
    Dock,
    Evac,
    Train,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct CommandInternal<'a> {
    #[serde(flatten)]
    command: Command,
    time: u64,
    initiator: &'a str,
}

pub(crate) struct Message {
    pub(crate) topic: &'static str,
    pub(crate) payload: String,
    #[cfg(test)]
    time: u64,
    #[cfg(test)]
    raw_json: serde_json::Value,
}

impl From<Command> for Message {
    fn from(command: Command) -> Self {
        let topic: &str = "cmd";
        let initiator: &str = "localApp";
        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let command_internal: CommandInternal = CommandInternal { command, time, initiator };
        let payload = serde_json::to_string(&command_internal).unwrap();
        Self {
            topic,
            payload,
            #[cfg(test)]
            time,
            #[cfg(test)]
            raw_json: serde_json::to_value(&command_internal).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{Command, InternalRegionType, Message, Region, StartRegions};
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    #[test]
    fn serialize_plain_command() {
        let command = Command::Clean;
        let message: Message = command.into();
        let expected = json!({
            "command": "clean",
            "time": message.time,
            "initiator": "localApp"
        });
        assert_eq!(message.topic, "cmd");
        assert_json_eq!(message.raw_json, expected);
    }

    #[test]
    fn serialize_start_simple_command() {
        let command = Command::Start { start_regions: None };
        let message: Message = command.into();
        let expected = json!({
            "command": "start",
            "time": message.time,
            "initiator": "localApp",
        });
        assert_eq!(message.topic, "cmd");
        assert_json_eq!(message.raw_json, expected);
    }

    #[test]
    fn serialize_start_complex_command() {
        let command = Command::Start {
            start_regions: Some(StartRegions {
                pmap_id: "some_pmap".to_string(),
                user_pmapv_id: "user_pmap".to_string(),
                ordered: 111,
                regions: vec![Region {
                    region_id: "123".to_string(),
                    region_name: "Hallway".to_string(),
                    region_type: "hallway".to_string(),
                    params: None,
                    internal_region_type: InternalRegionType::Rid,
                }],
            }),
        };
        let message: Message = command.into();
        let expected = json!({
            "command": "start",
            "time": message.time,
            "initiator": "localApp",
            "pmap_id": "some_pmap",
            "user_pmapv_id": "user_pmap",
            "ordered": 111,
            "regions": [
                {
                    "region_id": "123",
                    "region_name": "Hallway",
                    "region_type": "hallway",
                    "type": "rid"
                }
            ]
        });
        assert_eq!(message.topic, "cmd");
        assert_json_eq!(message.raw_json, expected);
    }
}

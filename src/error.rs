use thiserror::Error;

#[derive(Error, Debug)]
pub enum DoombaError {
    #[error(transparent)]
    MQTTError(#[from] paho_mqtt::Error),
}

use crate::api::{Command, Message};
use crate::error::DoombaError;
use paho_mqtt::AsyncClient;
use paho_mqtt::ConnectOptionsBuilder;
use paho_mqtt::CreateOptionsBuilder;
use paho_mqtt::MessageBuilder;
use paho_mqtt::SslOptionsBuilder;
use paho_mqtt::MQTT_VERSION_3_1_1;
use std::ops::{Deref, DerefMut};

const PORT: u16 = 8883;

#[derive(Clone)]
pub struct Client {
    pub client: AsyncClient,
}

impl Deref for Client {
    type Target = AsyncClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

impl Client {
    pub async fn new<S, B, P>(hostname: S, blid: B, password: P) -> Result<Self, DoombaError>
    where
        S: AsRef<str>,
        B: Into<String>,
        P: Into<String>,
    {
        let blid = blid.into();
        let uri = format!("ssl://{}:{PORT}", hostname.as_ref());
        let opts = CreateOptionsBuilder::new().server_uri(uri).client_id(blid.clone()).finalize();

        let client = AsyncClient::new(opts)?;

        let ssl_opts = SslOptionsBuilder::new()
            .enable_server_cert_auth(false)
            .enabled_cipher_suites("DEFAULT:!DH")
            .finalize();

        let conn_opts = ConnectOptionsBuilder::new()
            .mqtt_version(MQTT_VERSION_3_1_1)
            .ssl_options(ssl_opts)
            .user_name(blid)
            .password(password)
            .keep_alive_interval(std::time::Duration::from_secs(10))
            .connect_timeout(std::time::Duration::from_secs(3))
            .retry_interval(std::time::Duration::from_secs(3))
            .finalize();

        client.connect(conn_opts).await?;

        Ok(Self { client })
    }

    pub async fn send_command(&self, command: Command) -> Result<(), DoombaError> {
        self.send_message(command.into()).await
    }

    async fn send_message(&self, message: Message) -> Result<(), DoombaError> {
        self.client
            .publish(
                MessageBuilder::new()
                    .topic(message.topic)
                    .payload(message.payload)
                    .qos(0)
                    .finalize(),
            )
            .await
            .map_err(|e| e.into())
    }
}

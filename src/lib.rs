use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::io::{BufWriter, Write};

use byteorder::{WriteBytesExt, LittleEndian};

/// Client for BouyomiChan
/// using App Collaboration function (TCP)
/// default: 127.0.0.1:50001
pub struct BouyomichanClient {
    host: String,
    port: String,
}

/// Talk Config for BouyomiChan
/// Sorry, I don't know the details.
pub struct TalkConfig {
    code: u8,
    voice: i16,
    volume: i16,
    speed: i16,
    tone: i16,
}

impl Default for BouyomichanClient {
    fn default() -> Self {
        BouyomichanClient {
            host: String::from("127.0.0.1"),
            port: String::from("50001"),
        }
    }
}

impl Default for TalkConfig {
    fn default() -> Self {
        TalkConfig {
            code: 0,
            voice: 1,
            volume: -1,
            speed: -1,
            tone: -1,
        }
    }
}

impl BouyomichanClient {

    /// new Client with host and port
    pub fn new(host: impl AsRef<str>, port: impl AsRef<str>) -> Self {
        BouyomichanClient {
            host: host.as_ref().to_owned(),
            port: host.as_ref().to_owned(),
        }
    }

    /// talk with default config
    pub fn talk_with_default(&self, message: impl AsRef<str>) -> Result<(), Box<dyn std::error::Error>> {
        let config = TalkConfig::default();
        self.talk(message, &config)
    }

    /// talk with manual config
    pub fn talk(&self, message: impl AsRef<str>, config: &TalkConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;
        let message_bytes = message.as_ref().as_bytes();
        let message_length: u32 = message_bytes.len() as u32;
        let talk_command: i16 = 1;

        stream.write_i16::<LittleEndian>(talk_command)?;
        stream.write_i16::<LittleEndian>(config.speed)?;
        stream.write_i16::<LittleEndian>(config.tone)?;
        stream.write_i16::<LittleEndian>(config.volume)?;
        stream.write_i16::<LittleEndian>(config.voice)?;
        stream.write_u8(config.code)?;
        stream.write_u32::<LittleEndian>(message_length)?;
        stream.write_all(message_bytes)?;

        stream.flush()?;

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use crate::BouyomichanClient;

    #[test]
    fn it_works() {
        let client = BouyomichanClient::default();
        client.talk_with_default("こんにちは。").expect("failed to send message to BouyomiChan (this test requires local running BouyomiChan and enable App Collaboration)");
    }
}

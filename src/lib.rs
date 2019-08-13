use std::net::TcpStream;
use std::io::{Write, Read};

use byteorder::{WriteBytesExt, LittleEndian};

/// Client for BouyomiChan
/// using App Collaboration function (TCP)
/// default: 127.0.0.1:50001
pub struct BouyomichanClient {
    host: String,
    port: String,
}

type RequestResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Talk Config for BouyomiChan
/// Sorry, I don't know the details.
pub struct TalkConfig {
    pub code: u8,
    pub voice: i16,
    pub volume: i16,
    pub speed: i16,
    pub tone: i16,
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
            port: port.as_ref().to_owned(),
        }
    }

    /// talk with default config
    pub fn talk_with_default(&self, message: impl AsRef<str>) -> RequestResult<()> {
        let config = TalkConfig::default();
        self.talk(message, &config)
    }

    /// talk with manual config
    pub fn talk(&self, message: impl AsRef<str>, config: &TalkConfig) -> RequestResult<()> {
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

    /// pause BouyomiChan
    pub fn pause(&self) -> RequestResult<()> {
        self.send_simple_command(0x10)
    }

    /// resume BouyomiChan
    pub fn resume(&self) -> RequestResult<()> {
        self.send_simple_command(0x20)
    }

    /// skip a task
    pub fn skip(&self) -> RequestResult<()> {
        self.send_simple_command(0x30)
    }

    /// clear tasks
    pub fn clear(&self) -> RequestResult<()> {
        self.send_simple_command(0x40)
    }

    /// get pause status
    pub fn get_pause(&self) -> RequestResult<bool> {
        let res = self.send_command_has_response(0x110)?;
        Ok(*res.get(0).ok_or("could not parse request result..")? == 1)
    }

    /// get playing status
    pub fn get_now_playing(&self) -> RequestResult<bool> {
        let res = self.send_command_has_response(0x120)?;
        Ok(*res.get(0).ok_or("could not parse request result..")? == 1)
    }

    /// get number of remain tasks
    pub fn get_remain_task(&self) -> RequestResult<u32> {
        let res = self.send_command_has_response(0x130)?;
        let num = (0..=3).rev().into_iter().fold(0, |sum: u32, i| { (sum + (sum << 4)) + res[i] as u32 });
        Ok(num)
    }

    fn send_simple_command(&self, command_id: i16) -> RequestResult<()> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;
        stream.write_i16::<LittleEndian>(command_id)?;
        stream.flush()?;
        Ok(())
    }

    fn send_command_has_response(&self, command_id: i16) -> RequestResult<Vec<u8>> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;
        stream.write_i16::<LittleEndian>(command_id)?;
        stream.flush()?;
        let mut res = Vec::new();
        stream.read_to_end(&mut res)?;
        Ok(res)
    }

}

#[cfg(test)]
mod tests {
    use crate::BouyomichanClient;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn it_works() {

        // Expected behavior is..
        // こんばんは -> 月がきれいですね (End)

        let client = BouyomichanClient::default();

        // test talk
        client.talk_with_default("こんばんは。").expect("failed to send message to BouyomiChan (this test requires local running BouyomiChan and enable App Collaboration)");

        // test pause
        client.pause().expect("failed to pause.");

        // test pause status
        assert!(client.get_pause().expect("failed to get pause status."));

        // push task
        client.talk_with_default("月が綺麗ですね。").expect("failed to send message to BouyomiChan (this test requires local running BouyomiChan and enable App Collaboration)");

        // test skip
        client.talk_with_default("月が綺麗ですね。").expect("failed to send message to BouyomiChan (this test requires local running BouyomiChan and enable App Collaboration)");
        client.skip().expect("failed to skip");


        // wait updating tasks
        sleep(Duration::from_secs(3));
        // test remain number
        assert_eq!(client.get_remain_task().expect("failed to get remain tasks."), 2);


        // test resume
        client.resume().expect("failed to resume.");

        // test playing status
        assert_eq!(true, client.get_now_playing().expect("failed to get playing status"));

        // pause
        client.pause().expect("failed to pause.");

        // this will not play
        client.talk_with_default("さようなら。").expect("failed to send message to BouyomiChan (this test requires local running BouyomiChan and enable App Collaboration)");
        // clear tasks
        client.clear().expect("failed to clear.");
        // resume
        client.resume().expect("failed to resume.");
    }
}

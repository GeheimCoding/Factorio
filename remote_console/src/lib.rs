use std::{
    io::{self, Error, ErrorKind, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

const TCP_TIMEOUT_IN_SECONDS: u64 = 80;
const MAX_PACKET_LENGTH_IN_BYTES: usize = 4096;

const PACKET_TYPE_SERVERDATA_AUTH: u32 = 3;
const PACKET_TYPE_SERVERDATA_EXECCOMMAND: u32 = 2;

// TODO: add documentation
// TODO: add custom error type
// TODO: map response errors to custom error
pub struct RemoteConsole {
    stream: TcpStream,
    buffer: [u8; MAX_PACKET_LENGTH_IN_BYTES],
}

impl RemoteConsole {
    pub fn new(host: &str, port: u16, password: &str) -> io::Result<Self> {
        let timeout = Duration::from_secs(TCP_TIMEOUT_IN_SECONDS);
        let stream = TcpStream::connect_timeout(
            &format!("{host}:{port}")
                .to_socket_addrs()?
                .next()
                .ok_or(Error::new(ErrorKind::Other, "invalid socket address"))?,
            timeout,
        )?;
        stream.set_read_timeout(Some(timeout))?;
        stream.set_write_timeout(Some(timeout))?;

        let mut console = RemoteConsole {
            stream,
            buffer: [0u8; MAX_PACKET_LENGTH_IN_BYTES],
        };
        if console.is_authenticated(password)? {
            Ok(console)
        } else {
            Err(Error::new(ErrorKind::Other, "wrong password"))
        }
    }

    pub fn send_command(&mut self, command: &str) -> io::Result<String> {
        self.send_packet(
            &format!("/silent-command {command}"),
            PACKET_TYPE_SERVERDATA_EXECCOMMAND,
        )?;
        self.get_response()
    }

    fn is_authenticated(&mut self, password: &str) -> io::Result<bool> {
        self.send_packet(password, PACKET_TYPE_SERVERDATA_AUTH)?;
        self.receive_response()?;
        let mut id = [0u8; 4];

        id.swap_with_slice(&mut self.buffer[4..8]);
        // on success the id (8 in our case) will be returned
        Ok(u32::from_le_bytes(id) != u32::MAX)
    }

    fn send_packet(&mut self, payload: &str, packet_type: u32) -> io::Result<usize> {
        let payload_len = payload.len();
        // same as below but with additional 4 for packet size
        let mut packet = vec![0; payload_len + 14];

        // 4 for packet type, 4 for id and 2 for null termination and empty message
        (payload_len as u32 + 10u32)
            .to_le_bytes()
            .swap_with_slice(&mut packet[0..4]);
        // we don't care about the id (for now), so just set it to anything
        8u32.to_le_bytes().swap_with_slice(&mut packet[4..8]);
        packet_type
            .to_le_bytes()
            .swap_with_slice(&mut packet[8..12]);
        payload
            .as_bytes()
            .to_owned()
            .swap_with_slice(&mut packet[12..12 + payload_len]);

        self.stream.write(&packet)
    }

    fn get_response(&mut self) -> io::Result<String> {
        let mut packet_length = self.receive_response()?;
        let mut total_length_buffer = [0u8; 4];
        total_length_buffer.swap_with_slice(&mut self.buffer[0..4]);
        let mut total_length = u32::from_le_bytes(total_length_buffer) as usize;

        // even if the payload is exactly the total_length it is sent as multiple packets
        // due to the potential null termination and empty message
        let mut multi_message = packet_length <= total_length;
        let mut response = self.buffer[12..packet_length].to_vec();

        while multi_message {
            total_length -= packet_length;
            packet_length = self.receive_response()?;
            response.append(&mut self.buffer[0..packet_length].to_vec());

            if total_length < packet_length {
                multi_message = false;
            }
        }
        // removes potential null termination and empty message from last packet
        // normally the last three bytes are [10, 0, 0] with 10 = '\n'
        // however when the packet approaches MAX_PACKET_LENGTH_IN_BYTES
        // those will be pushed to the right and are no longer present
        let mut response = String::from_utf8(response)
            .map_err(|_| Error::new(ErrorKind::Other, "invalid UTF-8"))?;
        while let Some(last_char) = response.pop() {
            if last_char != '\0' && last_char != '\n' {
                response.push(last_char);
                break;
            }
        }
        Ok(response)
    }

    fn receive_response(&mut self) -> io::Result<usize> {
        self.stream.read(&mut self.buffer)
    }
}

// TODO: add tests

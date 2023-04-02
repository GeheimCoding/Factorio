use std::{
    io::{self, Error, ErrorKind, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

const SERVERDATA_AUTH: u32 = 3;
const SERVERDATA_EXECCOMMAND: u32 = 2;
const MAX_PACKET_LENGTH_IN_BYTES: usize = 4096;

// TODO: better error handling
pub struct RemoteConsole {
    stream: TcpStream,
    buffer: [u8; MAX_PACKET_LENGTH_IN_BYTES],
}

impl RemoteConsole {
    pub fn new(host: &str, port: u16, password: &str) -> io::Result<Self> {
        let mut console = RemoteConsole {
            stream: TcpStream::connect_timeout(
                &format!("{host}:{port}")
                    .to_socket_addrs()?
                    .next()
                    .ok_or(Error::new(ErrorKind::Other, "invalid socket address"))?,
                Duration::from_secs(2),
            )?,
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
            SERVERDATA_EXECCOMMAND,
        )?;
        self.get_response()
    }

    fn is_authenticated(&mut self, password: &str) -> io::Result<bool> {
        self.send_packet(password, SERVERDATA_AUTH)?;
        self.receive_response()?;
        let mut id = [0u8; 4];

        id.swap_with_slice(&mut self.buffer[4..8]);
        // on success the id (8) will be returned back
        Ok(u32::from_le_bytes(id) != u32::MAX)
    }

    fn get_response(&mut self) -> io::Result<String> {
        let mut packet_length = self.receive_response()?;
        let mut total_length_buffer = [0u8; 4];
        total_length_buffer.swap_with_slice(&mut self.buffer[0..4]);
        let mut total_length = u32::from_le_bytes(total_length_buffer) as usize;

        let multi_message = packet_length < total_length;
        let mut response = String::from_utf8(self.buffer[12..packet_length].to_vec()).unwrap();

        while multi_message {
            total_length -= packet_length;
            packet_length = self.receive_response()?;
            response.push_str(&String::from_utf8(self.buffer[0..packet_length].to_vec()).unwrap());

            // + 4 because of total packet size in first packet
            if total_length + 4 == packet_length {
                break;
            }
        }
        // removes null termination and empty message from last packet
        for _ in 0..3 {
            response.pop();
        }
        Ok(response)
    }

    fn send_packet(&mut self, payload: &str, packet_type: u32) -> io::Result<usize> {
        let payload_len = payload.len();
        // 4 for the packet size, 4 for the packet type,
        // 4 for the id and 2 for null termination and empty string
        let packet_size: u32 = payload_len as u32 + 14;
        if packet_size as usize > MAX_PACKET_LENGTH_IN_BYTES {
            return Err(Error::new(ErrorKind::Other, "packet is too big"));
        }

        let mut packet = [0u8; MAX_PACKET_LENGTH_IN_BYTES];
        (packet_size - 4)
            .to_le_bytes()
            .swap_with_slice(&mut packet[0..4]);
        // we don't care about the id, so just set it to anything
        8u32.to_le_bytes().swap_with_slice(&mut packet[4..8]);
        packet_type
            .to_le_bytes()
            .swap_with_slice(&mut packet[8..12]);
        payload
            .as_bytes()
            .to_owned()
            .swap_with_slice(&mut packet[12..12 + payload_len]);

        let packet = packet[..packet_size as usize].to_vec();
        self.stream.write(&packet)
    }

    fn receive_response(&mut self) -> io::Result<usize> {
        // TODO: read_to_string?
        self.stream.read(&mut self.buffer)
    }
}

// TODO: add tests

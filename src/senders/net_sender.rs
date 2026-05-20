use crate::types::{Pack, Sender};
use std::net::{UdpSocket, TcpStream};
use std::io::Write;

pub enum NetProtocol { UDP, TCP }

pub struct NetSender {
    protocol: NetProtocol,
    stream: Option<TcpStream>,
    udp_socket: Option<UdpSocket>,
    target_addr: &'static str,
}

impl NetSender {
    pub fn new(protocol: NetProtocol, local_addr: &'static str, target_addr: &'static str) -> Self {
        match protocol {
            NetProtocol::UDP => {
                // Привязываем UDP сокет к конкретному ЗАДАННОМУ локальному адресу/порту
                let socket = UdpSocket::bind(local_addr).expect("Ошибка привязки локального UDP");
                Self { protocol, stream: None, udp_socket: Some(socket), target_addr }
            }
            NetProtocol::TCP => {
                // TCP сразу устанавливает соединение с таргетом
                let stream = TcpStream::connect(target_addr).expect("Ошибка TCP подключения");
                Self { protocol, stream: Some(stream), udp_socket: None, target_addr }
            }
        }
    }
}

impl<const N: usize> Sender<N> for NetSender {
    fn send(&mut self, pack: &Pack<N>) -> Result<(), &'static str> {
        let slice_len = (N * 8) + 8;
        let raw_ptr = pack as *const Pack<N> as *const u8;
        let bytes = unsafe { std::slice::from_raw_parts(raw_ptr, slice_len) };

        match self.protocol {
            NetProtocol::UDP => {
                if let Some(ref socket) = self.udp_socket {
                    socket.send_to(bytes, self.target_addr).map_err(|_| "UDP Send Error")?;
                }
            }
            NetProtocol::TCP => {
                if let Some(ref mut stream) = self.stream {
                    stream.write_all(bytes).map_err(|_| "TCP Write Error")?;
                    stream.flush().map_err(|_| "TCP Flush Error")?;
                }
            }
        }
        Ok(())
    }
}

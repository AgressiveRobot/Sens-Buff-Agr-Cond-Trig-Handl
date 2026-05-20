use crate::types::{Pack, Receiver};
use std::net::{UdpSocket, TcpListener};
use std::io::Read;

pub struct NetReceiver {
    udp_socket: Option<UdpSocket>,
    tcp_listener: Option<TcpListener>,
}

impl NetReceiver {
    pub fn new_udp(listen_addr: &'static str) -> Self {
        let socket = UdpSocket::bind(listen_addr).expect("Udp bind error");
        socket.set_read_timeout(Some(std::time::Duration::from_millis(100))).unwrap();
        Self { udp_socket: Some(socket), tcp_listener: None }
    }

    pub fn new_tcp(listen_addr: &'static str) -> Self {
        let listener = TcpListener::bind(listen_addr).expect("Tcp bind error");
        listener.set_nonblocking(true).unwrap();
        Self { udp_socket: None, tcp_listener: Some(listener) }
    }
}

impl<const N: usize> Receiver<N> for NetReceiver {
    fn receive(&mut self) -> Option<Pack<N>> {
        let slice_len = (N * 8) + 8;
        let mut target_pack = Pack::<N> { values: [0; N], count: 0 };
        let raw_ptr = &mut target_pack as *mut Pack<N> as *mut u8;
        let buffer_slice = unsafe { std::slice::from_raw_parts_mut(raw_ptr, slice_len) };

        if let Some(ref socket) = self.udp_socket {
            if let Ok((amt, _)) = socket.recv_from(buffer_slice) {
                if amt == slice_len { return Some(target_pack); }
            }
        } else if let Some(ref listener) = self.tcp_listener {
            if let Ok((mut stream, _)) = listener.accept() {
                // Для TCP этого действительно достаточно — просто выгребаем точный размер pack байт
                if stream.read_exact(buffer_slice).is_ok() {
                    return Some(target_pack);
                }
            }
        }
        None
    }
}

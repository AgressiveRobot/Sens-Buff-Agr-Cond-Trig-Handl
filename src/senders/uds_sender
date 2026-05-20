use crate::types::{Pack, Sender};
use std::os::unix::net::UnixDatagram;
use std::fs;

pub struct UdsSender {
    socket: UnixDatagram,
    target_path: &'static str,
}

impl UdsSender {
    pub fn new(local_path: &'static str, target_path: &'static str) -> Self {
        let _ = fs::remove_file(local_path); // чистим за собой
        let socket = UnixDatagram::bind(local_path).expect("Ошибка привязки локального UDS");
        Self { socket, target_path }
    }
}

impl<const N: usize> Sender<N> for UdsSender {
    fn send(&mut self, pack: &Pack<N>) -> Result<(), &'static str> {
        let slice_len = (N * 8) + 8;
        let raw_ptr = pack as *const Pack<N> as *const u8;
        let bytes = unsafe { std::slice::from_raw_parts(raw_ptr, slice_len) };

        self.socket.send_to(bytes, self.target_path).map_err(|_| "UDS Send Error")?;
        Ok(())
    }
}

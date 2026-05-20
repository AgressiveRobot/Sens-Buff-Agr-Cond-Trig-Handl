use crate::types::{Pack, Receiver};
use std::os::unix::net::UnixDatagram;
use std::fs;

pub struct UdsReceiver {
    socket: UnixDatagram,
}

impl UdsReceiver {
    pub fn new(socket_path: &'static str) -> Self {
        let _ = fs::remove_file(socket_path);
        let socket = UnixDatagram::bind(socket_path).expect("Uds bind error");
        socket.set_read_timeout(Some(std::time::Duration::from_millis(100))).unwrap();
        Self { socket }
    }
}

impl<const N: usize> Receiver<N> for UdsReceiver {
    fn receive(&mut self) -> Option<Pack<N>> {
        let slice_len = (N * 8) + 8;
        let mut target_pack = Pack::<N> { values: [0; N], count: 0 };
        let raw_ptr = &mut target_pack as *mut Pack<N> as *mut u8;
        let buffer_slice = unsafe { std::slice::from_raw_parts_mut(raw_ptr, slice_len) };

        if let Ok((amt, _)) = self.socket.recv_from(buffer_slice) {
            if amt == slice_len { return Some(target_pack); }
        }
        None
    }
}

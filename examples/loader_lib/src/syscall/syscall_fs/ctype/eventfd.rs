use alloc::sync::Arc;
use axerrno::{AxError, AxResult};
use axsync::Mutex;
use axtask::yield_now;

use crate::linux_env::{
    axfs_ext::api::{FileIO, FileIOType, OpenFlags},
    eventfd_ext::{EventFd, EventFdFlag, EventFdWriteResult},
};

pub fn create_eventfd(initval: u64, flags: u32) -> Arc<dyn FileIO> {
    return Arc::new(EventFdWrapper::new(initval, flags));
}

// https://man7.org/linux/man-pages/man2/eventfd2.2.html
struct EventFdWrapper {
    eventfd: Arc<Mutex<EventFd>>,
}

impl EventFdWrapper {
    pub fn new(initval: u64, flags: u32) -> EventFdWrapper {
        EventFdWrapper {
            eventfd: Arc::new(Mutex::new(EventFd::new(initval, flags))),
        }
    }
}

impl FileIO for EventFdWrapper {
    fn read(&self, buf: &mut [u8]) -> AxResult<usize> {
        let len: usize = core::mem::size_of::<u64>();
        if buf.len() < len {
            return Err(AxError::InvalidInput);
        }

        loop {
            let mut eventfd_guard = self.eventfd.lock();
            if let Some(value) = eventfd_guard.read() {
                buf[0..len].copy_from_slice(&value.to_ne_bytes());
                return Ok(len);
            }

            if eventfd_guard.is_flag_set(EventFdFlag::EFD_NONBLOCK) {
                return Err(AxError::WouldBlock);
            }

            drop(eventfd_guard);
            yield_now()
        }
    }

    fn write(&self, buf: &[u8]) -> AxResult<usize> {
        let len: usize = core::mem::size_of::<u64>();
        let val = u64::from_ne_bytes(buf[0..len].try_into().unwrap());
        if buf.len() < 8 {
            return Err(AxError::InvalidInput);
        }

        loop {
            let mut eventfd_guard = self.eventfd.lock();
            let write_result = eventfd_guard.write(val);
            match write_result {
                EventFdWriteResult::OK => return Ok(len),
                EventFdWriteResult::BadInput => return Err(AxError::InvalidInput),
                EventFdWriteResult::NotReady => {
                    if eventfd_guard.is_flag_set(EventFdFlag::EFD_NONBLOCK) {
                        return Err(AxError::WouldBlock);
                    }
                }
            }

            drop(eventfd_guard);
            yield_now()
        }
    }

    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        true
    }

    fn executable(&self) -> bool {
        false
    }

    fn get_type(&self) -> FileIOType {
        FileIOType::Other
    }

    fn ready_to_read(&self) -> bool {
        self.eventfd.lock().ready_to_read()
    }

    fn ready_to_write(&self) -> bool {
        self.eventfd.lock().ready_to_write()
    }

    fn get_status(&self) -> OpenFlags {
        let eventfd_guard = self.eventfd.lock();
        let mut status = OpenFlags::RDWR;
        if eventfd_guard.is_flag_set(EventFdFlag::EFD_NONBLOCK) {
            status |= OpenFlags::NON_BLOCK;
        }
        if eventfd_guard.is_flag_set(EventFdFlag::EFD_CLOEXEC) {
            status |= OpenFlags::CLOEXEC;
        }

        status
    }
}

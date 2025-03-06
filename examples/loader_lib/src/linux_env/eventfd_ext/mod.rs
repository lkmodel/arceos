//! The EventFd object to be used by syscall

#![no_std]

use core::convert::{From, Into};
use core::option::Option;
use core::option::Option::{None, Some};
use core::prelude::rust_2021::derive;

bitflags::bitflags! {
    /// The flags used to create an EventFd object
    /// <https://sites.uclouvain.be/SystInfo/usr/include/sys/eventfd.h.html>
    #[derive(Clone, Copy, Debug)]
    pub struct EventFdFlag: u32 {
        /// SEMAPHORE flag
        const EFD_SEMAPHORE = 0x1;
        /// NONBLOCK flag
        const EFD_NONBLOCK  = 0x800;
        /// CLOEXEC flag
        const EFD_CLOEXEC   = 0x80000;
    }
}

/// the result of writing to a EventFd object
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventFdWriteResult {
    /// the write is done successfully
    OK,

    /// an attempt is made to write the value 0xffffffffffffffff (f64::MAX)
    BadInput,

    /// it's not writable yet
    NotReady,
}

/// <https://man7.org/linux/man-pages/man2/eventfd2.2.html>
pub struct EventFd {
    value: u64,
    flags: u32,
}

/// the EventFd data type
impl EventFd {
    /// create an EventFd object with initial value and flags
    pub fn new(initval: u64, flags: u32) -> EventFd {
        EventFd {
            value: initval,
            flags,
        }
    }

    /// read the EventFd
    pub fn read(&mut self) -> Option<u64> {
        // If EFD_SEMAPHORE was not specified and the eventfd counter has a nonzero value, then a read returns 8 bytes containing that value,
        // and the counter's value is reset to zero.
        if !self.is_semaphore_set() && self.value != 0 {
            let result = self.value;
            self.value = 0;
            return Some(result);
        }

        // If EFD_SEMAPHORE was specified and the eventfd counter has a nonzero value, then a read returns 8 bytes containing the value 1,
        // and the counter's value is decremented by 1.
        if self.is_semaphore_set() && self.value != 0 {
            self.value -= 1;
            return Some(1u64);
        }

        // If the eventfd counter is zero at the time of the call to read,
        // then the call either blocks until the counter becomes nonzero (at which time, the read proceeds as described above)
        // or fails with the error EAGAIN if the file descriptor has been made nonblocking.
        None
    }

    /// write to EventFd
    pub fn write(&mut self, val: u64) -> EventFdWriteResult {
        if val == u64::MAX {
            return EventFdWriteResult::BadInput;
        }

        match self.value.checked_add(val + 1) {
            // no overflow
            Some(_) => {
                self.value += val;
                EventFdWriteResult::OK
            }
            // overflow
            None => EventFdWriteResult::NotReady,
        }
    }

    /// check if the specified flag is set or not
    pub fn is_flag_set(&self, flag: EventFdFlag) -> bool {
        self.flags & flag.bits() != 0
    }

    /// The file descriptor is readable if the counter has a value greater than 0
    pub fn ready_to_read(&self) -> bool {
        self.value > 0
    }

    /// The file descriptor is writable if it is possible to write a value of at least "1" without blocking.
    pub fn ready_to_write(&self) -> bool {
        self.value < u64::MAX - 1
    }

    fn is_semaphore_set(&self) -> bool {
        self.is_flag_set(EventFdFlag::EFD_SEMAPHORE)
    }
}

/// create an EventFd object with flags of zero default value
pub fn create_eventfd(value: u64) -> EventFd {
    value.into()
}

/// the util function to create an Eventfd object
pub fn create_eventfd_with_flags(value: u64, flags: u32) -> EventFd {
    (value, flags).into()
}

impl From<u64> for EventFd {
    fn from(value: u64) -> Self {
        EventFd { value, flags: 0 }
    }
}

impl From<(u64, u32)> for EventFd {
    fn from((value, flags): (u64, u32)) -> Self {
        EventFd { value, flags }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_flags_should_not_set() {
        let event_fd: EventFd = 42.into();
        assert!(!event_fd.is_flag_set(EventFdFlag::EFD_SEMAPHORE));
        assert!(!event_fd.is_flag_set(EventFdFlag::EFD_NONBLOCK));
        assert!(!event_fd.is_flag_set(EventFdFlag::EFD_CLOEXEC));
    }

    #[test]
    fn only_efd_semaphore_is_set() {
        let event_fd: EventFd = (42, 0x1).into();
        assert!(event_fd.is_flag_set(EventFdFlag::EFD_SEMAPHORE));
        assert!(!event_fd.is_flag_set(EventFdFlag::EFD_NONBLOCK));
        assert!(!event_fd.is_flag_set(EventFdFlag::EFD_CLOEXEC));
    }

    #[test]
    fn only_efd_nonblock_is_set() {
        let event_fd: EventFd = (42, 0x800).into();
        assert!(!event_fd.is_flag_set(EventFdFlag::EFD_SEMAPHORE));
        assert!(event_fd.is_flag_set(EventFdFlag::EFD_NONBLOCK));
        assert!(!event_fd.is_flag_set(EventFdFlag::EFD_CLOEXEC));
    }

    #[test]
    fn only_efd_cloexec_is_set() {
        let event_fd: EventFd = (42, 0x80000).into();
        assert!(!event_fd.is_flag_set(EventFdFlag::EFD_SEMAPHORE));
        assert!(!event_fd.is_flag_set(EventFdFlag::EFD_NONBLOCK));
        assert!(event_fd.is_flag_set(EventFdFlag::EFD_CLOEXEC));
    }

    #[test]
    fn read_with_efd_semaphore_not_set() {
        let mut event_fd: EventFd = 42.into();
        assert_eq!(Some(42), event_fd.read());
        assert_eq!(None, event_fd.read());
    }

    #[test]
    fn read_with_efd_semaphore_set() {
        let mut event_fd: EventFd = create_eventfd_with_flags(2, EventFdFlag::EFD_SEMAPHORE.bits());
        assert_eq!(Some(1), event_fd.read());
        assert_eq!(Some(1), event_fd.read());
        assert_eq!(None, event_fd.read());
    }

    #[test]
    fn write_max_value() {
        let mut event_fd: EventFd = 42.into();
        assert_eq!(EventFdWriteResult::BadInput, event_fd.write(u64::MAX))
    }

    #[test]
    fn test_overflow_write() {
        let mut event_fd: EventFd = (u64::MAX - 1).into();
        assert_eq!(EventFdWriteResult::NotReady, event_fd.write(2))
    }

    #[test]
    fn test_non_overflow_write() {
        let mut event_fd: EventFd = (u64::MAX - 2).into();
        assert_eq!(EventFdWriteResult::OK, event_fd.write(1));

        assert_eq!(Some(u64::MAX - 1), event_fd.read());
    }

    #[test]
    fn test_ready_to_read() {
        let event_fd1: EventFd = 0.into();
        assert!(!event_fd1.ready_to_read());

        let event_fd2: EventFd = 42.into();
        assert!(event_fd2.ready_to_read());
    }

    #[test]
    fn test_ready_to_write() {
        let event_fd1: EventFd = 0.into();
        assert!(event_fd1.ready_to_write());

        let event_fd2: EventFd = (u64::MAX - 2).into();
        assert!(event_fd2.ready_to_write());

        let event_fd2: EventFd = (u64::MAX - 1).into();
        assert!(!event_fd2.ready_to_write());
    }
}

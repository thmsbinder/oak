//
// Copyright 2022 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::syscall;
use core::ffi::{c_int, c_size_t, c_ssize_t, c_void};
use oak_restricted_kernel_interface::{Errno, Syscall};

#[no_mangle]
pub extern "C" fn sys_read(fd: c_int, buf: *mut c_void, count: c_size_t) -> c_ssize_t {
    unsafe { syscall!(Syscall::Read, fd, buf, count) }
}

pub fn read(fd: i32, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = sys_read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());

    if ret < 0 {
        Err(Errno::from_repr(ret)
            .unwrap_or_else(|| panic!("unexpected error from read syscall: {}", ret)))
    } else {
        Ok(ret as usize)
    }
}

#[no_mangle]
pub extern "C" fn sys_write(fd: c_int, buf: *const c_void, count: c_size_t) -> c_ssize_t {
    unsafe { syscall!(Syscall::Write, fd, buf, count) }
}

pub fn write(fd: i32, buf: &[u8]) -> Result<usize, Errno> {
    let ret = sys_write(fd, buf.as_ptr() as *const c_void, buf.len());

    if ret < 0 {
        Err(Errno::from_repr(ret)
            .unwrap_or_else(|| panic!("unexpected error from write syscall: {}", ret)))
    } else {
        Ok(ret as usize)
    }
}

#[no_mangle]
pub extern "C" fn sys_fsync(fd: c_int) -> c_ssize_t {
    unsafe { syscall!(Syscall::Fsync, fd) }
}

#[inline]
pub fn fsync(fd: i32) -> Result<(), Errno> {
    let ret = sys_fsync(fd);

    if ret < 0 {
        Err(Errno::from_repr(ret)
            .unwrap_or_else(|| panic!("unexpected error from fsync syscall: {}", ret)))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::os::fd::AsRawFd;

    use super::*;

    #[test]
    fn test_read_write() {
        let (reader, writer) = os_pipe::pipe().unwrap();

        let tx = b"test";
        assert_eq!(Ok(4), write(writer.as_raw_fd(), tx));

        let mut rx = [0u8; 4];
        assert_eq!(Ok(4), read(reader.as_raw_fd(), &mut rx));

        assert_eq!(tx, &rx);
    }

    #[test]
    fn test_erroneus_read() {
        let fd = {
            let (reader, _) = os_pipe::pipe().unwrap();
            reader.as_raw_fd()
        };

        let mut rx = [0u8; 4];
        assert!(read(fd, &mut rx).is_err());
    }

    #[test]
    fn test_erroneus_write() {
        let fd = {
            let (_, writer) = os_pipe::pipe().unwrap();
            writer.as_raw_fd()
        };

        let tx = b"test";
        assert!(write(fd, tx).is_err());
    }

    #[test]
    fn test_fsync() {
        let fd = {
            let (_, writer) = os_pipe::pipe().unwrap();
            writer.as_raw_fd()
        };
        assert!(fsync(fd).is_err());
    }
}
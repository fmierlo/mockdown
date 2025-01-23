# Mockdown &emsp; [![latest]][crates.io] [![msrv]][crates.io] [![docs.rs]][docs.rs/mockdown] ![][build]

Mockdown is a single file and macro/dependency free mock library for Rust.

# Example

## Simple

```rust
pub mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

pub mod plus {
    #[cfg(not(test))]
    use super::math;

    #[cfg(test)]
    use mocks::math;

    pub fn one(x: i32) -> i32 {
        math::add(x, 1)
    }

    #[cfg(test)]
    pub mod mocks {
        pub mod math {
            use mockdown::{mockdown, Mock};

            #[derive(Debug, PartialEq)]
            pub struct Add(pub i32, pub i32);

            pub fn add(x: i32, y: i32) -> i32 {
                let args = Add(x, y);
                mockdown().mock(args).unwrap()
            }
        }
    }

    #[cfg(test)]
    pub mod tests {
        use mockdown::{mockdown, Mock};
        use super::mocks::math;

        #[test]
        pub fn test_one() {
            mockdown().expect(|args| {
                assert_eq!(math::Add(1, 1), args);
                2
            });

            let z = super::one(1);
            assert_eq!(z, 2);
        }
    }
}
```
## Libc

### mod sys

```rust
use libc::{c_int, c_ulong, c_void};

pub const SIOCGIFLLADDR: c_ulong = 0xc020699e;

pub fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int {
    unsafe { libc::socket(domain, ty, protocol) }
}

pub fn ioctl(fd: c_int, request: c_ulong, arg: *mut c_void) -> c_int {
    unsafe { libc::ioctl(fd, request, arg) }
}

pub fn close(fd: c_int) -> c_int {
    unsafe { libc::close(fd) }
}

pub fn errno() -> c_int {
    unsafe { *libc::__error() }
}
```

### mod socket

```rust
#[cfg(not(test))]
use super::sys;

#[cfg(test)]
use mocks::sys;

fn open_local_dgram() -> Result<OpenSocket, i32> {
    match sys::socket(libc::AF_LOCAL, libc::SOCK_DGRAM, 0) {
        fd if fd >= 0 => Ok(OpenSocket { fd }),
        _ => Err(sys::errno()),
    }
}

struct OpenSocket {
    fd: libc::c_int,
}

impl OpenSocket {
    fn get_lladdr(&self, arg: *mut libc::c_void) -> Result<(), i32> {
        let fd = self.fd;
        match sys::ioctl(fd, sys::SIOCGIFLLADDR, arg) {
            0 => Ok(()),
            _ => Err(sys::errno()),
        }
    }
}

impl Drop for OpenSocket {
    fn drop(&mut self) {
        match sys::close(self.fd) {
            0 => (),
            _ => eprintln!("Error: {:?}", sys::errno()),
        };
    }
}

#[cfg(test)]
pub mod mocks {
    pub mod sys {
        use libc::{c_int, c_ulong, c_void};
        use mockdown::Mockdown;
        use mockdown::Static;
        use std::{cell::RefCell, thread::LocalKey};

        pub use super::super::super::sys::SIOCGIFLLADDR;

        thread_local! {
            static MOCKDOWN: RefCell<Mockdown> = Mockdown::thread_local();
        }

        pub fn mockdown() -> &'static LocalKey<RefCell<Mockdown>> {
            &MOCKDOWN
        }

        #[derive(Debug, PartialEq)]
        pub struct Socket(pub c_int, pub c_int, pub c_int);
        #[derive(Debug, PartialEq)]
        pub struct IoCtl(pub (c_int, c_ulong), pub *mut c_void);
        #[derive(Debug, PartialEq)]
        pub struct Close(pub c_int);
        #[derive(Debug)]
        pub struct ErrNo();

        pub fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int {
            let args = Socket(domain, ty, protocol);
            MOCKDOWN.mock(args).unwrap()
        }

        pub fn ioctl(fd: c_int, request: c_ulong, arg: *mut c_void) -> c_int {
            let args = IoCtl((fd, request), arg);
            MOCKDOWN.mock(args).unwrap()
        }

        pub fn close(fd: c_int) -> c_int {
            let args = Close(fd);
            MOCKDOWN.mock(args).unwrap()
        }

        pub fn errno() -> c_int {
            let args = ErrNo();
            MOCKDOWN.mock(args).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    // The code from the modules below has been removed to shorten the example.
    // But they can be found here: https://github.com/fmierlo/nic-roaming/tree/main/net-sys/src
    use crate::{ifreq, IfName, LinkLevelAddress};

    use super::mocks::sys;
    use mockdown::Static;
    use std::sync::LazyLock;

    const MOCK_FD: libc::c_int = 3;

    static IFNAME: LazyLock<IfName> = LazyLock::new(|| "enx".try_into().unwrap());
    static LLADDR: LazyLock<LinkLevelAddress> =
        LazyLock::new(|| "00:11:22:33:44:55".parse().unwrap());

    #[test]
    fn test_open_socket_get_lladdr() {
        sys::mockdown()
            .expect(|args| {
                assert_eq!(sys::Socket(libc::AF_LOCAL, libc::SOCK_DGRAM, 0), args);
                MOCK_FD
            })
            .expect(|sys::IoCtl(args, ifreq)| {
                let ifreq = ifreq::from_mut_ptr(ifreq);
                assert_eq!((MOCK_FD, sys::SIOCGIFLLADDR), args);
                assert_eq!(ifreq::get_name(ifreq), *IFNAME);
                ifreq::set_lladdr(ifreq, &LLADDR);
                0
            })
            .expect(|args| {
                assert_eq!(sys::Close(MOCK_FD), args);
                0
            });

        let mut ifreq = ifreq::new();
        ifreq::set_name(&mut ifreq, &IFNAME);

        super::open_local_dgram()
            .unwrap()
            .get_lladdr(ifreq::as_mut_ptr(&mut ifreq))
            .unwrap();

        assert_eq!(ifreq::get_lladdr(&ifreq), *LLADDR);
    }

    #[test]
    fn test_open_socket_get_lladdr_error() {
        sys::mockdown()
            .expect(|args| {
                assert_eq!(sys::Socket(libc::AF_LOCAL, libc::SOCK_DGRAM, 0), args);
                MOCK_FD
            })
            .expect(|sys::IoCtl(args, ifreq)| {
                let ifreq = ifreq::from_mut_ptr(ifreq);
                assert_eq!((MOCK_FD, sys::SIOCGIFLLADDR), args);
                assert_eq!(ifreq::get_name(ifreq), *IFNAME);
                -1
            })
            .expect(|_: sys::ErrNo| {
                assert!(true);
                libc::EBADF
            })
            .expect(|args| {
                assert_eq!(sys::Close(MOCK_FD), args);
                0
            });

        let expected_error = "9";

        let mut ifreq = ifreq::new();
        ifreq::set_name(&mut ifreq, &IFNAME);

        let error = super::open_local_dgram()
            .unwrap()
            .get_lladdr(ifreq::as_mut_ptr(&mut ifreq))
            .unwrap_err();

        assert_eq!(format!("{}", error), expected_error);
    }
}
```

[latest]: https://img.shields.io/crates/v/mockdown.svg
[crates.io]: https://crates.io/crates/mockdown
[msrv]: https://img.shields.io/crates/msrv/mockdown
[docs.rs]: https://img.shields.io/docsrs/mockdown?label=docs.rs
[docs.rs/mockdown]: https://docs.rs/mockdown/
[build]: https://github.com/fmierlo/mockdown/workflows/test/badge.svg

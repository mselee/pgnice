use crate::ioprio::{BePriorityLevel, Class, RtPriorityLevel};
use nix::libc::rlim_t;
use nix::sys::resource::Resource;
use pgrx::{error, PostgresType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PostgresType)]
#[doc(hidden)]
pub struct ResourceLimit {
    soft: u64,
    hard: u64,
}

impl From<(rlim_t, rlim_t)> for ResourceLimit {
    fn from(value: (rlim_t, rlim_t)) -> Self {
        Self {
            soft: value.0,
            hard: value.1,
        }
    }
}

pub fn to_resource(name: &str) -> Resource {
    match name {
        #[cfg(any(target_os = "android", target_os = "linux"))]
        "nice" => Resource::RLIMIT_NICE,
        #[cfg(any(
            target_os = "android",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "linux",
        ))]
        "rss" => Resource::RLIMIT_RSS,
        #[cfg(any(
            target_os = "android",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "linux",
            target_os = "netbsd"
        ))]
        "memlock" => Resource::RLIMIT_MEMLOCK,
        "stack" => Resource::RLIMIT_STACK,
        "data" => Resource::RLIMIT_DATA,
        "fsize" => Resource::RLIMIT_FSIZE,
        "cpu" => Resource::RLIMIT_CPU,
        "core" => Resource::RLIMIT_CORE,
        #[cfg(not(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd")))]
        "as" => Resource::RLIMIT_AS,
        #[cfg(any(target_os = "android", target_os = "linux"))]
        "locks" => Resource::RLIMIT_LOCKS,
        #[cfg(any(target_os = "android", target_os = "linux"))]
        "msgqueue" => Resource::RLIMIT_MSGQUEUE,
        "nofile" => Resource::RLIMIT_NOFILE,
        #[cfg(any(
            target_os = "android",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "linux",
        ))]
        "nproc" => Resource::RLIMIT_NPROC,
        #[cfg(any(target_os = "android", target_os = "linux"))]
        "rtprio" => Resource::RLIMIT_RTPRIO,
        #[cfg(any(target_os = "linux"))]
        "rttime" => Resource::RLIMIT_RTTIME,
        #[cfg(any(target_os = "android", target_os = "linux"))]
        "sigpending" => Resource::RLIMIT_SIGPENDING,
        #[cfg(target_os = "freebsd")]
        "kqueues" => Resource::RLIMIT_KQUEUES,
        #[cfg(target_os = "freebsd")]
        "npts" => Resource::RLIMIT_NPTS,
        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
        "sbsize" => Resource::RLIMIT_SBSIZE,
        #[cfg(target_os = "freebsd")]
        "swap" => Resource::RLIMIT_SWAP,
        #[cfg(target_os = "freebsd")]
        "vmem" => Resource::RLIMIT_VMEM,
        _ => unreachable!(),
    }
}

pub fn to_class(class: char, level: i32) -> Class {
    match class {
        'R' => Class::Realtime(RtPriorityLevel::from_level(level as u8).unwrap()),
        'B' => Class::BestEffort(BePriorityLevel::from_level(level as u8).unwrap()),
        'I' => Class::Idle,
        _ => unreachable!(),
    }
}

pub fn handle_result<T>(result: nix::Result<T>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            error!("{}", err)
        }
    }
}

//! This is a postgres extension for managing process-specific priorities, such as:
//! - nice
//! - ionice
//! - rlimit
//!
//! Unless the user was given proper privileges, most OS (default) configurations will only allow you to de-prioritize/lower `nice`/`RLIMIT_*` values
//! but not prioritize/increase them again. This is because the OS doesn't keep track of which user performed the initial adjustment.
//!
//! So if these limits are supposed to be set for each backend process depending on the connecting client — and you're reusing connections between these clients (i.e. connection pooling) —
//! you might have to configure different pools each with their own set of limits (as it is still the same process and it is likely the user doesn't have enough privileges to raise them again).
//!
//! In the case of pgbouncer, you will need to set a different `connect_query` for each pool to apply the desired limits.
//!
pub mod ionice;
mod ioprio;
pub mod nice;
pub mod rlimit;
mod utils;

use crate::ioprio::{get_priority, set_priority, Priority, Target};
use ioprio::{BePriorityLevel, Class, RtPriorityLevel};
use nix::errno::Errno;
use nix::libc::{getpriority, setpriority, PRIO_PROCESS};
use nix::sys::resource::{getrlimit, rlim_t, setrlimit, Resource};
use nix::unistd::Pid;
use pgrx::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::c_int;
use utils::ResourceLimit;

pg_module_magic!();

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::ioprio::{BePriorityLevel, Class};
    use pgrx::prelude::*;

    #[pg_test]
    fn test_set_nice() {
        let expected = 15;
        crate::nice::pgnice_set_backend_nice(expected);
        let current = crate::nice::pgnice_get_backend_nice();
        assert_eq!(expected, current);
    }

    #[pg_test]
    fn test_get_ionice() {
        let expected = Class::BestEffort(BePriorityLevel::from_level(7).unwrap());
        crate::ionice::pgnice_set_backend_ionice('B', 7);
        let current = crate::ionice::pgnice_get_backend_ionice();
        assert_eq!(expected, current);
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}

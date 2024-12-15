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
//! Available functions:
//! - `pgnice.get_backend_ionice()`
//! - `pgnice.set_backend_ionice(class, level)`
//! - `pgnice.get_backend_nice()`
//! - `pgnice.set_backend_nice(prio)`
//! - `pgnice.get_backend_rlimit(name)`
//! - `pgnice.set_backend_rlimit(name, limit)`
pub mod ionice;
mod ioprio;
pub mod nice;
pub mod rlimit;
mod utils;

use pgrx::prelude::*;
use utils::ResourceLimit;

pg_module_magic!();

#[pg_schema]
mod pgnice {
    use super::*;

    #[pg_extern]
    pub fn get_backend_ionice() -> &'static str {
        crate::ionice::pgnice_get_backend_ionice()
    }

    #[pg_extern]
    pub fn set_backend_ionice(class: char, level: i32) {
        crate::ionice::pgnice_set_backend_ionice(class, level)
    }

    #[pg_extern]
    pub fn get_backend_nice() -> i32 {
        crate::nice::pgnice_get_backend_nice()
    }

    #[pg_extern]
    pub fn set_backend_nice(prio: i32) {
        crate::nice::pgnice_set_backend_nice(prio);
    }

    #[pg_extern]
    pub fn get_backend_rlimit(name: &str) -> ResourceLimit {
        crate::rlimit::pgnice_get_backend_rlimit(name)
    }

    #[pg_extern]
    pub fn set_backend_rlimit(name: &str, limit: i64) {
        crate::rlimit::pgnice_set_backend_rlimit(name, limit);
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_set_nice() {
        let expected = 15;
        crate::pgnice::set_backend_nice(expected);
        let current = crate::pgnice::get_backend_nice();
        assert_eq!(expected, current);
    }

    #[pg_test]
    fn test_get_ionice() {
        let expected = "best-effort: 7";
        crate::pgnice::set_backend_ionice('B', 7);
        let current = crate::pgnice::get_backend_ionice();
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

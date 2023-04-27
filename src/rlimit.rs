use crate::utils;
use crate::utils::ResourceLimit;
use nix::libc::rlim_t;
use nix::sys::resource::{getrlimit, setrlimit, Resource};
use pgrx::pg_extern;

fn set_process_resource(resource: Resource, soft: i64, hard: i64) -> nix::Result<()> {
    setrlimit(resource, soft as rlim_t, hard as rlim_t)
}

fn get_process_resource(resource: Resource) -> nix::Result<ResourceLimit> {
    getrlimit(resource).map(|value| ResourceLimit::from(value))
}

/// Retrieves the resource limits of the current backend process.
/// See [getrlimit(2)](https://man7.org/linux/man-pages/man2/getrlimit.2.html).
///
/// # Arguments
///
/// * `name` - The resource name. Allowed values are:
///     - nice
///     - rss
///     - memlock
///     - stack
///     - data
///     - fsize
///     - cpu
///     - core
///     - as
///     - locks
///     - msgqueue
///     - nofile
///     - nproc
///     - rtprio
///     - rttime
///     - sigpending
///     - kqueues
///     - npts
///     - sbsize
///     - swap
///     - vmem
///
/// # Returns
///
/// The limit of the current backend process as json.
///
/// # Safety
///
/// If the underlying call to `getrlimit(2)` fails, the current transaction will be aborted.
///
/// # Examples
///
/// ```sql
/// SELECT pgnice_get_backend_rlimit('data');
/// {"soft":18446744073709551615,"hard":18446744073709551615}
/// ```
#[pg_extern]
pub fn pgnice_get_backend_rlimit(name: &'static str) -> ResourceLimit {
    let result = get_process_resource(utils::to_resource(name));
    utils::handle_result(result)
}

/// Sets the resource limits of the current backend process.
/// See [setrlimit(2)](https://man7.org/linux/man-pages/man2/setrlimit.2.html).
///
/// # Arguments
///
/// * `name` - The resource name. Allowed values are:
///     - nice
///     - rss
///     - memlock
///     - stack
///     - data
///     - fsize
///     - cpu
///     - core
///     - as
///     - locks
///     - msgqueue
///     - nofile
///     - nproc
///     - rtprio
///     - rttime
///     - sigpending
///     - kqueues
///     - npts
///     - sbsize
///     - swap
///     - vmem
/// * `limit` - The new limit for the resource.
///
/// # Safety
///
/// If the underlying call to `setrlimit(2)` fails, the current transaction will be aborted.
///
/// # Examples
///
/// ```sql
/// SELECT pgnice_set_backend_rlimit('data', 1024 * 50);
/// ```
#[pg_extern]
pub fn pgnice_set_backend_rlimit(name: &'static str, limit: i64) {
    let result = set_process_resource(utils::to_resource(name), limit, limit);
    utils::handle_result(result);
}

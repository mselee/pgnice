use nix::errno::Errno;
use nix::libc::{getpriority, setpriority, PRIO_PROCESS};
use std::ffi::c_int;

fn get_process_nice() -> nix::Result<i32> {
    let ret = unsafe { getpriority(PRIO_PROCESS, 0) };
    Errno::result(ret)
}

fn set_process_nice(prio: i32) -> nix::Result<()> {
    let ret = unsafe { setpriority(PRIO_PROCESS, 0, prio as c_int) };
    Errno::result(ret).map(drop)
}

/// Retrieves the nice value of the current backend process.
/// See [getpriority(2)](https://man7.org/linux/man-pages/man2/getpriority.2.html).
///
/// # Returns
///
/// The nice value of the current backend process as an integer.
///
/// # Safety
///
/// If the underlying call to `getpriority(2)` fails, the current transaction will be aborted.
///
/// # Examples
///
/// ```sql
/// SELECT pgnice_get_backend_nice();
/// 15
/// ```
///
#[inline(always)]
pub fn pgnice_get_backend_nice() -> i32 {
    let result = get_process_nice();
    crate::utils::handle_result(result)
}

/// Sets the nice value of the current backend process.
/// See [setpriority(2)](https://man7.org/linux/man-pages/man2/setpriority.2.html).
///
/// # Arguments
///
/// * `prio` - The new priority value for the process.
///
/// # Safety
///
/// If the underlying call to `setpriority(2)` fails, the current transaction will be aborted.
///
/// # Examples
///
/// ```sql
/// SELECT pgnice_set_backend_nice(15);
/// ```
#[inline(always)]
pub fn pgnice_set_backend_nice(prio: i32) {
    let result = set_process_nice(prio);
    crate::utils::handle_result(result);
}

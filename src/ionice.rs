use crate::ioprio::{get_priority, set_priority, Class, Priority, Target};
use crate::utils;
use nix::unistd::Pid;
use pgrx::pg_extern;

fn get_process_ionice() -> nix::Result<Priority> {
    let target = Target::Process(Pid::from_raw(0));
    let ret = get_priority(target);
    ret
}

fn set_process_ionice(class: char, level: i32) -> nix::Result<()> {
    let class = utils::to_class(class, level);
    let target = Target::Process(Pid::from_raw(0));
    let ret = set_priority(target, Priority::new(class));
    ret
}

/// Retrieves the ionice value of the current backend process.
/// See [ioprio_get(2)](https://man7.org/linux/man-pages/man2/ioprio_get.2.html).
///
/// # Returns
///
/// The ionice value of the current backend process as json.
///
/// # Safety
///
/// If the underlying call to `ioprio_get(2)` fails, the current transaction will be aborted.
///
/// # Examples
///
/// ```sql
/// SELECT pgnice_get_backend_ionice();
/// {"class":"best-effort","level":0}
/// ```
#[pg_extern]
pub fn pgnice_get_backend_ionice() -> &'static str {
    let result = get_process_ionice();
    match utils::handle_result(result).class() {
        Class::Realtime(priority) => match priority.level() {
            0 => "real-time: 0",
            1 => "real-time: 1",
            2 => "real-time: 2",
            3 => "real-time: 3",
            4 => "real-time: 4",
            5 => "real-time: 5",
            6 => "real-time: 6",
            7 => "real-time: 7",
            _ => unreachable!(),
        },
        Class::BestEffort(priority) => match priority.level() {
            0 => "best-effort: 0",
            1 => "best-effort: 1",
            2 => "best-effort: 2",
            3 => "best-effort: 3",
            4 => "best-effort: 4",
            5 => "best-effort: 5",
            6 => "best-effort: 6",
            7 => "best-effort: 7",
            _ => unreachable!(),
        },
        Class::Idle => "idle",
        Class::None => "none",
    }
}

/// Sets the ionice value of the current backend process.
/// See [ioprio_set(2)](https://man7.org/linux/man-pages/man2/ioprio_set.2.html).
///
/// # Arguments
///
/// * `class` - The new priority class for the process. Allowed values are:
///     - `I`: idle
///     - `B`: best-effort
///     - `R`: real-time
/// * `level` - The new priority level for the process. This param is ignored if the `class` is `I`.
///
/// # Safety
///
/// If the underlying call to `ioprio_set(2)` fails, the current transaction will be aborted.
///
/// # Examples
///
/// ```sql
/// SELECT pgnice_set_backend_ionice('B', 0);
/// ```
#[pg_extern]
pub fn pgnice_set_backend_ionice(class: char, level: i32) {
    let result = set_process_ionice(class, level);
    utils::handle_result(result);
}

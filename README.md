# pgnice

This is a postgres extension for managing process-specific priorities, such as:
- nice
- ionice
- rlimit

Unless the user was given proper privileges, most OS (default) configurations will only allow you to de-prioritize/lower `nice`/`RLIMIT_*` values
but not prioritize/increase them again. This is because the OS doesn't keep track of which user performed the initial adjustment.

So if these limits are supposed to be set for each backend process depending on the connecting client — and you're reusing connections between these clients (i.e. connection pooling) —
you might have to configure different pools each with their own set of limits (as it is still the same process and it is likely the user doesn't have enough privileges to raise them again).

In the case of pgbouncer, you will need to set a different `connect_query` for each pool to apply the desired limits.

Available functions:
- `pgnice.get_backend_ionice()`
- `pgnice.set_backend_ionice(class, level)`
- `pgnice.get_backend_nice()`
- `pgnice.set_backend_nice(prio)`
- `pgnice.get_backend_rlimit(name)`
- `pgnice.set_backend_rlimit(name, limit)`

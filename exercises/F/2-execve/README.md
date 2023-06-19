# Executing programs with `execve` 

Try to run this:

```sh
> cargo build --bin log
> cargo run
```

This should show the following

```
arguments:


variables:
```

This program executed the `log` binary, which prints its arguments and environment variables. The signature of `execve` is

    int execve(const char *pathname, char *const argv[], char *const envp[]);

- pathname is a null-terminated string
- argv is a null-terminated array of null-terminated strings
- envp is a null-terminated array of null-terminated strings, its elements look like "KEY=value"

When doing this exercise, be careful that sequences have null-termination in the right place: strings should end in a NULL byte, arrays should end in a NULL pointer.

# 1. pass the executable path

The first argument to an executable should be the path to itself. We already have the `executable` CString. You can get a raw pointer from a CString with the `as_ptr` function. `as_ptr` also works on slices and vectors. Again, both strings and arrays must be NULL-terminated, and the CString type already guarantees that its bytes end in a NULL byte.

If the argument is supplied correctly, `cargo run` should show something similar to

```
arguments:
  /home/folkertdev/tg/101-rs/target/debug/log

variables:
```

# 2. pass an environment variable

Create a new `CString` that represents a path (e.g. `CString::new("FOO=bar")`), and pass it to the executable.

```
arguments:
  /home/folkertdev/tg/101-rs/target/debug/log

variables:
  FOO: bar
```

# 3. forward arguments and environment variables

using `std::env::{args, vars}`, forward all arguments and variables from the current process via execve to the log program.

- the first argument should still be the the path to `log`
- be sure that CString values live long enough (are not de-allocated before the call to execve)

In the end you should see something like this

```txt
> cargo run -- a b c
arguments:
  /home/folkertdev/tg/101-rs/target/debug/F2-execve
  a
  b
  c

variables:
  BASE16_THEME:
  CARGO: /home/folkertdev/.rustup/toolchains/1.64.0-x86_64-unknown-linux-gnu/bin/cargo
  CARGO_HOME: /home/folkertdev/.cargo
  CARGO_MANIFEST_DIR: /home/folkertdev/tg/101-rs/exercises/F2-execve
```

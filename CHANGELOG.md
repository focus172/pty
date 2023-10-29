### 0.3.0
* Updated to Rust 2021
* API Change: `Fork` must now be matched on to get the inner data.
* `Master` is no longer `io::Read` or `io::Write`.
  * Use the `pty` field on it to get at the inner data.
* Reduced error that can happened after call to `fork`
  * Less likely hood that one half succeed while the other doesn't
* API Addition: added `pty::fork`

### 0.2.0
* Improve the Error Handling.
* Improve the POO representation.
* Remove the **posix_openpt**(3) call.

### 0.1.6

* [#5](https://github.com/hibariya/pty-rs/pull/5) Improve error representation

### 0.1.5

* API Change: [#3 Remove unnecessary `Copy` trait](https://github.com/hibariya/pty-rs/pull/3)
  * Mark `Child#pty` as private, add public `Child#pty()`.
  * Remove `Copy` trait from `Child` and `ChildPTY`.
  * Remove `ChildPTY#fd()`, impl `AsRawFd` for `ChildPTY`.
* Bug fix: [#4 Continue wait if it's still alive](https://github.com/hibariya/pty-rs/pull/4)

### 0.1.4

* API Change: [#2](https://github.com/hibariya/pty-rs/pull/2) Make `pty::fork()` return a single value.

### 0.1.3

* Support stable rust

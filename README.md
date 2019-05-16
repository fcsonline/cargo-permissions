# Cargo Permissions

:warning: This repository is just a POC

TL;DR

Cargo package manager needs a way to detect tampered dependecies.

# What is the problem?

Crates.io, as many other package repositories have the challenge to keep all
the available packages in a reliable and secure way. Developers and users
of these repositories put a lot of confidence in repository maintainers.

To keep a healthy repository of packages in crates.io we need to enforce as
many as possible approaches to detect any kind of vulnerability.

With the increased use of dependencies between packages, the risk of
vulnerability propagation increases. A small security problem in a famous
crate can lead to a huge problem in many projects. We have seen many security
problems like this one in other platforms like NPM.

Rust developers need a tool to answer those questions about their dependencies:

- Why a png library uses the network layer?
- Why a http library uses the file system layer?

# Proof of concept

The main idea for this project is to have a set of _permissions_ associated
with some specific list of standard packages. On the other hand, through an AST
analysis, check the standard libraries used by a crate. For example, if a crate
starts using `std::net` library, is going to acquire the `net` permission. All
crates that use this other crate as dependency are going to acquire the `net`
permission, indirectly though. This permission acquisition goes up to the last
package.

Following this approach, we can build a dependency tree with all acquired
permissions. This set of permissions are going to give as much information
about packages we don't control.

This package tries to minimize the impact of a known set of risky use cases,
following the approach of source code analysis and dependency tree analysis. We
can take advantage of the static analysis to understand what is going on a
package under the hood.

This approach is inspired by permission systems in different platforms like:

- [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)
- [Feature policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Feature-Policy)
- [Android permissions](https://developer.android.com/guide/topics/permissions/overview)

## Possible scenarios

- Read unauthorized files
- Do requests to untrusted domains
- Execute unauthorized programs
- Stolen information
- Stolen CPU resources
- Execute code unsafely

## Proposed set of permissions

The list of available permissions is not closed but this is a starting point
for this proof of concept:

- **fs**: This crate uses the standard file system library
- **net**: This crate uses the standard network system library
- **io**: This crate uses the standard io system library
- **process**: This crate uses the standard process system library
- **thread**: This crate uses the standard thread system library
- **unsafe**: This crate uses unsafe code

# Example

Imagine you have an application that has as dependency `clap` and `hyper`, and
you want to control which _permissions_ you want to grant them. Then you can
add to the `Cargo.toml` file:

```
[dependencies]
clap = {version = "2", permissions = ["io", "process"]}
hyper = {version = "0.12", permissions = ["net"]}
```

If that for whatever reason, `clap` starts using a permission like `net` that
is not authorized to use, we are going going to raise red flags about the
version used for the `clap` crate.

```
$ cargo build
   Compiling memchr v2.2.0
   Compiling remove_dir_all v0.5.1
   Compiling termbox-sys v0.2.11
   Compiling cfg-if v0.1.6
   Compiling ucd-util v0.1.3
   Compiling lazy_static v1.3.0
   Compiling unicode-width v0.1.5
   ...
   Compiling my-package v0.0.1 (/home/user/my-package)

   Compilation Failed!

   `clap` package is not authorized to use `net` layer
```

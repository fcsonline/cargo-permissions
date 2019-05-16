# Cargo Permissions

:warning: This repository is just a POC

TLDR;

Cargo package manager needs a way to detect tampered dependecies.

# Which is the problem?

Crates.io, as many other package repositories have the challenge to keep all
the available packages in a reliable and secure way. Developers and users
of these repositories put a lot of confidence in repository maintainers.

With the increased use of dependencies between packages, the risk of
vulnerability propagation increases. A small security problem in a famous
crate can lead to a huge problem in many projects. We have seen many security
problems like this one in other platforms like NPM.

- Why a png library uses the network layer?
- Why a http library uses the file system layer?

# Introduction

One of the core principles of Rust is safety. To keep a healthy repository of
packages in crates.io we need to enforce as many as possible approaches to
detect any kind of vulnerability.

This package tries to minimize the impact of a known set of risky use cases,
following the approach of source code analysis and dependency tree analysis. We
can take advantage of the static analysis to understand what is going on a
package under the hood.

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

With those permissions, we can answer those questions:

This approach is inspired by permission systems in different platforms like:

- [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)
- [Feature policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Feature-Policy)
- [Android permissions](https://developer.android.com/guide/topics/permissions/overview)

## Risky use cases

- Read unauthorized files
- Do requests to untrusted domains
- Execute unauthorized programs
- Stolen information
- Stolen CPU resources
- Execute code unsafely

## Permissions

- **fs**: This crate uses the standard file system library
- **net**: This crate uses the standard network system library
- **io**: This crate uses the standard io system library
- **process**: This crate uses the standard process system library
- **thread**: This crate uses the standard thread system library
- **unsafe**: This crate uses unsafe code

## Configuration

Imagine you have an application that has as dependency `clap` and `hyper`, and
you want to control which _permissions_ you want to grant them. Then you can
add to the `Cargo.toml` file:

```
[permissions]
default = ["self"]
io = ["clap"]
process = ["clap"]
net = ["hyper"]
```

If that for whatever reason, `clap` starts using a permission like `net` that
is not authorized to use, we are going going to raise red flags about the
version used for the `clap` crate.

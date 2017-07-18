Baler downloads semantic web fragments with declared dependencies and builds transitive closures.
It can also execute test suites to support invariant traversal behavior across versions.

## Compiling from Source

Baler requires the following tools and packages to build:

* `python`
* `curl` (on Unix)
* `cmake`
* OpenSSL headers (only for Unix, this is the `libssl-dev` package on ubuntu)
* `cargo` and `rustc`

First, you'll want to check out this repository

```
git clone --recursive https://github.com/MultiscaleHN/baler.git
cd baler
```

With `cargo` already installed, you can simply run:

```
cargo build --release
```

## Running the tests

To run baler's tests, use `cargo test`.

## Contributing to the Docs

To contribute to the docs, all you need to do is change the markdown files in
the `src/doc` directory. To view the rendered version of changes you have
made locally, run:

```sh
sh src/ci/dox.sh
open target/doc/index.html
```

## Releases

No releases have been made yet.

## Reporting Issues

Found a bug? We'd love to know about it!

Please report all issues on the github [issue tracker][issues].

[issues]: https://github.com/MultiscaleHN/baler/issues

## License

Baler is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

### Third party software

This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit (http://www.openssl.org/).

In binary form, this product includes software that is licensed under the
terms of the GNU General Public License, version 2, with a linking exception,
which can be obtained from the [upstream repository][1].

[1]: https://github.com/libgit2/libgit2


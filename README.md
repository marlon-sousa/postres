# postres

Convert a Postman collection into `.http` files for the
[VS Code REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)
extension.

## Status

**Nothing works yet.** This repository was reset to an empty tree in September 2026
and is being written again from scratch, one part at a time. This README is the
first commit.

The first attempt, written in 2022, is kept as a record: the
[`archive/2022`](https://github.com/marlon-sousa/postres/tree/archive/2022) branch,
tagged [`defs-2022`](https://github.com/marlon-sousa/postres/tree/defs-2022).
Nothing will ever be pushed to it again.

## Built in the open

postres began in 2022 as a Rust tutorial hidden inside a working program: thirty-seven
numbered blocks of prose, written as code comments, for programmers who had been told
that Rust was not for them. Four years later it had no stars, no forks, no issues and
no readers — and the program had never actually written a file. The teaching was fine.
The container was wrong: nobody clones a repository to read a tutorial.

So it is being built again, in the open, from an empty repository. One article per
part, each one ending at a git tag, so you can check out exactly the code an article
describes and run it. The series is
**[Rust Beyond Systems Programming](https://marlon-sousa.com/series/rust-beyond-systems/)**.

| Part | Article | Tag |
| ---- | ------- | --- |
| _Parts are listed here as they are published._ | | |

## Building it

There is nothing to build yet. Once there is:

```sh
git clone https://github.com/marlon-sousa/postres.git
cd postres
cargo build --release
```

`cargo install postres` and prebuilt binaries for Linux, Windows and macOS arrive with
the final part.

## Usage

This is the interface the rebuild is aiming at. None of it runs today.

```sh
# one file, named after the collection
postres --postman-file collection.json

# one file, named by you
postres --postman-file collection.json --output-file requests.http

# one file per folder in the collection
postres --postman-file collection.json --output-dir ./requests/
```

## License

MIT — see [LICENSE](LICENSE).

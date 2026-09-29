# postres

Convert a Postman collection into `.http` files for the
[VS Code REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)
extension.

## Status

**It listens, and converts nothing yet.** This repository was reset to an empty tree
in September 2026 and is being written again from scratch, one part at a time. At
`part-05` it reads its command line and prints back what it understood. It does not
read or write a file yet. Every push is checked on Linux, Windows and macOS.

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

The first three parts are about the language itself and carry no code, so the table
starts at the fourth.

| Part | Article | Tag |
| ---- | ------- | --- |
| 4 | [A converter is a small compiler](https://marlon-sousa.com/blog/a-converter-is-a-small-compiler/) | [`part-04`](https://github.com/marlon-sousa/postres/tree/part-04) |
| 5 | [Arguments, without writing a parser](https://marlon-sousa.com/blog/arguments-without-writing-a-parser/) | [`part-05`](https://github.com/marlon-sousa/postres/tree/part-05) |

## Building it

Rust 1.85 or newer:

```sh
git clone https://github.com/marlon-sousa/postres.git
cd postres
cargo build --release
```

`cargo install postres` and prebuilt binaries for Linux, Windows and macOS arrive with
the final part.

## Usage

This is the interface the rebuild is aiming at. The first two commands already parse,
and print back what they understood; none of them converts anything yet.

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

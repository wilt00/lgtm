# lgtm

[![CI](https://github.com/wilt00/lgtm/actions/workflows/build.yml/badge.svg)](https://github.com/wilt00/lgtm/actions/workflows/build.yml)

We've all wondered - what does LGTM stand for? Who can say. Truly, a mystery.

But now, you'll never need to wonder again; generate your very own LGTMs with this, the world's first LGTMaaS platform. 

Try it for yourself!

```
$ curl lgtm.halsted.space
```

Fresh LGTMs served up piping hot, on demand, as soon as the Heroku free tier dyno spins up. Give it a sec. You think I'm spending money on this?

Also now available right here on the World Wide Web, in shiny Webassembly form, at https://wilt00.github.io/lgtm/ (thanks [@bmiddha](https://github.com/bmiddha)!)

Words lists are compiled by Infochimps, via https://github.com/dwyl/english-words

## About the project

### Rust

The rust code is separated into 3 cargo workspaces:

- `api` - HTTP API that returns plain text `lgtm` responses.
- `lgtm` - Library used by other projects (`api`, `cli`, `web`) to generate `lgtm`s.
- `cli` - Binary that prints `lgtm` to STDOUT.

All projects can be built and tested from the root of the repo using `cargo build` and `cargo test`. Projects can also be individually interacted with by navigating to their directories and running `cargo` commands or by passing the `-p` flag. For more information about cargo workspaces, refer to the [documentation](https://doc.rust-lang.org/cargo/reference/workspaces.html).

### Wasm

The `lgtm` library is also built to WebAssembly to be used in the `web` project.

`wasm-pack` is needed to build. Available from [here](https://rustwasm.github.io/).

### Web

The `web` project follows a build-less system. No bundling or installing nodejs is required, just a web server.

The project uses the `wasm` generated by `wasm-pack` and provides a web based frontend for it.


## Building `wasm`

```sh
# from lgtm directory
wasm-pack build --release --target web -d ../web/lgtm
# this will generate a folder repo/web/lgtm with the built wasm and js glue-code
```

## Building and testing Rust projects

```sh
# from repo/, build all
cargo build
# from repo/, build <project>
cargo build -p <project>
# from repo/<project>, build <project>
cargo build

# same commands apply for testing, replace build with test
```

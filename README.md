![Gemino Project](https://i.imgur.com/UjWAjDZ.png)

# About

Gemino is a workflow helper to get started with a React SPA.

- Quickly build a new React SPA in a Lerna managed environment

Future plans:

- Read Lerna config and install packages to the appropriate folder
- Integrate more tasks for orchestrating JavaScript builds
- Integrate helpers for other types of projects like utility libraries, styles, and more

# Commands

## Scaffolding a New React SPA

Scaffolds a new React SPA with 'project name' being the name of the folder to create.

Gemino will create a simple application that inherits from the SPA template with a simple component and test.

```
gemino new project-name
```

Jest and Babel will inherit from the global Jest and Babel configuration but you can override or add any configuration you need for your application by simpling overriding a key (or modifying the key in place).


# Development

## Installing Tools

Run:

- `brew install rustup`
- `rustup install stable`
- `rustup component add rustfmt`

You should be able to run:

```
rustc --version
cargo --version
```

## Building

Simply run:

```
cargo build ${RELEASE_TARGET}
```

Substitute `RELEASE_TARGET` for the target you wish to build, or leave it empty for a debugging build.

# Issues

Please open an issue on this repo!

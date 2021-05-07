# fnd

`fnd` is a tiny CLI to find file paths based on substring matching or regular
expressions. It honors gitignore files for file traversal.

## Motivation

Honestly? I forget the syntax for `find`, and I don't want to drill in the
muscle memory now to learn it.

Separately, `find` is a bit slow for my liking, so being able to use multiple
threads is nice.

## Usage

### Search for all files containing the string 'sql'

```sh
fnd sql
```

### Search for all files containing the string 'sql' while ignoring gitignores

```sh
fnd sql -a
```

### Search for all files matching a regex

```sh
fnd "\.xlsx?$" -r
```

## Installation

### Cargo

```sh
cargo install --path .
```

## License

Copyright 2021 Josh Clayton. See the [LICENSE](LICENSE).

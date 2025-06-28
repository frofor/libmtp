# Contributing

## Branches

- `main`

Contains code that is in active development and may contain bugs or unfinished features.

*Rationale:* Intended for testing and gathering user feedback prior to stable release.

- `stable`

Contains code that has been thoroughly tested and received positive user feedback.

*Rationale:* Intended for production use, ensuring reliability.

## Getting started

Fork this repository on Codeberg and clone it:

```sh
$ git clone https://codeberg.org/<user>/libmtp.git
```

Setup the `upstream` remote for pulling in new changes:

```sh
$ git remote add upstream https://codeberg.org/frofor/libmtp.git
$ git pull --rebase upstream main
```

Create and checkout a branch, where you will commit your changes:

```sh
$ git checkout main -b <descriptive-name>
```

## Commiting changes

Commit messages should follow the Conventional Commits specification:

```
<type>[(scope)]: <description>

[body]

[footer(s)]
```

Allowed types:
- `feat`: For new features.
- `fix`: For bug fixes.
- `docs`: For changes to documentation.
- `test`: For changes to tests.
- `chore`: For changes to the build process or auxiliary tools and libraries.

Allowed scopes (optional):
- `ffi`: Changes to FFI bindings.
- `api`: Changes to public API.

## Opening a Pull Request

After commiting your changes open the [pull request](https://codeberg.org/frofor/libmtp/pulls).

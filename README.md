# On a Roll

<!--toc:start-->

- [On a Roll](#on-a-roll)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installing](#installing)
  - [Development Practices](#development-practices)
    - [Conventional Commits](#conventional-commits)
    - [Pre-commit Hooks](#pre-commit-hooks)
  - [Contributing](#contributing)
  - [License](#license)
  <!--toc:end-->

On a Roll is a CLI task manager written in Rust that makes you feel like you are
on a roll today.

It uses Diesel for database interactions and SQLite as the database engine,
ensuring lightweight and efficient data management.

## Getting Started

These instructions will get you a copy of the project up and running on your
local machine for development and testing purposes.

### Prerequisites

- Rust Programming Language

- SQLite

- Diesel CLI

### Installing

First, clone the repository:

```sh
git clone https://github.com/donbignose/on-a-roll.git
cd on-a-roll
```

Install the Diesel CLI, which is necessary for handling database migrations:

```sh
cargo install diesel_cli --no-default-features --features sqlite
```

Set up the database by running Diesel migrations:

```sh
diesel setup
```

Now, you can build and run the project:

```sh
cargo build
cargo run
```

## Development Practices

### Conventional Commits

We use Commitizen and follow the Conventional Commits specification for our
commit messages. This leads to more readable messages that are easy to follow
when looking through the project history. It also allows us to automatically
generate a changelog.

#### Using Commitizen

We recommend using Commitizen for crafting your commit messages. It's a tool
that helps you write consistent and conventional commit messages. To use
Commitizen, run `cz commit` in your terminal instead of `git commit`. This
project includes a `.cz.toml` configuration file to facilitate Commitizen's
usage.

### Pre-commit Hooks

Our project employs pre-commit hooks to ensure code quality and adherence to
standards. Before committing, hooks for code formatting and linting are run,
helping to maintain a clean codebase.

#### Setting Up Pre-commit Hooks

To set up pre-commit hooks locally, run the following command:

```sh
pre-commit install
```

This command sets up hooks in your local repository, running checks such as
rustfmt and Clippy before each commit.

## Contributing

We welcome contributions from the community! Please read our contributing
guidelines and code of conduct before making pull requests.

## License

This project is licensed under the MIT License - see the file for details.

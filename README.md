# Password Generator

Simple and fast CLI password generator made in Rust.

## Usage

For build instructions, see [Installation](#installation)

### Linux and MacOS

From within it's parent directory:
```
./passgen [OPTION]... [LENGTH]
```

Example usages:
```
./passgen -ul 12
./passgen -luns 20
```

### Windows

From within it's parent directory:
```
.\passgen.exe [OPTION]... [LENGTH]
```

Example usages:
```
.\passgen.exe -ul 12
.\passgen.exe -luns 20
```

## Installation

Clone the repository from GitHub, then descend into it:
```
git clone https://github.com/ir1descent1/password_generator.git
cd password_generator
```

After having [installed Rust](https://www.rust-lang.org/tools/install), run from within the `password_generator` directory:
```
cargo build
```

This will build a script in `target/debug` called `password_generator`. From there, you can make this script executable, rename it to `passgen` or `passgen.exe`, and then move it to a convenient location within your filesystem.

## Note

Commits for this repository were originally pushed on December 2, 2022. Since then, I have changed my Github username to protect my privacy. This change did not propagate to the commit history for all of my repositories. Thus, I pulled the code, removed the commit history, and uploaded it under my new username.
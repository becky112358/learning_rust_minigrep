# minigrep

A project for learning Rust.

## Project Reference

The Rust Programming Language Chapter 12.

## Technologies

* Rust

## Usage

Input text to be searched for and a file to search within. For example:

```cargo run the poem.txt```

With this example, the file ```poem.txt``` will be searched and all lines containing ```the``` will be printed.

### Additional Options

By default, searches will be case sensitive. Case sensitivity can be switched off using an environment variable, as follows:

```CASE_INSENSITIVE=true cargo run are poem.txt```

## Limitations

* Does not support regular expression.


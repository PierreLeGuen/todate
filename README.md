# `todate`

A command-line utility that converts UNIX timestamps into human-readable dates.

Example:

```bash
➜ todate 1697745797056
2023-10-19T22:03:17.056+02:00
```


## Table of Contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)

## Description

The `todate` utility is a simple yet powerful tool designed to make the task of converting UNIX timestamps to standard date formats effortless. Whether you are debugging, logging, or just curious, `todate` has you covered.

## Installation

### Via `cargo`

If you have Rust's package manager `cargo` installed, you can quickly install `todate`:

```bash
cargo install todate
```

### From Source

Clone the repository:

```bash
https://github.com/PierreLeGuen/todate.git
```

Navigate to the project directory:

```
cd todate
```

Build and install

```bash
cargo install --path .
``` 

## Usage

To convert a UNIX timestamp to a human-readable date:

```bash
todate <timestamp>
```

## Features

- Supports timestamps in seconds, milliseconds, or nanoseconds.
- Automatically detects the precision of the provided timestamp.
- Displays the date and time in the system's local timezone.

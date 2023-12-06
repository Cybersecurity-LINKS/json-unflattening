# json-flattening

![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)
[![](https://img.shields.io/crates/v/json-flattening?style=flat-square)](https://crates.io/crates/json-flattening)
[![](https://img.shields.io/docsrs/json-flattening?style=flat-square)](https://docs.rs/json-flattening/)

A Rust library for flattening and unflattening JSON structures. Uses `serde_json` for JSON serialization and deserialization.

## Features

- **Flatten JSON**: Convert nested JSON structures into a flattened form.
- **Unflatten JSON**: Convert flattened JSON structures back to nested form.

## Installation

Add this library to your `Cargo.toml`:

```toml
[dependencies]
json-flattening = "0.1.0"
```


## Usage

```rust
use json_flattening::{flatten, unflatten};

fn main() {
    let input_json = json!({
        "name": {
            "first": "John",
            "last": "Doe"
        },
        "age": 30,
        "city": "New York"
    });

    let flattened_json = flatten(&input_json).unwrap();
    println!("Flattened JSON: {:#}", serde_json::to_string_pretty(&flattened_json).unwrap());

    let unflattened_json = unflatten(&flattened_json).unwrap();
    println!("Unflattened JSON: {:#}", unflattened_json);
}

```


## Example

### Original JSON

```json
{
  "x": [
    ["y", "z"],
    { "p": "q" },
    ["r", "s"],
    [
      { "u": "v" },
      { "w": "x" }
    ],
    ["y", "z"]
  ]
}

```

### Flattened JSON

```json
{
  "x[0][0]": "y",
  "x[0][1]": "z",
  "x[1].p": "q",
  "x[2][0]": "r",
  "x[2][1]": "s",
  "x[3][0].u": "v",
  "x[3][1].w": "x",
  "x[4][0]": "y",
  "x[4][1]": "z"
}

```

#### Explanation:

1. `"x[0][0]": "y"` and `"x[0][1]": "z"`: The first element of array `x` is itself an array, so we represent its elements with indices.

2. `"x[1].p": "q"`: The second element of array `x` is an object with a key-value pair, so we represent it with dot notation.

3. `"x[2][0]": "r"` and `"x[2][1]": "s"`: The third element of array `x` is again an array, so we use indices.

4. `"x[3][0].u": "v"` and `"x[3][1].w": "x"`: The fourth element of array `x` is an array of objects. We use indices for the outer array and dot notation for the objects.

5. `"x[4][0]": "y"` and `"x[4][1]": "z"`: The fifth element of array `x` is similar to the first, so we again use indices.
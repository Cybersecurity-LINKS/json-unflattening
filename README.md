# json-unflattening

![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)
[![](https://img.shields.io/crates/v/json-unflattening?style=flat-square)](https://crates.io/crates/json-unflattening)
[![](https://img.shields.io/docsrs/json-unflattening?style=flat-square)](https://docs.rs/json-unflattening/)

A Rust library for flattening and unflattening JSON structures. Uses `serde_json` for JSON serialization and deserialization.

## Features

- **Flatten JSON**: Convert nested JSON structures into a flattened form.
- **Unflatten JSON**: Convert flattened JSON structures back to nested form.

## Installation

Add this library to your `Cargo.toml`:

```toml
[dependencies]
json-unflattening = "0.1.3"
```


## Usage

```rust
use json_unflattening::{flatten, unflatten};

fn main() {
    let input_json = json!({
        "name": {
            "first": "John",
            "last": "Doe"
        },
        "age": 30,
        "city": "New York",
        "hobbies": ["Reading", "Hiking", "Gaming"]
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
  "name": {
      "first": "John",
      "last": "Doe"
  },
  "age": 30,
  "city": "New York",
  "hobbies": ["Reading", "Hiking", "Gaming"]
}
```

### Flattened JSON

```json
{
  "name.first": "John",
  "name.last": "Doe",
  "age": 30,
  "city": "New York",
  "hobbies[0]": "Reading",
  "hobbies[1]": "Hiking",
  "hobbies[2]": "Gaming"
}
```

#### Flattening process:

1. **Flatten Object Properties:**
   - Flatten the `"name"` object properties using dot notation: `"name.first"` and `"name.last"`.
   - Flatten the scalar properties `"age"` and `"city"` directly without modification.

   Result:
   ```json
   {
     "name.first": "John",
     "name.last": "Doe",
     "age": 30,
     "city": "New York"
   }
   ```

2. **Flatten Array Elements:**
   - Flatten the array `"hobbies"` by appending indices to each element: `"hobbies[0]"`, `"hobbies[1]"`, and `"hobbies[2]"`.

   Result:
   ```json
   {
     "name.first": "John",
     "name.last": "Doe",
     "age": 30,
     "city": "New York",
     "hobbies[0]": "Reading",
     "hobbies[1]": "Hiking",
     "hobbies[2]": "Gaming"
   }
   ```
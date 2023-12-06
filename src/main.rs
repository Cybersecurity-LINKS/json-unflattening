// Copyright 2023 Fondazione LINKS

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.



use json_unflattening::{flattening::flatten, unflattening::unflatten};
use serde_json::json;

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

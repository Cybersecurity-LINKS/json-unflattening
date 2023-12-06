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



use serde_json::{Map, Value, json};
use crate::errors;


/// Unflattens a flattened JSON structure into the original JSON object.
///
/// # Arguments
///
/// * `data` - The flattened JSON structure represented as a key-value map (`serde_json::Map<String, Value>`).
///
/// # Returns
///
/// A Result containing the reconstructed original JSON object (`serde_json::Value`) or an error (`errors::Error`).
///
pub fn unflatten(data: &Map<String, Value>) -> Result<Value, errors::Error> {
    let mut output = json!({});

    for (p, value) in data {
        let mut cur = &mut output;
        let mut property  = "";

        let regex = regex::Regex::new(r"\.?([^.\[\]]+)|\[(\d+)\]").unwrap();

        for c in regex.captures_iter(&p){

            let c2 = c.get(2).and_then(|m|  Some(m.as_str()));

            let value = if c2.is_some() {
                Value::Array(vec![])
            } else {
                Value::Object(Map::new())
            };

            match cur {
                Value::Array(a) => {
                    let index = property.parse::<usize>().map_err(|_| errors::Error::InvalidProperty)?;
                    if a.get(index).is_none() {
                        a.push(value);
                    }
                    cur = cur.get_mut(index).ok_or(errors::Error::FormatError)?;
                },
                Value::Object(o) => {
                    if o.get(property).is_none() {
                        o.insert(property.to_owned(), value);
                    }
                    cur = cur.get_mut(property).ok_or(errors::Error::Unspecified)?;
                    
                },
                _ => return Err(errors::Error::InvalidType)
            };

            if let Some(v2) = c2 {
                property = v2;
            } else if let Some(v1) = c.get(1).and_then(|m|  Some(m.as_str())){
                property = v1;
            } else {
                return Err(errors::Error::InvalidProperty);
            };

        }

        match cur {
            Value::Array(a) => {
                a.push(value.clone());
            },
            Value::Object(o) => {
                o.insert(property.to_owned(), value.clone());
            },
            _ => return Err(errors::Error::InvalidType)
            
        }

    }
    return output.get("").ok_or(errors::Error::InvalidProperty).cloned()
}
    



#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::flattening::flatten;
    use super::*;


    #[test]
    fn unflattening_nested_arrays_and_objects_1() {
        let json: Value = json!({
            "x": [
                ["y", "z"],
                { "p": "q" },
                ["r", "s"],
                [
                    { "u": "v" },
                    { "w": "x" },
                ],
                ["y", "z"],
            ]
        });
        
        let flat = flatten(&json).unwrap();
        let unflat = unflatten(&flat).unwrap();

        println!(
            "got:\n{}\nexpected:\n{}\n",
            serde_json::to_string_pretty(&unflat).unwrap(),
            serde_json::to_string_pretty(&json).unwrap()
        );

        assert_eq!(json, unflat);
    }


    #[test]
    fn unflattening_nested_arrays_and_objects_2() {
        let json: Value = json!({
            "x": [
                "y",
                ["z", "w"],
                { "v": ["u", "t"] },
                [
                    { "s": "r" },
                    { "v": ["q", { "y": "x" }] },
                ],
                ["a"],
                "b",
            ]
        });
       
        let flat = flatten(&json).unwrap();
        let unflat = unflatten(&flat).unwrap();

        println!(
            "got:\n{}\nexpected:\n{}\n",
            serde_json::to_string_pretty(&unflat).unwrap(),
            serde_json::to_string_pretty(&json).unwrap()
        );

        assert_eq!(unflat, json);

    }


    #[test]
    fn unflattening_nested_arrays_and_objects_3() {
        let json: Value = json!({
            "a": {
              "b": "c",
              "d": [
                {
                  "e": "f",
                  "g": ["h", "i"]
                },
                {
                  "j": "k",
                  "l": ["m", "n"]
                }
              ],
              "o": {
                "p": "q",
                "r": ["s", "t"]
              }
            },
            "u": [
              {
                "v": "w",
                "x": ["y", "z"]
              },
              {
                "aa": "ab",
                "ac": [
                  {
                    "ad": "ae",
                    "af": ["ag", "ah"]
                  },
                  "ai"
                ]
              }
            ],
            "aj": "ak"
          });
       
        let flat = flatten(&json).unwrap();
        let unflat = unflatten(&flat).unwrap();

        println!(
            "got:\n{}\nexpected:\n{}\n",
            serde_json::to_string_pretty(&unflat).unwrap(),
            serde_json::to_string_pretty(&json).unwrap()
        );

        assert_eq!(unflat, json);

    }

}
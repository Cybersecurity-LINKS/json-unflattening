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



use serde_json::{Value, Map, json};
use crate::errors;


/// Flattens a JSON Value into a key-value map.
///
/// # Arguments
///
/// * `value` - The JSON Value to be flattened (`serde_json::Value`).
///
/// # Returns
///
/// A Result containing a flattened JSON structure (`serde_json::Map<String, Value>`) or an error (`errors::Error`).
///
pub fn flatten(value: &Value) -> Result<Map<String, Value>, errors::Error> {
    let mut flattened_json = Map::<String, Value>::new();

    match value {
        Value::Object(map) => {
            if map.is_empty() {
                return Ok(flattened_json);
            }
            flatten_object(&mut flattened_json, None, map)?;
        }
        _ => return Err(errors::Error::NotAnObject),
    }
    
    
    Ok(flattened_json)
}

fn flatten_object(result: &mut Map<String, Value>, property: Option<&str>, nested_json: &Map<String, Value>) -> Result<(), errors::Error>{
    for (prop, value) in nested_json {
        let flattened_prop = property.map_or_else(|| prop.clone(), |parent_key| format!("{}.{}", parent_key, prop));

        match value {
            Value::Array(array) => flatten_array(result, &flattened_prop, array),
            Value::Object(sub_json) => flatten_object(result, Some(&flattened_prop), sub_json),
            _ => flatten_value(result, &flattened_prop, value.clone()),
        }?
    }

    Ok(())
}

fn flatten_array(result: &mut Map<String, Value>, property: &str, array: &Vec<Value>) -> Result<(), errors::Error> {
    for (i, value) in array.iter().enumerate() {
        let flattened_prop = format!("{}[{}]", property, i);

        match value {
            Value::Object(sub_json) => flatten_object(result, Some(&flattened_prop), sub_json),
            Value::Array(sub_array) => flatten_array(result, &flattened_prop, sub_array),
            _ => flatten_value(result, &flattened_prop, value.clone()),
        }?
    }

    Ok(())
}

fn flatten_value(result: &mut Map<String, Value>, property: &str, val: Value) -> Result<(), errors::Error> {

    if val.is_object() || val.is_array() {
        return Err(errors::Error::NotAValue);
    }

    if let Some(v) = result.get_mut(property) {
        if let Some(existing_array) = v.as_array_mut() {
            existing_array.push(val);
        } else {
            let v = v.take();
            result[property] = json!([v, val]);
        }
    } else {
        result.insert(property.to_string(), json!(val));
    }

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn flattening_nested_arrays_and_objects_1() {
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
        let expected = json!({
            "x[0][0]": "y",
            "x[0][1]": "z",
            "x[1].p": "q",
            "x[2][0]": "r",
            "x[2][1]": "s",
            "x[3][0].u": "v",
            "x[3][1].w": "x",
            "x[4][0]": "y",
            "x[4][1]": "z"
          });


        println!(
            "got:\n{}\nexpected:\n{}\n",
            serde_json::to_string_pretty(&flat).unwrap(),
            serde_json::to_string_pretty(&expected).unwrap()
        );

        assert_eq!(
            serde_json::to_value(&flat).unwrap(),
            expected
        );
    }


    #[test]
    fn flattening_nested_arrays_and_objects_2() {
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
        let expected = json!({
            "x[0]": "y",
            "x[1][0]": "z",
            "x[1][1]": "w",
            "x[2].v[0]": "u",
            "x[2].v[1]": "t",
            "x[3][0].s": "r",
            "x[3][1].v[0]": "q",
            "x[3][1].v[1].y": "x",
            "x[4][0]": "a",
            "x[5]": "b"
          });

        println!(
            "got:\n{}\nexpected:\n{}\n",
            serde_json::to_string_pretty(&flat).unwrap(),
            serde_json::to_string_pretty(&expected).unwrap()
        );


        assert_eq!(
            serde_json::to_value(&flat).unwrap(),
            expected
        );
    }


    #[test]
    fn flattening_nested_arrays_and_objects_3() {
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
        let expected = json!({
            "a.b": "c",
            "a.d[0].e": "f",
            "a.d[0].g[0]": "h",
            "a.d[0].g[1]": "i",
            "a.d[1].j": "k",
            "a.d[1].l[0]": "m",
            "a.d[1].l[1]": "n",
            "a.o.p": "q",
            "a.o.r[0]": "s",
            "a.o.r[1]": "t",
            "u[0].v": "w",
            "u[0].x[0]": "y",
            "u[0].x[1]": "z",
            "u[1].aa": "ab",
            "u[1].ac[0].ad": "ae",
            "u[1].ac[0].af[0]": "ag",
            "u[1].ac[0].af[1]": "ah",
            "u[1].ac[1]": "ai",
            "aj": "ak"
          });

        println!(
            "got:\n{}\nexpected:\n{}\n",
            serde_json::to_string_pretty(&flat).unwrap(),
            serde_json::to_string_pretty(&expected).unwrap()
        );


        assert_eq!(
            serde_json::to_value(&flat).unwrap(),
            expected
        );
    }
}
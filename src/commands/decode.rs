use serde_bencode::value::Value;
use serde_json::Map;

pub fn bencode_to_json(value: serde_bencode::value::Value) -> serde_json::Value {
    match value {
        Value::Int(i) => i.into(),
        Value::Bytes(b) => {
            let s = String::from_utf8(b).unwrap();
            serde_json::Value::String(s)
        }
        Value::List(l) => {
            let mut lists: Vec<serde_json::Value> = Vec::new();
            for v in l.into_iter() {
                let ans = bencode_to_json(v);
                lists.push(ans);
            }

            serde_json::Value::Array(lists)
        }
        Value::Dict(d) => {
            let mut lists: serde_json::Map<String, serde_json::Value> = Map::new();
            for (k, v) in d.into_iter() {
                let ans = bencode_to_json(v);
                lists.insert(String::from_utf8(k).unwrap(), ans);
            }

            serde_json::Value::Object(lists)
        }
    }
}

pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let value = serde_bencode::from_str(encoded_value).unwrap();
    bencode_to_json(value)
}

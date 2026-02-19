use serde_bencode::value::Value;

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
        _ => unreachable!(),
    }
}

pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let value = serde_bencode::from_str(encoded_value).unwrap();
    bencode_to_json(value)
}

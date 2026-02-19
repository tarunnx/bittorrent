pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    match encoded_value.chars().next().unwrap() {
        'i' => {
            let value = encoded_value
                .strip_prefix('i')
                .unwrap()
                .strip_suffix('e')
                .unwrap()
                .parse::<i64>()
                .unwrap();
            return value.into();
        }
        '0'..='9' => {
            let word = encoded_value.split(':').nth(1).unwrap();
            return serde_json::Value::String(word.to_string());
        }
        _ => {
            panic!("Unhandled encoded value: {}", encoded_value)
        }
    }
}

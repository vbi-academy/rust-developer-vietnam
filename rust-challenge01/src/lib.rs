use std::{
    iter::{Iterator, Peekable},
    str::Chars,
};

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Value {
    True,
    False,
    String(String),
    Number(i64),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}

pub fn parse(src: &str) -> Result<Value, &'static str> {
    let removed_whitespace: String = src.chars().filter(|&c| !c.is_whitespace()).collect();

    let mut src = removed_whitespace.chars().peekable();
    object(&mut src)
}
pub fn expect(src: &mut Peekable<Chars>, expected: char) -> Result<(), &'static str> {
    if src.next_if_eq(&expected).is_none() {
        eprintln!("expected '{}' found '{:?}'", expected, src.peek());
        Err("Cannot match correct character")
    } else {
        Ok(())
    }
}

pub fn string(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    expect(src, '"')?;
    let mut res = String::new();
    while let Some(c) = src.next_if(|c| *c != '"') {
        res.push(c);
    }
    expect(src, '"')?;

    Ok(Value::String(res))
}

pub fn number(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    let mut res = String::new();
    while let Some(c) = src.next_if(|c| c.is_numeric()) {
        res.push(c);
    }

    Ok(Value::Number(res.parse().expect("Can not parse to number")))
}

pub fn object(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    expect(src, '{')?;

    if src.next_if_eq(&'}').is_some() {
        return Ok(Value::Object(HashMap::new()));
    }
    let mut object = HashMap::new();
    loop {
        let key = string(src)?;
        expect(src, ':')?;
        let value = value(src)?;
        if let Value::String(s) = key {
            object.insert(s, value);
        }

        if let Some(_) = src.next_if_eq(&',') {
            continue;
        } else {
            break;
        }
    }
    Ok(Value::Object(object))
}
pub fn value(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    match src.peek() {
        Some('{') => object(src),
        Some('"') => string(src),
        Some('[') => array(src),
        Some(c) if c.is_numeric() => number(src),
        _ => bool(src),
    }
}

pub fn bool(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    let mut res = String::new();

    while let Some(c) = src.next_if(|c| *c != ',') {
        res.push(c);
    }
    match res.as_str() {
        "true" => Ok(Value::True),
        "false" => Ok(Value::False),
        _ => Err("Cant not parse to bool"),
    }
}

pub fn array(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    expect(src, '[')?;
    if src.next_if_eq(&']').is_some() {
        return Ok(Value::Array(Vec::new()));
    }

    let mut res = Vec::new();

    loop {
        let value = value(src)?;
        res.push(value);

        if let Some(_) = src.next_if_eq(&',') {
            continue;
        } else {
            break;
        }
    }

    expect(src, ']')?;

    Ok(Value::Array(res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_empty_json() {
        let src =
            std::fs::read_to_string("tests/step1/valid.json").expect(&format!("Missing test file"));

        assert_eq!(parse(&src).unwrap(), Value::Object(HashMap::new()));
    }

    #[test]
    fn step2_key_value_json() {
        let src = std::fs::read_to_string("tests/step2/valid1.json")
            .expect(&format!("Missing test file"));
        println!("Source:{}", src);

        let parsed = parse(&src);
        let mut body = HashMap::new();
        body.insert("key".to_string(), Value::String("value".to_string()));

        let expectation = Ok(Value::Object(body));
        assert_eq!(parsed, expectation);
    }

    #[test]
    fn step3_multiple_key_value_json() {
        let src = std::fs::read_to_string("tests/step3/valid1.json")
            .expect(&format!("Missing test file"));

        let parsed = parse(&src);
        let mut body = HashMap::new();
        body.insert("key1".to_string(), Value::True);
        body.insert("key2".to_string(), Value::False);
        body.insert("key3".to_string(), Value::String("value".to_string()));
        body.insert("key4".to_string(), Value::Number(101));

        let expectation = Ok(Value::Object(body));
        assert_eq!(parsed, expectation);
    }

    #[test]
    fn step4_multiple_value_json() {
        let src = std::fs::read_to_string("tests/step4/valid1.json")
            .expect(&format!("Missing test file"));

        let parsed = parse(&src);
        let mut body = HashMap::new();
        body.insert("key".to_string(), Value::String("value".to_string()));
        body.insert("key1".to_string(), Value::Number(101));
        body.insert("key2".to_string(), Value::Object(HashMap::new()));
        body.insert("key3".to_string(), Value::Array(vec![]));

        let expectation = Ok(Value::Object(body));
        assert_eq!(parsed, expectation);
    }

    // {
    //     "title": "Rust",
    //     "year": 2023,
    //     "live": true,
    //     "organizers": [
    //         "vbi",
    //         "techfest"
    //     ],
    //     "presenter": {
    //         "name": "Dung",
    //         "age": 27,
    //         "occupation": "Engineer"
    //     }
    // }
    #[test]
    fn step5_multiple_value_json() {
        let src =
            std::fs::read_to_string("tests/step5/valid.json").expect(&format!("Missing test file"));

        let parsed = parse(&src);
        let mut body = HashMap::new();
        let payload_org = vec![
            Value::String("vbi".to_string()),
            Value::String("techfest".to_string()),
        ];
        let mut payload_presenter = HashMap::new();
        payload_presenter.insert("name".to_string(), Value::String("Dung".to_string()));
        payload_presenter.insert("age".to_string(), Value::Number(27));
        payload_presenter.insert(
            "occupation".to_string(),
            Value::String("Engineer".to_string()),
        );

        body.insert("title".to_string(), Value::String("Rust".to_string()));
        body.insert("year".to_string(), Value::Number(2023));
        body.insert("live".to_string(), Value::True);
        body.insert("presenter".to_string(), Value::Object(payload_presenter));
        body.insert("organizers".to_string(), Value::Array(payload_org));

        let expectation = Ok(Value::Object(body));
        assert_eq!(parsed, expectation);
    }
}

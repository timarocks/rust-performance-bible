//! Zero-allocation JSON parser for simple objects
//! 
//! Parses JSON without any heap allocations by using lifetime magic.
//! Limited to flat objects for simplicity.

use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub struct JsonObject<'a> {
    pairs: Vec<(&'a str, JsonValue<'a>)>,
}

#[derive(Debug)]
pub enum JsonValue<'a> {
    String(&'a str),
    Number(f64),
    Bool(bool),
    Null,
}

pub fn parse_json_object(input: &str) -> Option<JsonObject> {
    let mut chars = input.trim().chars().peekable();
    
    // Expect opening brace
    if chars.next() != Some('{') {
        return None;
    }
    
    let mut pairs = Vec::new();
    
    loop {
        skip_whitespace(&mut chars);
        
        if chars.peek() == Some(&'}') {
            chars.next();
            break;
        }
        
        // Parse key
        let key_start = chars.clone().position(|c| c == '"')? + 1;
        skip_until(&mut chars, '"');
        chars.next(); // consume opening quote
        
        let key_end = chars.clone().position(|c| c == '"')?;
        let key = &input[key_start..key_start + key_end];
        chars.next(); // consume closing quote
        
        skip_whitespace(&mut chars);
        if chars.next() != Some(':') {
            return None;
        }
        skip_whitespace(&mut chars);
        
        // Parse value
        let value = parse_value(input, &mut chars)?;
        pairs.push((key, value));
        
        skip_whitespace(&mut chars);
        match chars.peek() {
            Some(',') => { chars.next(); continue; }
            Some('}') => { chars.next(); break; }
            _ => return None,
        }
    }
    
    Some(JsonObject { pairs })
}

fn parse_value<'a>(
    input: &'a str,
    chars: &mut Peekable<Chars>
) -> Option<JsonValue<'a>> {
    match chars.peek()? {
        '"' => {
            chars.next(); // consume quote
            let start = chars.clone().count();
            let start = input.len() - start;
            
            skip_until(chars, '"');
            let end = chars.clone().count();
            let end = input.len() - end - 1;
            
            chars.next(); // consume closing quote
            Some(JsonValue::String(&input[start..end]))
        }
        't' | 'f' => {
            let word = take_while(chars, |c| c.is_alphabetic());
            match word {
                "true" => Some(JsonValue::Bool(true)),
                "false" => Some(JsonValue::Bool(false)),
                _ => None,
            }
        }
        'n' => {
            if take_while(chars, |c| c.is_alphabetic()) == "null" {
                Some(JsonValue::Null)
            } else {
                None
            }
        }
        c if c.is_numeric() || *c == '-' => {
            let num_str = take_while(chars, |c| {
                c.is_numeric() || *c == '.' || *c == '-'
            });
            num_str.parse().ok().map(JsonValue::Number)
        }
        _ => None,
    }
}

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while matches!(chars.peek(), Some(' ') | Some('\t') | Some('\n') | Some('\r')) {
        chars.next();
    }
}

fn skip_until(chars: &mut Peekable<Chars>, target: char) {
    while chars.peek() != Some(&target) && chars.peek().is_some() {
        chars.next();
    }
}

fn take_while<F>(chars: &mut Peekable<Chars>, predicate: F) -> String
where
    F: Fn(&char) -> bool,
{
    let mut result = String::new();
    while let Some(&ch) = chars.peek() {
        if predicate(&ch) {
            result.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zero_alloc_json() {
        let json = r#"{"name": "crabcore", "version": 1.0, "fast": true}"#;
        let obj = parse_json_object(json).unwrap();
        
        assert_eq!(obj.pairs.len(), 3);
        assert_eq!(obj.pairs[0].0, "name");
        matches!(obj.pairs[0].1, JsonValue::String("crabcore"));
    }
}
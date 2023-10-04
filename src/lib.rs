use thiserror::Error;
use wasm_bindgen::prelude::*;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("長さが足りません")]
    TooShort,
    #[error("`{0}` という文字は使えません")]
    BadCharacter(char),
    #[error("{0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

impl From<CodecError> for JsValue {
    fn from(value: CodecError) -> Self {
        JsValue::from(value.to_string())
    }
}

trait TryIterator: Iterator {
    type Ok;
    type Err;

    fn try_next(&mut self) -> Result<Option<Self::Ok>, Self::Err>;
}

impl<I, T, E> TryIterator for I
where
    I: Iterator<Item = Result<T, E>>,
{
    type Ok = T;
    type Err = E;

    fn try_next(&mut self) -> Result<Option<Self::Ok>, Self::Err> {
        match self.next() {
            None => Ok(None),
            Some(Ok(t)) => Ok(Some(t)),
            Some(Err(e)) => Err(e),
        }
    }
}

fn encode_byte(input: u8) -> char {
    match input & 7 {
        0 => '楽',
        1 => '可',
        2 => '愛',
        3 => 'っ',
        4 => 'た',
        5 => 'し',
        6 => 'か',
        7 => '。',
        _ => unreachable!(),
    }
}

fn decode_char(input: char) -> Result<u8, CodecError> {
    match input {
        '楽' => Ok(0),
        '可' => Ok(1),
        '愛' => Ok(2),
        'っ' => Ok(3),
        'た' => Ok(4),
        'し' => Ok(5),
        'か' => Ok(6),
        '。' => Ok(7),
        _ => Err(CodecError::BadCharacter(input)),
    }
}

#[wasm_bindgen]
pub fn encode(input: &str) -> String {
    let input = input.as_bytes();
    let mut output = String::with_capacity(input.len() * 8 / 3 + 1);
    let mut input = input.iter();

    while let Some(b1) = input.next() {
        output.push(encode_byte(b1 >> 5));
        output.push(encode_byte(b1 >> 2));

        let (b2, end) = if let Some(b2) = input.next() {
            (*b2, false)
        } else {
            (0, true)
        };
        output.push(encode_byte(b1 << 1 | b2 >> 7));
        if end {
            break;
        }

        output.push(encode_byte(b2 >> 4));
        output.push(encode_byte(b2 >> 1));

        let (b3, end) = if let Some(b3) = input.next() {
            (*b3, false)
        } else {
            (0, true)
        };
        output.push(encode_byte(b2 << 2 | b3 >> 6));
        if end {
            break;
        }

        output.push(encode_byte(b3 >> 3));
        output.push(encode_byte(b3));
    }

    output
}

#[wasm_bindgen]
pub fn decode(input: &str) -> Result<String, CodecError> {
    let mut output = Vec::with_capacity(input.len() * 3 / 8 + 1);
    let mut input = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(decode_char);

    while let Some(mut b) = input.try_next()? {
        let i = input.try_next()?.ok_or(CodecError::TooShort)?;
        b = b << 3 | i;

        let i = input.try_next()?.ok_or(CodecError::TooShort)?;
        output.push(b << 2 | i >> 1);
        b = i;

        let Some(i) = input.try_next()? else { break };
        b = b << 3 | i;

        let i = input.try_next()?.ok_or(CodecError::TooShort)?;
        b = b << 3 | i;

        let i = input.try_next()?.ok_or(CodecError::TooShort)?;
        output.push(b << 1 | i >> 2);
        b = i;

        let Some(i) = input.try_next()? else { break };
        b = b << 3 | i;

        let i = input.try_next()?.ok_or(CodecError::TooShort)?;
        output.push(b << 3 | i);
    }

    Ok(String::from_utf8(output)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq_reflexive(raw: &str, artr: &str) {
        assert_eq!(encode(raw), artr);
        assert_eq!(decode(artr).unwrap(), raw);
    }

    #[test]
    fn test_encode_decode() {
        assert_eq_reflexive(
            "ありとろ",
            "。楽。楽楽か楽愛。楽。楽可愛可愛。楽。楽楽かし楽。楽。楽可愛可し",
        );
        assert_eq_reflexive("", "");
        assert_eq_reflexive("A", "愛楽愛");
        assert_eq_reflexive("AA", "愛楽愛た楽た");
        assert_eq_reflexive("AAA", "愛楽愛た楽し楽可");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("愛 楽 愛").unwrap(), "A");
    }

    #[test]
    fn test_decode_error() {
        assert!(matches!(decode("。"), Err(CodecError::TooShort)));
        assert!(matches!(decode("あ"), Err(CodecError::BadCharacter('あ'))));
        assert!(matches!(decode("。。。"), Err(_)));
    }
}

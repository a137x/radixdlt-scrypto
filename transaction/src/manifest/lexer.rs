use crate::manifest::compiler::CompileErrorDiagnosticsStyle;
use crate::manifest::diagnostic_snippets::create_snippet;
use crate::manifest::token::{Position, Span, Token, TokenKind};
use sbor::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectedChar {
    Exact(char),
    OneOf(Vec<char>),
    HexDigit,
    DigitLetterQuotePunctuation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerErrorKind {
    UnexpectedEof,
    UnexpectedChar(char, ExpectedChar),
    InvalidIntegerLiteral(String),
    InvalidIntegerType(String),
    InvalidInteger(String),
    InvalidUnicode(u32),
    MissingUnicodeSurrogate(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub error_kind: LexerErrorKind,
    pub span: Span,
}

impl LexerError {
    fn unexpected_char(position: Position, c: char, expected: ExpectedChar) -> Self {
        Self {
            error_kind: LexerErrorKind::UnexpectedChar(c, expected),
            span: Span {
                start: position,
                end: position.advance(c),
            },
        }
    }

    fn invalid_integer_type(ty: String, start: Position, end: Position) -> Self {
        Self {
            error_kind: LexerErrorKind::InvalidIntegerType(ty),
            span: Span { start, end },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    /// The input text chars
    text: Vec<char>,
    /// The current position in the text (in case of end of file it equals to text length)
    current: Position,
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Lexer::new(s);
    let mut tokens = Vec::new();
    loop {
        if let Some(token) = lexer.next_token()? {
            tokens.push(token);
        } else {
            break;
        }
    }
    Ok(tokens)
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.chars().collect(),
            current: Position {
                full_index: 0,
                line_number: 1,
                line_char_index: 0,
            },
        }
    }

    pub fn is_eof(&self) -> bool {
        self.current.full_index == self.text.len()
    }

    fn peek(&self) -> Result<char, LexerError> {
        if self.is_eof() {
            Err(LexerError {
                error_kind: LexerErrorKind::UnexpectedEof,
                span: Span {
                    start: self.current,
                    end: self.current,
                },
            })
        } else {
            Ok(self.text[self.current.full_index])
        }
    }

    fn advance(&mut self) -> Result<char, LexerError> {
        let c = self.peek()?;
        self.current = self.current.advance(c);
        Ok(c)
    }

    fn advance_expected(&mut self, expected: char) -> Result<char, LexerError> {
        self.advance_matching(|c| c == expected, ExpectedChar::Exact(expected))
    }

    fn advance_matching(
        &mut self,
        matcher: impl Fn(char) -> bool,
        expected: ExpectedChar,
    ) -> Result<char, LexerError> {
        let previous = self.current;
        let c = self.advance()?;
        if !matcher(c) {
            Err(LexerError::unexpected_char(previous, c, expected))
        } else {
            Ok(c)
        }
    }

    fn advance_and_append(&mut self, s: &mut String) -> Result<char, LexerError> {
        let c = self.advance()?;
        s.push(c);
        Ok(c)
    }

    fn is_whitespace(c: char) -> bool {
        // slightly different from the original specs, we skip `\n`
        // rather than consider it as a terminal
        c == ' ' || c == '\t' || c == '\r' || c == '\n'
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        // skip comment and whitespace
        let mut in_comment = false;
        while !self.is_eof() {
            if in_comment {
                if self.advance()? == '\n' {
                    in_comment = false;
                }
            } else if self.peek()? == '#' {
                in_comment = true;
            } else if Self::is_whitespace(self.peek()?) {
                self.advance()?;
            } else {
                break;
            }
        }

        // check if it's the end of file
        if self.is_eof() {
            return Ok(None);
        }

        // match next token
        match self.peek()? {
            '-' | '0'..='9' => self.tokenize_number(),
            '"' => self.tokenize_string(),
            'a'..='z' | 'A'..='Z' => self.tokenize_identifier(),
            '{' | '}' | '(' | ')' | '<' | '>' | ',' | ';' | '&' | '=' => {
                self.tokenize_punctuation()
            }
            c => Err(LexerError::unexpected_char(
                self.current,
                c,
                ExpectedChar::DigitLetterQuotePunctuation,
            )),
        }
        .map(Option::from)
    }

    // TODO: consider using DFA
    fn tokenize_number(&mut self) -> Result<Token, LexerError> {
        let literal_start = self.current;
        let mut s = String::new();

        // negative sign
        if self.peek()? == '-' {
            s.push(self.advance()?);
        }

        // integer
        match self.advance_and_append(&mut s)? {
            '0' => {}
            '1'..='9' => {
                while self.peek()?.is_ascii_digit() {
                    s.push(self.advance()?);
                }
            }
            _ => {
                return Err(LexerError {
                    error_kind: LexerErrorKind::InvalidIntegerLiteral(s),
                    span: Span {
                        start: literal_start,
                        end: self.current,
                    },
                });
            }
        }

        // type
        let ty_start = self.current;
        let mut t = String::new();
        match self.advance_and_append(&mut t)? {
            'i' => match self.advance_and_append(&mut t)? {
                '1' => match self.advance_and_append(&mut t)? {
                    '2' => match self.advance_and_append(&mut t)? {
                        '8' => self.parse_int(&s, "i128", TokenKind::I128Literal, literal_start),
                        _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
                    },
                    '6' => self.parse_int(&s, "i16", TokenKind::I16Literal, literal_start),
                    _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
                },
                '3' => match self.advance_and_append(&mut t)? {
                    '2' => self.parse_int(&s, "i32", TokenKind::I32Literal, literal_start),
                    _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
                },
                '6' => match self.advance_and_append(&mut t)? {
                    '4' => self.parse_int(&s, "i64", TokenKind::I64Literal, literal_start),
                    _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
                },
                '8' => self.parse_int(&s, "i8", TokenKind::I8Literal, literal_start),
                _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
            },
            'u' => match self.advance_and_append(&mut t)? {
                '1' => match self.advance_and_append(&mut t)? {
                    '2' => match self.advance_and_append(&mut t)? {
                        '8' => self.parse_int(&s, "u128", TokenKind::U128Literal, literal_start),
                        _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
                    },
                    '6' => self.parse_int(&s, "u16", TokenKind::U16Literal, literal_start),
                    _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
                },
                '3' => match self.advance_and_append(&mut t)? {
                    '2' => self.parse_int(&s, "u32", TokenKind::U32Literal, literal_start),
                    _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
                },
                '6' => match self.advance_and_append(&mut t)? {
                    '4' => self.parse_int(&s, "u64", TokenKind::U64Literal, literal_start),
                    _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
                },
                '8' => self.parse_int(&s, "u8", TokenKind::U8Literal, literal_start),
                _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
            },
            _ => Err(LexerError::invalid_integer_type(t, ty_start, self.current)),
        }
        .map(|kind| self.new_token(kind, literal_start, self.current))
    }

    fn parse_int<T>(
        &self,
        int: &str,
        ty: &str,
        map: fn(T) -> TokenKind,
        token_start: Position,
    ) -> Result<TokenKind, LexerError>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        int.parse::<T>().map(map).map_err(|err| LexerError {
            error_kind: LexerErrorKind::InvalidInteger(format!(
                "'{}{}' - {}",
                int,
                ty,
                err.to_string()
            )),
            span: Span {
                start: token_start,
                end: self.current,
            },
        })
    }

    fn tokenize_string(&mut self) -> Result<Token, LexerError> {
        let start = self.current;
        assert_eq!(self.advance()?, '"');

        let mut s = String::new();
        while self.peek()? != '"' {
            let c = self.advance()?;
            if c == '\\' {
                // Remember '\\' position
                let token_start = self.current;

                // See the JSON string specifications
                match self.advance()? {
                    '"' => s.push('\"'),
                    '\\' => s.push('\\'),
                    '/' => s.push('/'),
                    'b' => s.push('\x08'),
                    'f' => s.push('\x0c'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    'u' => {
                        let mut unicode = self.read_utf16_unit()?;
                        // Check unicode surrogate pair
                        // (see https://unicodebook.readthedocs.io/unicode_encodings.html#surrogates)
                        if (0xD800..=0xDFFF).contains(&unicode) {
                            let position = self.current;
                            if self.advance()? == '\\' && self.advance()? == 'u' {
                                unicode = 0x10000
                                    + ((unicode - 0xD800) << 10)
                                    + self.read_utf16_unit()?
                                    - 0xDC00;
                            } else {
                                return Err(LexerError {
                                    error_kind: LexerErrorKind::MissingUnicodeSurrogate(unicode),
                                    span: Span {
                                        start: token_start,
                                        end: position,
                                    },
                                });
                            }
                        }
                        s.push(char::from_u32(unicode).ok_or(LexerError {
                            error_kind: LexerErrorKind::InvalidUnicode(unicode),
                            span: Span {
                                start: token_start,
                                end: self.current,
                            },
                        })?);
                    }
                    c => {
                        return Err(LexerError::unexpected_char(
                            token_start,
                            c,
                            ExpectedChar::OneOf(vec!['"', '\\', '/', 'b', 'f', 'n', 'r', 't', 'u']),
                        ));
                    }
                }
            } else {
                s.push(c);
            }
        }
        self.advance()?;

        Ok(self.new_token(TokenKind::StringLiteral(s), start, self.current))
    }

    fn read_utf16_unit(&mut self) -> Result<u32, LexerError> {
        let mut code: u32 = 0;

        for _ in 0..4 {
            let c = self.advance_matching(|c| c.is_ascii_hexdigit(), ExpectedChar::HexDigit)?;
            code = code * 16 + c.to_digit(16).unwrap();
        }

        Ok(code)
    }

    fn tokenize_identifier(&mut self) -> Result<Token, LexerError> {
        let start = self.current;

        let mut id = String::from(self.advance()?);
        while !self.is_eof() {
            let next_char = self.peek()?;
            let next_char_can_be_part_of_ident =
                next_char.is_ascii_alphanumeric() || next_char == '_' || next_char == ':';
            if !next_char_can_be_part_of_ident {
                break;
            }
            id.push(self.advance()?);
        }

        let kind = match id.as_str() {
            "true" => TokenKind::BoolLiteral(true),
            "false" => TokenKind::BoolLiteral(false),
            other => TokenKind::Ident(other.to_string()),
        };
        Ok(self.new_token(kind, start, self.current))
    }

    fn tokenize_punctuation(&mut self) -> Result<Token, LexerError> {
        let start = self.current;

        let expected = vec!['(', ')', '<', '>', ',', ';', '='];
        let token_kind = match self.advance_matching(
            |c| expected.contains(&c),
            ExpectedChar::OneOf(expected.clone()),
        )? {
            '(' => TokenKind::OpenParenthesis,
            ')' => TokenKind::CloseParenthesis,
            '<' => TokenKind::LessThan,
            '>' => TokenKind::GreaterThan,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '=' => {
                self.advance_expected('>')?;
                TokenKind::FatArrow
            }
            _ => {
                unreachable!();
            }
        };

        Ok(self.new_token(token_kind, start, self.current))
    }

    fn new_token(&self, kind: TokenKind, start: Position, end: Position) -> Token {
        Token {
            kind,
            span: Span { start, end },
        }
    }
}

pub fn lexer_error_diagnostics(
    s: &str,
    err: LexerError,
    style: CompileErrorDiagnosticsStyle,
) -> String {
    let (title, label) = match err.error_kind {
        LexerErrorKind::UnexpectedEof => (
            "unexpected end of file".to_string(),
            "unexpected end of file".to_string(),
        ),
        LexerErrorKind::UnexpectedChar(c, expected) => {
            let expected = match expected {
                ExpectedChar::Exact(exact) => format!("'{}'", exact),
                ExpectedChar::OneOf(one_of) => {
                    let v: Vec<String> = one_of.iter().map(|c| format!("'{}'", c)).collect();
                    if let Some((last, init)) =  v.split_last() {
                        format!("{} or {}", init.join(", "), last)
                    }
                    else {
                        "unknown".to_string()
                    }
                }
                ExpectedChar::HexDigit => "hex digit".to_string(),
                ExpectedChar::DigitLetterQuotePunctuation => "digit, letter, quotation mark or one of punctuation characters '(', ')', '<', '>', ',', ';', '='".to_string(),
            };
            (
                format!("unexpected character {:?}, expected {}", c, expected),
                "unexpected character".to_string(),
            )
        }
        LexerErrorKind::InvalidIntegerLiteral(string) => (
            format!("invalid integer literal '{}'", string),
            "invalid integer literal".to_string(),
        ),
        LexerErrorKind::InvalidIntegerType(string) => (
            format!("invalid integer type '{}'", string),
            "invalid integer type".to_string(),
        ),
        LexerErrorKind::InvalidInteger(string) => (
            format!("invalid integer value {}", string),
            "invalid integer value".to_string(),
        ),
        LexerErrorKind::InvalidUnicode(value) => (
            format!("invalid unicode point {}", value),
            "invalid unicode".to_string(),
        ),
        LexerErrorKind::MissingUnicodeSurrogate(value) => (
            format!("missing unicode '{:X}' surrogate pair", value),
            "missing unicode surrogate pair".to_string(),
        ),
    };
    create_snippet(s, &err.span, &title, &label, style)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{position, span};

    #[macro_export]
    macro_rules! lex_ok {
        ( $s:expr, $expected:expr ) => {{
            let mut lexer = Lexer::new($s);
            for i in 0..$expected.len() {
                assert_eq!(
                    lexer.next_token().map(|opt| opt.map(|t| t.kind)),
                    Ok(Some($expected[i].clone()))
                );
            }
            assert_eq!(lexer.next_token(), Ok(None));
        }};
    }

    #[macro_export]
    macro_rules! lex_error {
        ( $s:expr, $expected:expr ) => {{
            let mut lexer = Lexer::new($s);
            loop {
                match lexer.next_token() {
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        panic!("Expected {:?} but no error is thrown", $expected);
                    }
                    Err(e) => {
                        assert_eq!(e, $expected);
                        break;
                    }
                }
            }
        }};
    }

    #[test]
    fn test_empty_strings() {
        lex_ok!("", Vec::<TokenKind>::new());
        lex_ok!("  ", Vec::<TokenKind>::new());
        lex_ok!("\r\n\t", Vec::<TokenKind>::new());
    }

    #[test]
    fn test_bool() {
        lex_ok!("true", vec![TokenKind::BoolLiteral(true)]);
        lex_ok!("false", vec![TokenKind::BoolLiteral(false)]);
        lex_ok!("false123u8", vec![TokenKind::Ident("false123u8".into())]);
    }

    #[test]
    fn test_int() {
        lex_ok!(
            "1u82u1283i84i128",
            vec![
                TokenKind::U8Literal(1),
                TokenKind::U128Literal(2),
                TokenKind::I8Literal(3),
                TokenKind::I128Literal(4),
            ]
        );
        lex_ok!(
            "1u8 2u32",
            vec![TokenKind::U8Literal(1), TokenKind::U32Literal(2)]
        );
        lex_error!(
            "123",
            LexerError {
                error_kind: LexerErrorKind::UnexpectedEof,
                span: span!(start = (3, 1, 3), end = (3, 1, 3))
            }
        );
    }

    #[test]
    fn test_comment() {
        lex_ok!("# 1u8", Vec::<TokenKind>::new());
        lex_ok!("1u8 # comment", vec![TokenKind::U8Literal(1),]);
        lex_ok!(
            "# multiple\n# line\nCALL_FUNCTION",
            vec![TokenKind::Ident("CALL_FUNCTION".to_string()),]
        );
    }

    #[test]
    fn test_string() {
        lex_ok!(
            r#"  "" "abc" "abc\r\n\"def\uD83C\uDF0D"  "#,
            vec![
                TokenKind::StringLiteral("".into()),
                TokenKind::StringLiteral("abc".into()),
                TokenKind::StringLiteral("abc\r\n\"def🌍".into()),
            ]
        );
        lex_error!(
            "\"",
            LexerError {
                error_kind: LexerErrorKind::UnexpectedEof,
                span: span!(start = (1, 1, 1), end = (1, 1, 1))
            }
        );
    }

    #[test]
    fn test_mixed() {
        lex_ok!(
            r#"CALL_FUNCTION Map<String, Array>("test", Array<String>("abc"));"#,
            vec![
                TokenKind::Ident("CALL_FUNCTION".to_string()),
                TokenKind::Ident("Map".to_string()),
                TokenKind::LessThan,
                TokenKind::Ident("String".to_string()),
                TokenKind::Comma,
                TokenKind::Ident("Array".to_string()),
                TokenKind::GreaterThan,
                TokenKind::OpenParenthesis,
                TokenKind::StringLiteral("test".into()),
                TokenKind::Comma,
                TokenKind::Ident("Array".to_string()),
                TokenKind::LessThan,
                TokenKind::Ident("String".to_string()),
                TokenKind::GreaterThan,
                TokenKind::OpenParenthesis,
                TokenKind::StringLiteral("abc".into()),
                TokenKind::CloseParenthesis,
                TokenKind::CloseParenthesis,
                TokenKind::Semicolon,
            ]
        );
    }

    #[test]
    fn test_precise_decimal() {
        lex_ok!(
            "PreciseDecimal(\"12\")",
            vec![
                TokenKind::Ident("PreciseDecimal".to_string()),
                TokenKind::OpenParenthesis,
                TokenKind::StringLiteral("12".into()),
                TokenKind::CloseParenthesis,
            ]
        );
    }

    #[test]
    fn test_precise_decimal_collection() {
        lex_ok!(
            "Array<PreciseDecimal>(PreciseDecimal(\"12\"), PreciseDecimal(\"212\"), PreciseDecimal(\"1984\"))",
            vec![
                TokenKind::Ident("Array".to_string()),
                TokenKind::LessThan,
                TokenKind::Ident("PreciseDecimal".to_string()),
                TokenKind::GreaterThan,
                TokenKind::OpenParenthesis,
                TokenKind::Ident("PreciseDecimal".to_string()),
                TokenKind::OpenParenthesis,
                TokenKind::StringLiteral("12".into()),
                TokenKind::CloseParenthesis,
                TokenKind::Comma,
                TokenKind::Ident("PreciseDecimal".to_string()),
                TokenKind::OpenParenthesis,
                TokenKind::StringLiteral("212".into()),
                TokenKind::CloseParenthesis,
                TokenKind::Comma,
                TokenKind::Ident("PreciseDecimal".to_string()),
                TokenKind::OpenParenthesis,
                TokenKind::StringLiteral("1984".into()),
                TokenKind::CloseParenthesis,
                TokenKind::CloseParenthesis,
            ]
        );
    }

    #[test]
    fn test_invalid_integer() {
        lex_error!(
            "-_28u32",
            LexerError {
                error_kind: LexerErrorKind::InvalidIntegerLiteral("-_".to_string()),
                span: span!(start = (0, 1, 0), end = (2, 1, 2))
            }
        );

        lex_error!(
            "1i128\n 1u64 \n 1i37",
            LexerError {
                error_kind: LexerErrorKind::InvalidIntegerType("i37".to_string()),
                span: span!(start = (15, 3, 2), end = (18, 3, 5))
            }
        );

        lex_error!(
            "3_0i8",
            LexerError {
                error_kind: LexerErrorKind::InvalidIntegerType("_".to_string()),
                span: span!(start = (1, 1, 1), end = (2, 1, 2))
            }
        );
    }

    #[test]
    fn test_unexpected_char() {
        lex_error!(
            "1u8 +2u32",
            LexerError {
                error_kind: LexerErrorKind::UnexpectedChar(
                    '+',
                    ExpectedChar::DigitLetterQuotePunctuation
                ),
                span: span!(start = (4, 1, 4), end = (5, 1, 5))
            }
        );

        lex_error!(
            "x=7",
            LexerError {
                error_kind: LexerErrorKind::UnexpectedChar('7', ExpectedChar::Exact('>')),
                span: span!(start = (2, 1, 2), end = (3, 1, 3))
            }
        );
    }

    #[test]
    fn test_unicode() {
        lex_ok!(
            r#""\u2764""#,
            vec![TokenKind::StringLiteral("❤".to_string())]
        );
        lex_ok!(
            r#""\uFA84""#,
            vec![TokenKind::StringLiteral("彩".to_string())]
        );
        lex_ok!(
            r#""\uD83D\uDC69""#,
            vec![TokenKind::StringLiteral("👩".to_string())]
        );
        lex_error!(
            r#""\uDCAC\u1234""#,
            LexerError {
                error_kind: LexerErrorKind::InvalidUnicode(1238580),
                span: span!(start = (2, 1, 2), end = (13, 1, 13))
            }
        );
    }
}

// Copyright 2022 the Gigamono authors. All rights reserved. GPL-3.0 License.

use crate::span::Span;

/// Represents a valid Raccoon token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// The valid kinds of token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Newline,
    Indent,
    Dedent,
    Delimiter,
    Identifier { value: String },
    DecFloat { value: String },
    DecFloatImag { value: String },
    PrefixedString { value: String },
    ByteString { value: String },
    String { value: String },
    DecInteger { value: String },
    DecIntegerImag { value: String },
    BinInteger { value: String },
    OctInteger { value: String },
    HexInteger { value: String },
    Operator { value: String },
    Keyword { value: String },
    Unknown,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

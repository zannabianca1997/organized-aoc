use std::{fmt::Display, mem, num::ParseIntError, str::FromStr};

use darling::{export::syn::Lit, FromMeta};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8", expecting = "a day in the advent (1-25)")]
#[repr(u8)]
pub enum AoCDay {
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
    _9 = 9,
    _10 = 10,
    _11 = 11,
    _12 = 12,
    _13 = 13,
    _14 = 14,
    _15 = 15,
    _16 = 16,
    _17 = 17,
    _18 = 18,
    _19 = 19,
    _20 = 20,
    _21 = 21,
    _22 = 22,
    _23 = 23,
    _24 = 24,
    _25 = 25,
}

impl From<AoCDay> for u8 {
    fn from(value: AoCDay) -> Self {
        value as u8
    }
}

#[derive(Debug, Clone, Copy, Error)]
#[error("{0} is not a day of advent")]
pub struct NotAoCDayError(pub u8);

impl TryFrom<u8> for AoCDay {
    type Error = NotAoCDayError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 || value > 25 {
            return Err(NotAoCDayError(value));
        }
        Ok(unsafe {
            /* SAFETY: #[repr(u8)] guarantee this */
            mem::transmute(value)
        })
    }
}

impl Display for AoCDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        u8::from(*self).fmt(f)
    }
}

#[derive(Debug, Clone, Error)]
pub enum ParseAoCDayError {
    #[error(transparent)]
    PareIntError(#[from] ParseIntError),
    #[error(transparent)]
    NotAdventDayError(#[from] NotAoCDayError),
}

impl FromStr for AoCDay {
    type Err = ParseAoCDayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u8>()?.try_into()?)
    }
}

impl FromMeta for AoCDay {
    fn from_value(value: &Lit) -> darling::Result<Self> {
        (if let Lit::Int(ref s) = *value {
            s.base10_parse::<u8>()
                .map_err(darling::Error::custom)
                .and_then(|v| Self::try_from(v).map_err(darling::Error::custom))
        } else {
            Err(darling::Error::unexpected_lit_type(value))
        })
        .map_err(|e| e.with_span(value))
    }
}

impl ToTokens for AoCDay {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("_{}", u8::from(*self));
        quote!(::aoc_runtime::calendar::AoCDay::#ident).to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(
    try_from = "u16",
    into = "u16",
    expecting = "a year AoC was held (2015-2022)"
)]
#[repr(u8)]
pub enum AoCYear {
    _2015 = 15,
    _2016 = 16,
    _2017 = 17,
    _2018 = 18,
    _2019 = 19,
    _2020 = 20,
    _2021 = 21,
    _2022 = 22,
}

impl From<AoCYear> for u16 {
    fn from(value: AoCYear) -> Self {
        (value as u8) as u16 + 2000
    }
}

#[derive(Debug, Clone, Copy, Error)]
#[error("AoC was not held in {0}")]
pub struct NotAoCYearError(pub u16);

impl TryFrom<u16> for AoCYear {
    type Error = NotAoCYearError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 2015 || value > 2022 {
            return Err(NotAoCYearError(value));
        }
        let value = (value - 2000) as u8;
        Ok(unsafe {
            /* SAFETY: #[repr(u8)] guarantee this */
            mem::transmute(value)
        })
    }
}

impl Display for AoCYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        u16::from(*self).fmt(f)
    }
}

#[derive(Debug, Clone, Error)]
pub enum ParseAoCYearError {
    #[error(transparent)]
    PareIntError(#[from] ParseIntError),
    #[error(transparent)]
    NotAoCYearError(#[from] NotAoCYearError),
}

impl FromStr for AoCYear {
    type Err = ParseAoCYearError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u16>()?.try_into()?)
    }
}

impl FromMeta for AoCYear {
    fn from_value(value: &Lit) -> darling::Result<Self> {
        (if let Lit::Int(ref s) = *value {
            s.base10_parse::<u16>()
                .map_err(darling::Error::custom)
                .and_then(|v| Self::try_from(v).map_err(darling::Error::custom))
        } else {
            Err(darling::Error::unexpected_lit_type(value))
        })
        .map_err(|e| e.with_span(value))
    }
}

impl ToTokens for AoCYear {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("_{}", u16::from(*self));
        quote!(::aoc_runtime::calendar::AoCYear::#ident).to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(expecting = "a part of an AoC day")]
pub enum AoCPart {
    First,
    Second,
}

impl Display for AoCPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoCPart::First => write!(f, "1"),
            AoCPart::Second => write!(f, "2"),
        }
    }
}

#[derive(Debug, Clone, Error)]
#[error("Invalid AoC part (valid values: 1 or 2)")]
pub struct ParseAoCPartError;

impl FromStr for AoCPart {
    type Err = ParseAoCPartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::First),
            "2" => Ok(Self::Second),
            _ => Err(ParseAoCPartError),
        }
    }
}

impl FromMeta for AoCPart {
    fn from_value(value: &Lit) -> darling::Result<Self> {
        (if let Lit::Int(ref s) = *value {
            s.base10_parse::<u8>()
                .map_err(darling::Error::custom)
                .and_then(|v| match v {
                    1 => Ok(Self::First),
                    2 => Ok(Self::Second),
                    _ => Err(darling::Error::custom(ParseAoCPartError)),
                })
        } else {
            Err(darling::Error::unexpected_lit_type(value))
        })
        .map_err(|e| e.with_span(value))
    }
}

impl ToTokens for AoCPart {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = match self {
            AoCPart::First => format_ident!("First"),
            AoCPart::Second => format_ident!("Second"),
        };
        quote!(::aoc_runtime::calendar::AoCPart::#ident).to_tokens(tokens);
    }
}

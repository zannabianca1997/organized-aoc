use std::{collections::BTreeMap, process::id, str::FromStr};

use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    bracketed,
    parse::Parse,
    parse2, parse_macro_input,
    punctuated::Punctuated,
    token::{self, Bracket, Token},
    LitInt, Path, Token,
};

use crate::calendar::{AoCDay, AoCPart, AoCYear};

impl ToTokens for AoCYear {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("_{}", u16::from(*self));
        quote!(AoCYear::#ident).to_tokens(tokens);
    }
}

impl ToTokens for AoCDay {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("_{}", u8::from(*self));
        quote!(AoCDay::#ident).to_tokens(tokens);
    }
}

impl ToTokens for AoCPart {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = match self {
            AoCPart::First => format_ident!("First"),
            AoCPart::Second => format_ident!("Second"),
        };
        quote!(AoCPart::#ident).to_tokens(tokens);
    }
}

struct LibraryItem {
    year: LitInt,
    arrow: Token![=>],
    path: Path,
}
impl Parse for LibraryItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            year: input.parse()?,
            arrow: input.parse()?,
            path: input.parse()?,
        })
    }
}

struct LibraryInput {
    items: Punctuated<LibraryItem, Token![,]>,
}

impl LibraryInput {
    fn into_map(self) -> Result<BTreeMap<AoCYear, Path>, syn::Error> {
        let mut res = BTreeMap::new();
        for LibraryItem { year, path, .. } in self.items {
            let year = year.base10_parse::<u16>().and_then(|y| {
                AoCYear::try_from(y)
                    .map_err(|err| syn::Error::new_spanned(year, format!("Invalid year: {err}")))
            })?;
            if res.insert(year, path).is_some() {
                return Err(syn::Error::new_spanned(year, "Duplicated year"));
            }
        }
        Ok(res)
    }
}
impl Parse for LibraryInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: input.parse_terminated(LibraryItem::parse, Token![,])?,
        })
    }
}

pub fn library(input: TokenStream) -> TokenStream {
    let map = match parse2(input).and_then(LibraryInput::into_map) {
        Ok(m) => m,
        Err(err) => return err.into_compile_error(),
    };
    let mut years = vec![None; AoCYear::NUM_YEARS];
    for (year, path) in map {
        years[year.idx()] = Some(path)
    }
    let years = years.into_iter().map(|y| match y {
        Some(y) => quote!(::std::option::Option::Some(& #y)),
        None => quote!(::std::option::Option::None),
    });
    quote!(
        ::aoc::Library([#(#years),*])
    )
}

struct YearItem {
    day: LitInt,
    arrow: Token![=>],
    path: Path,
}
impl Parse for YearItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            day: input.parse()?,
            arrow: input.parse()?,
            path: input.parse()?,
        })
    }
}

struct YearInput {
    items: Punctuated<YearItem, Token![,]>,
}

impl YearInput {
    fn into_map(self) -> Result<BTreeMap<AoCDay, Path>, syn::Error> {
        let mut res = BTreeMap::new();
        for YearItem { day, path, .. } in self.items {
            let day = day.base10_parse::<u8>().and_then(|y| {
                AoCDay::try_from(y)
                    .map_err(|err| syn::Error::new_spanned(day, format!("Invalid day: {err}")))
            })?;
            if res.insert(day, path).is_some() {
                return Err(syn::Error::new_spanned(day, "Duplicated day"));
            }
        }
        Ok(res)
    }
}
impl Parse for YearInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: input.parse_terminated(YearItem::parse, Token![,])?,
        })
    }
}

pub fn year(input: TokenStream) -> TokenStream {
    let map = match parse2(input).and_then(YearInput::into_map) {
        Ok(m) => m,
        Err(err) => return err.into_compile_error(),
    };
    let mut days = vec![None; AoCDay::NUM_DAYS];
    for (day, path) in map {
        days[day.idx()] = Some(path)
    }
    let days = days.into_iter().map(|y| match y {
        Some(y) => quote!(::std::option::Option::Some(& #y)),
        None => quote!(::std::option::Option::None),
    });
    quote!(
        ::aoc::Year([#(#days),*])
    )
}

struct DayItemInitSlice {
    bracket: Bracket,
    items: Punctuated<Path, Token![,]>,
}
impl Parse for DayItemInitSlice {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items;
        Ok(Self {
            bracket: bracketed!(items in input),
            items: items.parse_terminated(Path::parse, Token![,])?,
        })
    }
}

enum DayItemInit {
    Path(Path),
    Slice(DayItemInitSlice),
}
impl Parse for DayItemInit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(match input.peek(token::Bracket) {
            true => Self::Slice(input.parse()?),
            false => Self::Path(input.parse()?),
        })
    }
}

struct DayItem {
    ident: Ident,
    colon: Option<Token![:]>,
    init: DayItemInit,
}
impl Parse for DayItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        Ok(match input.peek(Token![:]) {
            true => Self {
                ident,
                colon: Some(input.parse().unwrap()),
                init: input.parse()?,
            },
            false => Self {
                ident: ident.clone(),
                colon: None,
                init: DayItemInit::Path(Path::from(ident)),
            },
        })
    }
}

struct DayInput {
    parts: Punctuated<DayItem, Token![,]>,
}
impl Parse for DayInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            parts: input.parse_terminated(DayItem::parse, Token![,])?,
        })
    }
}
impl DayInput {
    fn into_parts(self) -> syn::Result<[Vec<Path>; 2]> {
        let mut res = [vec![], vec![]];
        for DayItem { ident, init, .. } in self.parts {
            let part = AoCPart::from_str(&ident.to_string()).map_err(|err| {
                syn::Error::new_spanned(ident, format!("Invalid day part: {err}"))
            })?;
            match init {
                DayItemInit::Path(path) => res[part.idx()].push(path),
                DayItemInit::Slice(DayItemInitSlice { items, .. }) => res[part.idx()].extend(items),
            }
        }
        Ok(res)
    }
}

pub fn day(input: TokenStream) -> TokenStream {
    let [part1, part2] = match parse2(input).and_then(DayInput::into_parts) {
        Ok(m) => m,
        Err(err) => return err.into_compile_error(),
    }
    .map(|p| p.into_iter().map(|p| quote!(& #p)));
    quote!(
        ::aoc::Day {
            part1: &[#(#part1),*],
            part2: &[#(#part2),*],
        }
    )
}

struct SolutionParams {
    multiline: bool,
    long_running: bool,
    descr: Option<String>,
}

pub fn solution(attr: TokenStream, item: TokenStream) -> TokenStream {
    todo!()
}

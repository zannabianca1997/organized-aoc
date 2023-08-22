use std::{collections::BTreeMap, mem, str::FromStr};

use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    bracketed,
    parse::Parse,
    parse2,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{self, Bracket},
    Expr, ExprLit, ItemFn, Lit, LitInt, MetaNameValue, Path, Token, Type, TypePath, Visibility,
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

#[allow(unused)]
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
        Some(y) => quote!(& #y),
        None => quote!(&::aoc::Year(
            [&::aoc::Day {
                part1: &[],
                part2: &[]
            }; ::aoc::AoCDay::NUM_DAYS]
        )),
    });
    quote!(
        ::aoc::Library([#(#years),*])
    )
}

#[allow(unused)]
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
        Some(d) => quote!(& #d),
        None => quote!(&::aoc::Day {
            part1: &[],
            part2: &[]
        }),
    });
    quote!(
        ::aoc::Year([#(#days),*])
    )
}

#[allow(unused)]
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

#[allow(unused)]
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

#[derive(FromMeta, Default)]
struct SolutionParams {
    #[darling(default)]
    multiline: bool,
    #[darling(default)]
    long_running: bool,
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    descr: Option<String>,
}

pub fn solution(params: TokenStream, item: TokenStream) -> TokenStream {
    let SolutionParams {
        multiline,
        long_running,
        name,
        descr,
    } = if params.is_empty() {
        SolutionParams::default()
    } else {
        match syn::parse2(quote_spanned!(params.span()=>solution(#params)))
            .map(|meta| SolutionParams::from_meta(&meta))
        {
            Ok(Ok(p)) => p,
            Ok(Err(err)) => return err.write_errors(),
            Err(err) => return err.into_compile_error(),
        }
    };
    let mut item: ItemFn = match syn::parse2(item) {
        Ok(fun) => fun,
        Err(err) => return err.into_compile_error(),
    };
    // taking docs and visibility
    let vis = mem::replace(&mut item.vis, Visibility::Inherited);
    let docs: Vec<_> = item
        .attrs
        .drain_filter(|attr| {
            attr.meta
                .require_name_value()
                .is_ok_and(|MetaNameValue { path, .. }| path.is_ident("doc"))
        })
        .collect();
    let item_name = item.sig.ident.clone();
    // setting defaults
    let name = name.unwrap_or_else(|| item_name.to_string());
    let descr = descr
        .or_else(|| {
            docs.iter()
                .filter_map(|attr| {
                    attr.meta.require_name_value().ok().and_then(
                        |MetaNameValue { value, .. }| {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            }) = value
                            {
                                Some(s.value())
                            } else {
                                None
                            }
                        },
                    )
                })
                .reduce(|mut a, b| {
                    a.push_str(&b);
                    a
                })
        })
        .map(|d| quote!(::std::option::Option::Some(#d)))
        .unwrap_or_else(|| quote!(::std::option::Option::None));
    // Checking if the result is numeric
    let numeric = match &item.sig.output {
        syn::ReturnType::Type(_, box Type::Path(TypePath { qself: None, path }))
            if path.is_ident("i64") =>
        {
            true
        }
        syn::ReturnType::Type(_, box Type::Path(TypePath { qself: None, path }))
            if path.is_ident("String") =>
        {
            false
        }
        out => {
            return syn::Error::new_spanned(out, "Accepted outputs are i64 and String")
                .into_compile_error()
        }
    };

    let block = quote_spanned!(item.span() =>  {
        #item
        #item_name
    });

    let fun = match (numeric, multiline) {
        (true, true) => {
            return syn::Error::new_spanned(
                item.sig.output,
                "i64 is not compatible with multiline output",
            )
            .into_compile_error()
        }
        (true, false) => quote_spanned!(block.span()=>::aoc::SolutionFn::Numeric(#block)),
        (false, true) => quote_spanned!(block.span()=>::aoc::SolutionFn::Multiline(#block)),
        (false, false) => quote_spanned!(block.span()=>::aoc::SolutionFn::Alpha(#block)),
    };

    quote!(
            #(#docs)*
            #[allow(non_upper_case_globals)]
            #vis static #item_name: ::aoc::Solution = ::aoc::Solution {
                name: #name,
                long_running: #long_running,
                descr: #descr,
                fun: #fun,
            };
    )
}

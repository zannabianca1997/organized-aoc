#![feature(box_patterns)]

use either::Either::{Left, Right};
use proc_macro::TokenStream;

use darling::FromMeta;
use quote::{quote, TokenStreamExt};
use quote::{quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::token::Paren;
use syn::{
    FnArg, Ident, ItemFn, Meta, MetaList, PatType, ReturnType, Signature, Type, TypePath,
    TypeReference,
};

use aoc_runtime::calendar::{AoCDay, AoCPart, AoCYear};

#[derive(Debug, Clone, Copy, FromMeta)]
struct AoCMacroArgs {
    pub year: AoCYear,
    pub day: AoCDay,
    pub part: AoCPart,
    #[darling(default)]
    pub long_running: bool,
    #[darling(default)]
    pub multiline: bool,
}

#[derive(Debug, Clone, Copy)]
enum OutputT {
    I64,
    String,
}

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = {
        let attr = proc_macro2::TokenStream::from(attr);
        let attr = quote_spanned! {attr.span()=> aoc(#attr)};
        syn::parse2(attr)
            .map_err(Left)
            .and_then(|attr| AoCMacroArgs::from_meta(&attr).map_err(Right))
    };
    let fun = syn::parse::<ItemFn>(item).and_then(|fun| {
        let (name, output) = check_signature(&fun.sig)?;
        Ok((name, output, fun))
    });
    match (args, fun) {
        (
            Ok(AoCMacroArgs {
                multiline: true, ..
            }),
            Ok((
                _,
                OutputT::I64,
                ItemFn {
                    sig: Signature { output, .. },
                    ..
                },
            )),
        ) => {
            return syn::Error::new_spanned(output, "Multiline is incompatible with i64 output")
                .into_compile_error()
                .into();
        }
        (
            Ok(AoCMacroArgs {
                year,
                day,
                part,
                long_running,
                multiline,
            }),
            Ok((name, output, fun)),
        ) => {
            let mut tokens = proc_macro2::TokenStream::new();
            make_hook(year, day, part, long_running, multiline, name, output)
                .to_tokens(&mut tokens);
            fun.to_tokens(&mut tokens);
            tokens.into()
        }
        (Err(Right(a)), Ok(_)) => a.write_errors().into(),
        (Err(Right(a)), Err(b)) => {
            let mut errs = a.write_errors();
            errs.append_all(b.into_compile_error());
            errs.into()
        }
        (Ok(_), Err(a)) | (Err(Left(a)), Ok(_)) => a.into_compile_error().into(),
        (Err(Left(mut a)), Err(b)) => {
            a.combine(b);
            a.into_compile_error().into()
        }
    }
}

fn make_hook(
    year: AoCYear,
    day: AoCDay,
    part: AoCPart,
    long_running: bool,
    multiline: bool,
    name: Ident,
    output: OutputT,
) -> impl ToTokens {
    let fun = match (multiline, output) {
        (true, OutputT::I64) => panic!("The multiline should never be used with an output of i64"),
        (true, OutputT::String) => quote!(::aoc_runtime::SolutionFn::Multiline(#name)),
        (false, OutputT::I64) => quote!(::aoc_runtime::SolutionFn::Numeric(#name)),
        (false, OutputT::String) => quote!(::aoc_runtime::SolutionFn::Alpha(#name)),
    };
    quote!(
        #[::linkme::distributed_slice(::aoc_runtime::SOLUTIONS)]
        static SOLUTION: ::aoc_runtime::Solution = ::aoc_runtime::Solution{
            year: #year,
            day: #day,
            part: #part,
            long_running: #long_running,
            fun: #fun,
        };
    )
}

fn check_signature(sig: &Signature) -> Result<(Ident, OutputT), syn::Error> {
    let mut errs = vec![];
    let name = &sig.ident;
    let output = match &sig.output {
        ReturnType::Type(_, box Type::Path(TypePath { qself: None, path }))
            if path.is_ident("i64") =>
        {
            OutputT::I64
        }
        ReturnType::Type(_, box Type::Path(TypePath { qself: None, path }))
            if path.is_ident("String") =>
        {
            OutputT::String
        }
        out => {
            errs.push(syn::Error::new_spanned(
                out,
                "Solution must return either i64 or String",
            ));
            OutputT::I64
        }
    };
    if let Some(a) = &sig.asyncness {
        errs.push(syn::Error::new_spanned(
            a,
            "async solution are not supported",
        ));
    };
    if let Some(a) = &sig.unsafety {
        errs.push(syn::Error::new_spanned(
            a,
            "unsafe solution are not supported",
        ));
    };
    if let Some(a) = &sig.variadic {
        errs.push(syn::Error::new_spanned(
            a,
            "Solution must take only a &str for input",
        ));
    };
    if sig.inputs.len() != 1
        || !matches!(
            &sig.inputs[0],
            FnArg::Typed(PatType {
                ty: box Type::Reference(TypeReference{ mutability:None, elem: box Type::Path(TypePath { qself:None, path }),.. }),
                ..
            }) if path.is_ident("str")
        )
    {
        errs.push(syn::Error::new_spanned(
            &sig.inputs,
            "Solution must take only a &str for input",
        ));
    };
    if errs.is_empty() {
        Ok((name.clone(), output))
    } else {
        Err(errs
            .into_iter()
            .reduce(|mut a, b| {
                a.combine(b);
                a
            })
            .unwrap())
    }
}

//! Generator for the element table
#![feature(iterator_try_collect)]

use std::{
    borrow::Cow,
    collections::BTreeSet,
    env,
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

use anyhow::{bail, Result};
use either::Either::{Left, Right};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

struct Element<'i> {
    name: Cow<'i, str>,
    sequence: Cow<'i, [u8]>,
    decay: Vec<Cow<'i, str>>,
    start_set: Option<BTreeSet<u8>>,
}
impl Element<'_> {
    fn parse(l: &str) -> Result<impl IntoIterator<Item = Element>> {
        let [name, sequence, decay] = *l.split('\t').collect::<Vec<_>>() else {
            bail!("Every line should be made of three parts")
        };
        let (parametric, name) = if name.ends_with("(n)") {
            (true, &name[..name.len() - 3])
        } else {
            (false, name)
        };
        let sequence = sequence.as_bytes();

        Ok(if parametric {
            Left((0u8..=9).map(move |n| {
                Element {
                    name: Cow::Owned(format!("{name}{n}")),
                    sequence: Cow::Owned(
                        sequence
                            .into_iter()
                            .map(|b| if *b == b'n' { n } else { *b })
                            .collect(),
                    ),
                    decay: decay
                        .split('.')
                        .map(|d| {
                            if d.ends_with("(n)") {
                                Cow::Owned(d.replace("(n)", &n.to_string()))
                            } else {
                                Cow::Borrowed(d)
                            }
                        })
                        .collect(),
                    start_set: None,
                }
            }))
        } else {
            Right(
                Some(Element {
                    name: Cow::Borrowed(name),
                    sequence: Cow::Borrowed(sequence),
                    decay: decay.split('.').map(Cow::Borrowed).collect(),
                    start_set: None,
                })
                .into_iter(),
            )
        })
    }
}

fn main() -> Result<()> {
    let table_file = PathBuf::from("./src/elements.tsv");
    let table = read_to_string(&table_file)?;
    cargo_emit::rerun_if_changed!(table_file.display());
    let mut elements: Vec<_> = table
        .lines()
        .filter(|l| !l.trim().is_empty())
        .flat_map(|l| match Element::parse(l) {
            Ok(elements) => Left(elements.into_iter().map(Ok)),
            Err(err) => Right(Some(Err(err)).into_iter()),
        })
        .try_collect()?;
    fill_start_sets(&mut elements);

    let mut tokens = TokenStream::new();
    make_enum(&elements, &mut tokens);

    // prettyprinting
    let tokens = prettyplease::unparse(&match syn::parse2(tokens) {
        Ok(f) => f,
        Err(err) => syn::parse2(err.into_compile_error()).unwrap(),
    });

    let out_file = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("elements.rs");
    let mut out = File::create(&out_file)?;
    write!(out, "{}", tokens)?;
    cargo_emit::rustc_env!("ELEMENTS_RS", "{}", out_file.display());
    Ok(())
}

fn fill_start_sets(elements: &mut [Element<'_>]) {
    for i in 0..elements.len() {
        let mut start_set = BTreeSet::from([elements[i].sequence[0]]);
        {
            let mut visited = BTreeSet::from([elements[i].name.as_ref()]);
            let mut j = elements
                .iter()
                .find(|el| el.name.as_ref() == elements[i].decay[0].as_ref())
                .unwrap();
            while !visited.contains(j.name.as_ref()) {
                start_set.insert(j.sequence[0]);
                visited.insert(&j.name);
                j = elements
                    .iter()
                    .find(|el| el.name.as_ref() == j.decay[0].as_ref())
                    .unwrap();
            }
        }
        elements[i].start_set = Some(start_set)
    }
}

fn make_enum(elements: &[Element<'_>], tokens: &mut TokenStream) {
    let variants = elements
        .iter()
        .map(|Element { name, .. }| format_ident!("{name}"));
    let sequences = elements.iter().map(|Element { name, sequence, .. }| {
        let name = format_ident!("{name}");
        quote!( Element::#name => &[#(#sequence),*])
    });
    let decays = elements.iter().map(|Element { name, decay, .. }| {
        let name = format_ident!("{name}");
        let decay = decay.iter().map(|d| format_ident!("{d}"));
        quote!( Element::#name => &[#(Element::#decay),*])
    });
    let start_sets = elements.iter().map(
        |Element {
             name, start_set, ..
         }| {
            let name = format_ident!("{name}");
            let start_set = start_set.as_ref().unwrap();
            quote!( Element::#name => &[#(#start_set),*])
        },
    );
    let from_sequences = elements.iter().map(|Element { name, sequence, .. }| {
        let name = format_ident!("{name}");
        quote!( [#(#sequence),*] => Some(Element::#name))
    });
    quote!(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum Element {
            #(#variants),*
        }

        impl Element {
            const fn sequence(self) -> &'static [u8] {
                match self {
                    #(#sequences,)*
                }
            }
            const fn decay(self) -> &'static [Element] {
                match self {
                    #(#decays,)*
                }
            }
            const fn start_set(self) -> &'static [u8] {
                match self {
                    #(#start_sets,)*
                }
            }

            const fn from_seq(seq:&[u8]) -> Option<Element> {
                match *seq {
                    #(#from_sequences,)*
                    _=>None
                }
            }
        }
    )
    .to_tokens(tokens)
}

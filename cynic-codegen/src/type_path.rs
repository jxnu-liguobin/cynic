use proc_macro2::TokenStream;

use crate::ident::Ident;

/// The path to a type e.g. serde_json::Value
///
/// Implements ToToken so it can be used inside quote!
#[derive(Clone, Debug)]
pub struct TypePath {
    path: Vec<PathElement>,
    relative: bool,
    is_void: bool,
    builtin: bool,
}

#[derive(Clone, Debug)]
enum PathElement {
    Super,
    Ident(Ident),
}

impl TypePath {
    pub fn new(path: Vec<Ident>) -> Self {
        TypePath {
            path: path.into_iter().map(Into::into).collect(),
            relative: true,
            is_void: false,
            builtin: false,
        }
    }

    pub fn new_absolute(path: Vec<Ident>) -> Self {
        TypePath {
            path: path.into_iter().map(Into::into).collect(),
            relative: false,
            is_void: false,
            builtin: false,
        }
    }

    pub fn new_builtin(ident: Ident) -> Self {
        TypePath {
            path: vec![ident.into()],
            relative: false,
            is_void: false,
            builtin: true,
        }
    }

    pub fn new_super() -> Self {
        TypePath {
            path: vec![PathElement::Super],
            relative: true,
            is_void: false,
            builtin: false,
        }
    }

    pub fn void() -> Self {
        TypePath {
            path: vec![],
            relative: false,
            is_void: true,
            builtin: false,
        }
    }

    pub fn concat(paths: &[TypePath]) -> Self {
        let relative = if let Some(path) = paths.get(0) {
            path.relative
        } else {
            false
        };
        let mut result_path = vec![];

        for type_path in paths {
            for path in &type_path.path {
                result_path.push(path.clone());
            }
        }

        TypePath {
            path: result_path,
            relative,
            is_void: false,
            builtin: false,
        }
    }

    pub fn empty() -> Self {
        TypePath {
            path: vec![],
            relative: true,
            is_void: false,
            builtin: false,
        }
    }

    pub fn push(&mut self, ident: Ident) {
        self.path.push(ident.into());
    }

    pub fn is_absolute(&self) -> bool {
        !self.relative
    }
}

impl From<Ident> for TypePath {
    fn from(ident: Ident) -> TypePath {
        TypePath {
            path: vec![ident.into()],
            relative: true,
            is_void: false,
            builtin: false,
        }
    }
}

impl From<Ident> for PathElement {
    fn from(ident: Ident) -> PathElement {
        PathElement::Ident(ident)
    }
}

impl quote::ToTokens for PathElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use quote::{quote, TokenStreamExt};

        tokens.append_all(match self {
            PathElement::Ident(ident) => {
                quote! { #ident }
            }
            PathElement::Super => {
                quote! { super }
            }
        });
    }
}

impl quote::ToTokens for TypePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use quote::{quote, TokenStreamExt};

        if self.is_void {
            tokens.append_all(quote! { () });
            return;
        }

        let initial = if !self.relative && !self.path.is_empty() && !self.builtin {
            Some(quote! { :: })
        } else {
            None
        };

        let path = &self.path;

        tokens.append_all(quote! {
            #initial
            #(
                 #path
            )::*
        })
    }
}

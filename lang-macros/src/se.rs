use lang_inner::*;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::*;

macro_rules! array_member {
	($s:ident, $($m:ident),*$(,)?) => {
		vec![$(array_member!(@inner, $s, $m)),*]
	};
	(@inner, $s:ident, $m:ident) => {
		FieldValue {
			attrs: vec![],
			member: Member::Named(Ident::new(stringify!($m), Span::mixed_site())),
			colon_token: Some(Default::default()),
			expr: $s.$m.serialize(),
		}
	};
}

pub(crate) trait Serialize {
    fn serialize(self) -> Expr;
}

impl<const N: usize, T: Serialize> Serialize for [T; N] {
    fn serialize(self) -> Expr {
        Expr::Array(ExprArray {
            attrs: vec![],
            bracket_token: Default::default(),
            elems: Punctuated::from_iter(self.map(|t| t.serialize())),
        })
    }
}

impl Serialize for &str {
    fn serialize(self) -> Expr {
        Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Str(LitStr::new(self, Span::mixed_site())),
        })
    }
}

impl Serialize for u32 {
    fn serialize(self) -> Expr {
        Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Int(LitInt::new(&*self.to_string(), Span::mixed_site())),
        })
    }
}

impl Serialize for char {
    fn serialize(self) -> Expr {
        Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Char(LitChar::new(self, Span::mixed_site())),
        })
    }
}

impl Serialize for bool {
    fn serialize(self) -> Expr {
        Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Bool(LitBool::new(self, Span::mixed_site())),
        })
    }
}

impl<A: Serialize> Serialize for (A,) {
    fn serialize(self) -> Expr {
        Expr::Tuple(ExprTuple {
            attrs: vec![],
            paren_token: Default::default(),
            elems: Punctuated::from_iter(vec![self.0.serialize()]),
        })
    }
}

impl<A: Serialize, B: Serialize> Serialize for (A, B) {
    fn serialize(self) -> Expr {
        Expr::Tuple(ExprTuple {
            attrs: vec![],
            paren_token: Default::default(),
            elems: Punctuated::from_iter(vec![self.0.serialize(), self.1.serialize()]),
        })
    }
}

impl<A: Serialize, B: Serialize, C: Serialize> Serialize for (A, B, C) {
    fn serialize(self) -> Expr {
        Expr::Tuple(ExprTuple {
            attrs: vec![],
            paren_token: Default::default(),
            elems: Punctuated::from_iter(vec![
                self.0.serialize(),
                self.1.serialize(),
                self.2.serialize(),
            ]),
        })
    }
}

impl Serialize for LanguageRaw<'_> {
    fn serialize(self) -> Expr {
        Expr::Struct(ExprStruct {
            attrs: vec![],
            qself: None,
            path: Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![PathSegment {
                    ident: Ident::new("LanguageRaw", Span::mixed_site()),
                    arguments: Default::default(),
                }]),
            },
            brace_token: Default::default(),
            fields: Punctuated::from_iter(vec![
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("name", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: self.name.serialize(),
                },
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("left_right", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: self.left_right.serialize(),
                },
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("keywords", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: self.keywords.serialize(),
                },
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("messages", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: self.messages.serialize(),
                },
            ]),
            dot2_token: None,
            rest: None,
        })
    }
}

impl Serialize for Messages<'_> {
    fn serialize(self) -> Expr {
        Expr::Struct(ExprStruct {
            attrs: vec![],
            qself: None,
            path: Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![PathSegment {
                    ident: Ident::new("Messages", Span::mixed_site()),
                    arguments: Default::default(),
                }]),
            },
            brace_token: Default::default(),
            fields: Punctuated::from_iter(vec![
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("errors", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: self.errors.serialize(),
                },
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("warnings", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: self.warnings.serialize(),
                },
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("cli_keywords", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: self.cli_keywords.serialize(),
                },
            ]),
            dot2_token: None,
            rest: None,
        })
    }
}

impl Serialize for Keywords<'_> {
    fn serialize(self) -> Expr {
        let mut elems = array_member!(
            self,
            keywords,
            type_kwds,
            builtins,
            bool,
            symbol_keys,
            shell_keys,
            manifest_keys
        );
        elems.push(FieldValue {
            attrs: vec![],
            member: Member::Named(Ident::new("digits", Span::mixed_site())),
            colon_token: Some(Default::default()),
            expr: self.digits.serialize(),
        });
        elems.push(FieldValue {
            attrs: vec![],
            member: Member::Named(Ident::new("manifest_keys_short", Span::mixed_site())),
            colon_token: Some(Default::default()),
            expr: Expr::Array(ExprArray {
                attrs: vec![],
                bracket_token: Default::default(),
                elems: Punctuated::from_iter(self.manifest_keys_short.iter().map(|t| {
                    if let Some(t) = t {
                        Expr::Call(ExprCall {
                            attrs: vec![],
                            func: Box::new(Expr::Path(ExprPath {
                                attrs: vec![],
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: Punctuated::from_iter(vec![PathSegment {
                                        ident: Ident::new("Some", Span::mixed_site()),
                                        arguments: Default::default(),
                                    }]),
                                },
                            })),
                            paren_token: Default::default(),
                            args: Punctuated::from_iter(vec![Expr::Lit(ExprLit {
                                attrs: vec![],
                                lit: Lit::Str(LitStr::new(t, Span::mixed_site())),
                            })]),
                        })
                    } else {
                        Expr::Path(ExprPath {
                            attrs: vec![],
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: Punctuated::from_iter(vec![PathSegment {
                                    ident: Ident::new("None", Span::mixed_site()),
                                    arguments: Default::default(),
                                }]),
                            },
                        })
                    }
                })),
            }),
        });
        elems.push(array_member!(@inner, self, compile_words));
        Expr::Struct(ExprStruct {
            attrs: vec![],
            qself: None,
            path: Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![PathSegment {
                    ident: Ident::new("Keywords", Span::mixed_site()),
                    arguments: Default::default(),
                }]),
            },
            brace_token: Default::default(),
            fields: Punctuated::from_iter(elems),
            dot2_token: None,
            rest: None,
        })
    }
}

impl Serialize for Digits {
    fn serialize(self) -> Expr {
        match self {
            Digits::Short { digits, u8arrays } => {
                let u8arrays = u8arrays.iter().map(|(t, s)| quote!(([#(#t),*], #s)));
                Expr::Verbatim(quote!(Digits::Short {
                    digits: [#(#digits),*],
                    u8arrays: [#(#u8arrays),*]
                }))
            },
            Digits::Long { digits, u8arrays } => {
                let u8arrays = u8arrays.iter().map(|(t, s)| quote!(([#(#t),*], #s)));
                Expr::Verbatim(quote!(Digits::Long {
                    digits: [#(#digits),*],
                    u8arrays: [#(#u8arrays),*]
                }))
            }
        }
    }
}

impl Serialize for Errors<'_> {
    fn serialize(self) -> Expr {
        Expr::Struct(ExprStruct {
            attrs: vec![],
            qself: None,
            path: Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![PathSegment {
                    ident: Ident::new("Errors", Span::mixed_site()),
                    arguments: Default::default(),
                }]),
            },
            brace_token: Default::default(),
            fields: Punctuated::from_iter(array_member!(self, e00, e01, e02, e03, e04)),
            dot2_token: None,
            rest: None,
        })
    }
}

impl Serialize for Warns<'_> {
    fn serialize(self) -> Expr {
        Expr::Struct(ExprStruct {
            attrs: vec![],
            qself: None,
            path: Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![PathSegment {
                    ident: Ident::new("Warns", Span::mixed_site()),
                    arguments: Default::default(),
                }]),
            },
            brace_token: Default::default(),
            fields: Punctuated::from_iter(array_member!(self, w00, w01, w02, w03, w04)),
            dot2_token: None,
            rest: None,
        })
    }
}

impl Serialize for CLIKeywords<'_> {
    fn serialize(self) -> Expr {
        Expr::Struct(ExprStruct {
            attrs: vec![],
            qself: None,
            path: Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![PathSegment {
                    ident: Ident::new("CLIKeywords", Span::mixed_site()),
                    arguments: Default::default(),
                }]),
            },
            brace_token: Default::default(),
            fields: Punctuated::from_iter(vec![
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("desc", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: Expr::Lit(ExprLit {
                        attrs: vec![],
                        lit: Lit::Str(LitStr::new(self.desc, Span::mixed_site())),
                    }),
                },
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("commands", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: Expr::Array(ExprArray {
                        attrs: vec![],
                        bracket_token: Default::default(),
                        elems: Punctuated::from_iter(self.commands.iter().map(|c| {
                            Expr::Tuple(ExprTuple {
                                attrs: vec![],
                                paren_token: Default::default(),
                                elems: Punctuated::from_iter(vec![
                                    Expr::Lit(ExprLit {
                                        attrs: vec![],
                                        lit: Lit::Str(LitStr::new(c.0, Span::mixed_site())),
                                    }),
                                    Expr::Lit(ExprLit {
                                        attrs: vec![],
                                        lit: Lit::Str(LitStr::new(c.1, Span::mixed_site())),
                                    }),
                                ]),
                            })
                        })),
                    }),
                },
                FieldValue {
                    attrs: vec![],
                    member: Member::Named(Ident::new("args", Span::mixed_site())),
                    colon_token: Some(Default::default()),
                    expr: Expr::Array(ExprArray {
                        attrs: vec![],
                        bracket_token: Default::default(),
                        elems: Punctuated::from_iter(self.args.iter().map(|c| c.serialize())),
                    }),
                },
            ]),
            dot2_token: None,
            rest: None,
        })
    }
}

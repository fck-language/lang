use crate::compress::unique_stream::UStream;
use crate::compress::Zero;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use std::ops::Index;
use syn::punctuated::Punctuated;
use syn::{
    Expr, ExprStruct, FieldValue, Member, Path, PathSegment,
};

/// Generate a [`Punctuated<FieldValue, _>`][Punctuated] from provided field names. Each name (`$t`)
/// expands to the following
/// ```ignore
/// FieldValue {
/// 	attrs: vec![],
/// 	member: Member::Named(Ident::new(stringify!($t), Span::mixed_site())),
/// 	colon_token: Some(Default::default()),
/// 	expr: Expr::Verbatim(quote!{[#(#$t),*]})
/// }
/// ```
macro_rules! fields {
    ($($t:ident),*$(,)?) => {
		Punctuated::from_iter([$(
			FieldValue {
				attrs: vec![],
				member: Member::Named(Ident::new(stringify!($t), Span::mixed_site())),
				colon_token: Some(Default::default()),
				expr: Expr::Verbatim(quote!{[#(#$t),*]}),
			}
		),*])
	};
}

impl<D, S, OR, OF> ToTokens for UStream<D, S, OR, OF>
where
    D: Copy + Clone + Sized + Zero + PartialEq + ToTokens,
    S: Index<usize, Output = D> + Sized + IntoIterator<Item = D> + Clone,
    OR: Index<usize, Output = u16> + Sized + IntoIterator<Item = u16> + Clone,
    OF: Index<usize, Output = usize> + Sized + IntoIterator<Item = usize> + Clone,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let stream = self.stream.clone().into_iter().map(|t| quote! {#t});
        let origin = self.origin.clone().into_iter().map(|t| quote! {#t});
        let offsets = self.offsets.clone().into_iter().map(|t| quote! {#t});
        ExprStruct {
            attrs: vec![],
            qself: None,
            path: Path::from(PathSegment::from(Ident::new("UStream", Span::mixed_site()))),
            brace_token: Default::default(),
            fields: fields!(stream, origin, offsets),
            dot2_token: None,
            rest: None,
        }
        .to_tokens(tokens)
    }
}

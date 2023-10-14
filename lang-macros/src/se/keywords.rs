use lang_inner::prelude::{Keywords, Digits, DigitsRaw, ControlKwds, TypeKwds, PrimitiveKwds, BoolKwds, ManifestKwds, CompileKwds};
use proc_macro2::TokenStream;
use quote::quote;
use crate::se::Serialize;


impl Serialize for Keywords<'_> {
    fn serialize(self) -> TokenStream {
        let digits = self.digits.serialize();
        let keywords = self.keywords.serialize();
        let type_kwds = self.type_kwds.serialize();
        let builtins = self.builtins.serialize();
        let bool = self.bool.serialize();
        let manifest_keys = self.manifest_keys.serialize();
        let compile_words = self.compile_words.serialize();
        quote!{ Keywords {
            digits: #digits, keywords: #keywords, type_kwds: #type_kwds, builtins: #builtins,
            bool: #bool, manifest_keys: #manifest_keys, compile_words: #compile_words
        } }
    }
}


impl Serialize for Digits {
    fn serialize(self) -> TokenStream {
        let (name, inner) = match self {
            Digits::Short(inner) => (quote!{ Digits::Short }, inner.serialize()),
            Digits::Long(inner) => (quote!{ Digits::Long }, inner.serialize()),
        };
        quote!{ #name(#inner) }
    }
}


impl<const N: usize> Serialize for DigitsRaw<N> {
    fn serialize(self) -> TokenStream {
	    let bin_pre = self.bin_pre;
	    let bin_pre = quote!{ bin_pre: #bin_pre };
        let bin_pre_u8_0 = self.bin_pre_u8.0;
        let bin_pre_u8_1 = self.bin_pre_u8.1;
	    let bin_pre_u8 = quote!{ bin_pre_u8: ([#(#bin_pre_u8_0),*], #bin_pre_u8_1) };
	    let hex_pre = self.hex_pre;
	    let hex_pre = quote!{ hex_pre: #hex_pre };
        let hex_pre_u8_0 = self.hex_pre_u8.0;
        let hex_pre_u8_1 = self.hex_pre_u8.1;
	    let hex_pre_u8 = quote!{ hex_pre_u8: ([#(#hex_pre_u8_0),*], #hex_pre_u8_1) };
	    let oct_pre = self.oct_pre;
	    let oct_pre = quote!{ oct_pre: #oct_pre };
        let oct_pre_u8_0 = self.oct_pre_u8.0;
        let oct_pre_u8_1 = self.oct_pre_u8.1;
	    let oct_pre_u8 = quote!{ oct_pre_u8: ([#(#oct_pre_u8_0),*], #oct_pre_u8_1) };
	    let digits_mapped = self.digits.map(|t| quote!{ #t });
	    let digits = quote!{ digits: [#(#digits_mapped),*] };
        let u8arrays_mapped = self.u8arrays.map(|(a, b)| quote!{ ([#(#a),*], #b) });
	    let u8arrays = quote!{ u8arrays: [#(#u8arrays_mapped),*] };
        quote!{ DigitsRaw {
            #bin_pre, #bin_pre_u8, #hex_pre, #hex_pre_u8, #oct_pre, #oct_pre_u8, #digits, #u8arrays
        } }
    }
}


impl Serialize for ControlKwds<'_> {
    fn serialize(self) -> TokenStream {
        macro_rules! fields {
            ($($name:ident),*$(,)?) => { quote!{
                ControlKwds { $($name: #$name),* }
            } };
        }
        let k_set = self.k_set;
        let k_and = self.k_and;
        let k_or = self.k_or;
        let k_not = self.k_not;
        let k_if = self.k_if;
        let k_else = self.k_else;
        let k_match = self.k_match;
        let k_repeat = self.k_repeat;
        let k_for = self.k_for;
        let k_in = self.k_in;
        let k_to = self.k_to;
        let k_as = self.k_as;
        let k_while = self.k_while;
        let k_fn = self.k_fn;
        let k_return = self.k_return;
        let k_continue = self.k_continue;
        let k_break = self.k_break;
        let k_where = self.k_where;
        fields!(
            k_set, k_and, k_or, k_not, k_if, k_else, k_match, k_repeat, k_for, k_in, k_to,
            k_as, k_while, k_fn, k_return, k_continue, k_break, k_where
        )
    }
}


impl Serialize for TypeKwds<'_> {
    fn serialize(self) -> TokenStream {
        macro_rules! fields {
            ($($name:ident),*$(,)?) => { quote!{
                TypeKwds { $($name: #$name),* }
            } };
        }
        let k_struct = self.k_struct;
		let k_properties = self.k_properties;
		let k_enum = self.k_enum;
		let k_variants = self.k_variants;
		let k_self = self.k_self;
		let k_Self = self.k_Self;
		let k_extension = self.k_extension;
		let k_extend = self.k_extend;
		let k_const = self.k_const;
        fields!(
            k_struct, k_properties, k_enum, k_variants, k_self, k_Self, k_extension, k_extend, k_const
        )
    }
}


impl Serialize for PrimitiveKwds<'_> {
    fn serialize(self) -> TokenStream {
        macro_rules! fields {
            ($($name:ident),*$(,)?) => { quote!{
                PrimitiveKwds { $($name: #$name),* }
            } };
        }
        let k_int = self.k_int;
		let k_uint = self.k_uint;
		let k_dint = self.k_dint;
		let k_udint = self.k_udint;
		let k_float = self.k_float;
		let k_bfloat = self.k_bfloat;
		let k_str = self.k_str;
		let k_char = self.k_char;
		let k_list = self.k_list;
		let k_bool = self.k_bool;
        fields!(
            k_int, k_uint, k_dint, k_udint, k_float, k_bfloat, k_str, k_char, k_list, k_bool
        )
    }
}


impl Serialize for BoolKwds<'_> {
    fn serialize(self) -> TokenStream {
        macro_rules! fields {
            ($($name:ident),*$(,)?) => { quote!{
                BoolKwds { $($name: #$name),* }
            } };
        }
        let k_true = self.k_true;
        let k_false = self.k_false;
        fields!(k_true, k_false)
    }
}


impl Serialize for ManifestKwds<'_> {
    fn serialize(self) -> TokenStream {
        macro_rules! fields {
            ($($name:ident),*$(,)?) => { quote!{
                ManifestKwds { $($name: #$name),* }
            } };
        }
        let k_package = self.k_package;
		let k_name = self.k_name;
		let k_src = self.k_src;
		let k_tests = self.k_tests;
		let k_benches = self.k_benches;
		let k_type = self.k_type;
		let k_lib = self.k_lib;
		let k_app = self.k_app;
		let k_version = self.k_version;
		let k_authors = self.k_authors;
		let k_github = self.k_github;
		let k_gitlab = self.k_gitlab;
		let k_email = self.k_email;
		let k_license = self.k_license;
		let k_description = self.k_description;
		let k_readme = self.k_readme;
		let k_homepage = self.k_homepage;
        let k_repo = self.k_repo;
		let k_features = self.k_features;
		let k_dependencies = self.k_dependencies;
		let k_usage = self.k_usage;
		let k_git = self.k_git;
		let k_branch = self.k_branch;
		let k_path = self.k_path;
		let k_dev = self.k_dev;
		let k_build = self.k_build;
		let k_main = self.k_main;
        fields!(
			k_package, k_name, k_src, k_tests, k_benches, k_type, k_lib, k_app, k_version,
			k_authors, k_github, k_gitlab, k_email, k_license, k_description, k_readme,
			k_homepage, k_repo, k_features, k_dependencies, k_usage, k_git, k_branch, k_path,
			k_dev, k_build, k_main
		)
    }
}


impl Serialize for CompileKwds<'_> {
    fn serialize(self) -> TokenStream {
        macro_rules! fields {
            ($($name:ident),*$(,)?) => { quote!{
                CompileKwds { $($name: #$name),* }
            } };
        }
        let k_Compiling = self.k_Compiling;
		let k_Building = self.k_Building;
		let k_Built = self.k_Built;
		let k_Linking = self.k_Linking;
		let k_Emitted = self.k_Emitted;
		let k_Error = self.k_Error;
		let k_errors = self.k_errors;
		let k_Warning = self.k_Warning;
		let k_warning = self.k_warning;
        fields!(k_Compiling, k_Building, k_Built, k_Linking, k_Emitted, k_Error, k_errors, k_Warning, k_warning)
    }
}

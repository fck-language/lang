//! Language file for German
//!
//! **Development version**

use crate::prelude::*;

/// Raw german language definition
pub const LANG: LanguageRaw = LanguageRaw {
	name: ("Deutsch", "de"),
	keywords: Keywords {
		keywords: ["lassen", "und", "oder", "nicht", "wenn", "sonst", "sonn", "falls", "option",
			"standard", "iterieren", "bis", "importieren", "stufe", "während", "def",
			"zurückschicken", "fortsetzen", "ausbrechen", "still", "als"],
		builtins: ["ganze", "fließkomma", "bool", "zeich", "liste", "stadtplan"],
		bool: ["wahr", "falsch"],
		symbols: ["#", "#", "\"", "\"", "c\"", "\"", "###"],
		// TODO
		symbol_keys: ["comment_start", "comment_end", "string_start", "string_end", "char_start", "char_end", "doc_comment"],
		// TODO
		shell_keys: ["wrapLength", "historyLength", "name"],
		// TODO
		manifest_keys: ["package", "name", "version", "edition", "description", "authors", "github", "email", "twitter", "repository", "homepage", "readme", "license", "features", "dependencies"],
		// TODO
		manifest_keys_short: [None, None, Some("v"), None, Some("desc"), Some("authors"), None, None, None, Some("repo"), None, None, None, None, Some("deps")],
		// TODO
		compile_words: ["Compiling project", "Building", "Built", "Linking", "Emitted", "errors", "warnings"]
	},
	errors: Errors {
		e00: ["", "", "", "", "", "", ""],
		e01: ["", ""],
		e02: ["", "", "", "", "", "", "", "", ""],
		e03: [""],
		e04: ["", ""]
	},
	warnings: Warns {
		w00: [],
		w01: [],
		w02: [],
		w03: [],
		w04: []
	},
	cli_keywords: CLIKeywords {
		desc: "fck command line interface",
		commands: [
			("neu", ""),
			("shell", ""),
			("bauen", ""),
			("laufen", ""),
			("testen", ""),
			("info", ""),
			("lint", ""),
			("roh", ""),
			("dok", ""),
			("translate", "")
		],
		args: [
			Arg("path", 'p', ""),
			Arg("git", 'g', ""),
			Arg("dump-llvm", 'd', ""),
			Arg("kein-bauen", 'n', ""),
			Arg("testen", 't', ""),
			Arg("raw", 'r', ""),
			Arg("target", 'l', ""),
			Arg("output", 'o', ""),
			Arg("comment", 'c', "")
		]
	},
};

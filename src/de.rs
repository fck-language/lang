//! Language file for German
//!
//! **Development version**

use crate::keywords::*;

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["und", "oder", "nicht", "wenn", "sonst", "sonn", "falls", "option", "standard",
        "iterieren", "bis", "importieren/verwenden", /* import/use */
        "stufe", "während", "def", "zurückschicken", "fortsetzen", "ausbrechen", "still", "als",
        "wahr", "falsch"
    ],
    var_keywords:
    // TODO: Work out the actual words
    ["ganze", "fließkomma", "bool", "liste", "zeich"],
    config_keys:
    // TODO: Work out the actual words
    ["wrapLength", "shellLanguageChange", "historyLength", "name", "github", "email"],
    manifest_keys:
    ["project", "name", "default", "description", "authors", "github", "email", "repository",
        "homepage", "publish", "license", "readme", "categories", "dependencies"],
    debug_words:
    ["Tokens", "ASTs", "Symbol tables", "LLVM IR generated", "Writing to file", "Unable to write to file", "Written LLVM IR to file"],
    flavours:
    ["pure", "counting"]
};

pub const CLI_KEYWORDS: CLIKeywords = CLIKeywords {
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
};

pub const MESSAGES: Messages = Messages{
    generic: ["Die Shell-Sprache wurde auf Deutsch geändert"],
    errors: ErrorHolder{
        language_errors: [
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" }
        ],
        unknown_errors: [
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" }
        ],
        expected_errors: [
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" }
        ],
        not_here_errors: [
            ErrorMessages{ name: "", desc: "" }
        ],
        type_errors: [
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "Type with type ID _ does not exist", desc: "Returned when one type was found that cannot be cast into the required type" }
        ]
    }
};

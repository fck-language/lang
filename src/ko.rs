﻿//! Language file for Korean
//!
//! **Development version**

use crate::keywords::*;

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["그리고", "or", "not", "면", "else", "elif", "case", "option", "default",
        "iterate", "to", "import", "step", "while", "def", "return", "continue", "break",
        "silent", "as", "true", "false"],
    var_keywords:
    ["int", "float", "bool", "list", "str"],
    config_keys:
    ["wrapLength", "shellLanguageInfo", "historyLength"],
    manifest_keys:
    ["[package]", "name", "version", "authors", "edition", "flavour", "[dependencies]"],
    debug_words:
    ["Tokens", "ASTs", "Symbol tables", "LLVM IR generated", "Writing to file", "Unable to write to file", "Written LLVM IR to file"],
    cli_commands:
    ["test", "info", "run", "build", "lint", "raw", "shell", "doc"],
    cli_args:
    ["debug", "duml-llvm"],
    flavours:
    ["pure", "counting"]
};

pub const MESSAGES: Messages = Messages{
    generic: ["The shell language has been changed to Korean"],
    errors: ErrorHolder{
        language_errors: [
            ErrorMessages{ name: "Unknown language code", desc: "Returned when an unknown language code is specified" },
            ErrorMessages{ name: "Incomplete language file", desc: "Returned when trying to use an incomplete language file" }
        ],
        unknown_errors: [
            ErrorMessages{ name: "Unknown character", desc: "Returned when you use a character that fck doesn't understand" },
            ErrorMessages{ name: "Unknown operator", desc: "You tried some sort of operation that I just don't know" }
        ],
        expected_errors: [
            ErrorMessages{ name: "Expected newline", desc: "Expected a newline or end or file" },
            ErrorMessages{ name: "Expected a condition", desc: "Expected a conditional statement here" },
            ErrorMessages{ name: "Expected opening bracket", desc: "Expected an opening bracket here" },
            ErrorMessages{ name: "Expected identifier", desc: "Expected an identifier" },
            ErrorMessages{ name: "Expected expression", desc: "" },
            ErrorMessages{ name: "Expected assignment operator", desc: "" },
            ErrorMessages{ name: "Expected colon (:)", desc: "" },
            ErrorMessages{ name: "Expected closing bracket", desc: "" },
            ErrorMessages{ name: "Expected type identifier", desc: "" }
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

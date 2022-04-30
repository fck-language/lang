//! Language file for French
//!
//! **Development version**

use crate::keywords::*;

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["et", "ou", "non", "si", "autre", "auti", "cas", "option", "défaut",
        "répéter", "à", "import", "pas", "pendant", "déf", "rendre", "continuer", "interruption",
        "muet", "comme", "vrai", "faux"],
    var_keywords:
    ["ent", "flottante", "bool", "liste", "chaîne"],
    config_keys:
    // TODO: Work out the actual words
    ["wrapLength", "shellLanguageChange", "historyLength", "name", "github", "email"],
    manifest_keys:
    ["project", "name", "default", "description", "authors", "github", "email", "repository",
        "homepage", "publish", "license", "readme", "categories", "dependencies"],
    debug_words:
    ["Tokens", "ASTs", "Symbol tables", "LLVM IR generated", "Writing to file", "Unable to write to file", "Written LLVM IR to file"],
    cli_commands:
    ["test", "info", "run", "build", "lint", "raw", "shell", "doc", "new"],
    cli_args:
    ["debug", "duml-llvm"],
    flavours:
    ["pure", "counting"]
};

pub const MESSAGES: Messages = Messages{
    generic: ["La langue du shell a été changée en français"],
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

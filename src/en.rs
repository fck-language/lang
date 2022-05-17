//! Language file for English

use crate::keywords::*;

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["and", "or", "not", "if", "else", "elif", "case", "option", "default",
        "iterate", "to", "import", "step", "while", "def", "return", "continue", "break",
        "silent", "as", "true", "false"],
    var_keywords:
    ["int", "float", "bool", "list", "str"],
    config_keys:
    ["wrapLength", "shellLanguageInfo", "historyLength", "name", "github", "email"],
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
        ("new", "Generate a new project"),
        ("shell", "Run the shell"),
        ("build", "Build the specified project or file"),
        ("run", "Run the specified project after (optionally) building"),
        ("test", "Test the given project using all or some tests"),
        ("info", "Get info about the current fck version"),
        ("lint", "Lint a project depending on the style file"),
        ("raw", "Run a raw piece of fck code"),
        ("doc", "Generate the documentation for a project"),
        ("translate", "Translate a file or project into a target language")
    ],
    args: [
        Arg("path", 'p', "Path to file or directory"),
        Arg("git", 'g', "Initialise the new project as a git repository"),
        Arg("dump-llvm", 'd', "Dump the LLVM IR to a file"),
        Arg("no-build", 'n', "Don't build before running the command"),
        Arg("test", 't', "Path like string to a specific file, module, or test function to run. Can be given more than once"),
        Arg("raw", 'r', "Raw string to run"),
        Arg("target", 'l', "Language to translate the code into"),
        Arg("output", 'o', "Path to output the translated file to"),
        Arg("comment", 'c', "Include the comments in translation using LibreTranslate")
    ]
};

pub const MESSAGES: Messages = Messages{
    generic: ["The shell language has been changed to English"],
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
            ErrorMessages{ name: "Cannot use a keyword here", desc: "Need to use an identifier that's not a keyword" }
        ],
        type_errors: [
            ErrorMessages{ name: "Expected type _ got _", desc: "Returned when one type was found that cannot be cast into the required type" },
            ErrorMessages{ name: "Type with type ID _ does not exist", desc: "Returned when one type was found that cannot be cast into the required type" }
        ],
    }
};

# fck Lang files

These files are the language files used to allow fck to have multilingual support. This README outlines the layout of files and what is in each one.

## Contents
- [File names](#file-names)
- [What goes in the file](#file-contents)
- [Non-UTF-8](#non-utf-8-characters)
- [Dependencies](#dependencies)

## File names

Each file name is the [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) language code for the language. For example `cn.rs` is the language file for Chinese

## File contents

Each file has to have the following `pub const` structs:
- `pub const KEYWORDS: Keywords`
- `pub const CLI_KEYWORDS: CLIKeywords`
- `pub const MESSAGES: Messages`

If you want to add a language, duplicate a currently existing (advisably non-developmental) language file, modify the file name to the language code for the language you'll be adding, and modify the file.

## Non-UTF-8 characters

All the files have to be in UTF-8 because Rust will have a meltdown otherwise. If you have to use non-UTF-8 characters (for example Korean characters), just type them as normal and it somehow works in the back

# Dependencies

This crate has one dependency, [`clap`](https://crates.io/crates/clap/3.1.18). We use this to parse command line arguments and do errors (hopefully in several languages I haven't checked)

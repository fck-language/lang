# fck languages

![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/fck-language/lang/test.yaml?style=for-the-badge&label=Tests)

This repo contains anything language based and is split into three crates:
- `lang` Root crate
- [`lang-inner`] Main crate 
- [`lang-macros`] Proc macro crate

`lang` is primarily doing some re-exports of `lang-inner` and using the `language` macro from `lang-macros` to include languages at compile time.

> There is also the [`table-page`] crate. This is an optional dependency of `lang-macros` and is used for debugging only. It generates simple web pages of the NFA tables for simpler debugging

## Language files

The currently supported language files are in the [languages](languages) directory. If you want to add a new language, we recommend modifying an existing language file, so you're more likely to get it right first time instead of trying to make one from scratch using the specification.

## What does `lang` do

This root crate uses the `language` macro defined in [`lang-inner`] to include some languages and make a couple of functions to access these languages based on their name.

It also re-exports some parts of [`lang-inner`] (specifically `lang_inner::{prelude::*, tok::*}` in a module called `prelude`).

## Adding an official language

If you want to add a language to fck, you'll need a few things:
1. A language file
2. An ISO 639-1 or 639-2 code for the language
3. Some people to verify the language

The first two are fairly simple. To add a language you need a correctly named language file. fck has chosen to use a mixture of ISO 639-1 and 639-2 codes for the official language files. This decision was made to maximise the number of official languages that we could support (ISO 639-2), whilst still maintaining some more recognisable language codes (ISO 639-2).

There are caveats to this however. If you want to add a language using a non-Latin alphabet and have a widely used abbreviation for the language you're adding in a similar style to ISO-639, then we recommend you use that.

The last part is having people to verify the language. This makes sure that any mistakes or misspellings you missed are more likely to be caught, and allows for different ideas on translation. If you're using a non ISO 639 code for your language abbreviation, this will also need to be verified.

### Maintenance

Once you submit a language, you're added as the maintainer of that language file as well. This is a pretty simple task and requires very little effort. If anything is ever added or modified, then the language files will also have to change. This will most likely be that an error or warning was added, or (less likely) be that a keyword or built-in type was added.

This repo contains all the built-in and officially supported languages in fck. The actual languages are contained in the [`src/languages`](src/languages) directory with all utility coming from the [`src/prelude`](src/prelude) directory.

When built, the [`build.rs`](build.rs) file will generate a `main.rs` which when run will generate a `lib.rs` file and `src/generated` directory. This is how functionality is added.

These files are the language files used to allow fck to have multilingual support. This README outlines the layout of files and what is in each one.

## Contents
- [File names](#file-names)
- [What goes in the file](#file-contents)
- [Adding a new language](#contributing)
- [Dependencies](#dependencies)

## File names

Each file name is the [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) language code for the language. For example `sv.rs` is the language file for Swedish

## File contents

Each file contains a single constant `LANG`. This is the language definition for that language file. Some files may also contain a few todo comments.

`fckl` equivalents are also supplied in the [fckl equivalents](fckl%20equivalents) directory.

## Contributing

If you would like to add a new official language to fck, make a new rust file with the correct name and fill it in with the language constant. If you already have an `fckl` file you can use the `cargo run -- generate path_to_file.fckl` command to build the associated source file. This is placed in the `src` directory.

Once you've got the source file for the language submit a pull request with your changes, but please note that we will only accept spoken languages into the official repository. We will also not accept language duplicates with modifications based on dialect or other modifications to a language such as alternative comment delimiters. These are specified in language files to allow personal customisation of the language, but should not be used to convert official languages away from the original vision for fck.

### Making a custom language

If you just want a custom language that you've customised to suite you or whatever other reason, you might want to have a look at [this page]() that will walk you through the process of making your own language.

## Dependencies

This crate has one dependency, [`clap`](https://crates.io/crates/clap/3.1.18). We use this to parse command line arguments and do errors (hopefully in several languages I haven't checked)

[`lang-inner`]: lang-inner
[`lang-macros`]: lang-macros

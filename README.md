# fck Language files

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

use lang_inner::{Errors, Warns, CLIKeywords, CLICommands, CLIArgs};
use proc_macro2::TokenStream;
use quote::quote;
use crate::se::Serialize;


impl Serialize for Errors<'_> {
    fn serialize(self) -> TokenStream {
        let e00 = self.e00.serialize();
        let e01 = self.e01.serialize();
        let e02 = self.e02.serialize();
        let e03 = self.e03.serialize();
        let e04 = self.e04.serialize();
        quote!{Errors { e00: #e00, e01: #e01, e02: #e02, e03: #e03, e04: #e04 }}
    }
}


impl Serialize for Warns<'_> {
    fn serialize(self) -> TokenStream {
        let w00 = self.w00.serialize();
        let w01 = self.w01.serialize();
        let w02 = self.w02.serialize();
        let w03 = self.w03.serialize();
        let w04 = self.w04.serialize();
        quote!{Warns { w00: #w00, w01: #w01, w02: #w02, w03: #w03, w04: #w04 }}
    }
}


impl Serialize for CLIKeywords<'_> {
    fn serialize(self) -> TokenStream {
		let desc = self.desc;
        let commands = self.commands.serialize();
        let args = self.args.serialize();
        quote!{ CLIKeywords { desc: #desc, commands: #commands, args: #args } }
    }
}


impl Serialize for CLICommands<'_> {
    fn serialize(self) -> TokenStream {
        macro_rules! ser {
            ($($name:ident),*$(,)?) => { quote!{
                CLICommands { $(#$name),* }
            }};
        }
        let (k_new_0, k_new_1) = self.k_new;
        let k_new = quote!{ k_new: (#k_new_0, #k_new_1) };
		let (k_shell_0, k_shell_1) = self.k_shell;
        let k_shell = quote!{ k_shell: (#k_shell_0, #k_shell_1) };
		let (k_build_0, k_build_1) = self.k_build;
        let k_build = quote!{ k_build: (#k_build_0, #k_build_1) };
		let (k_run_0, k_run_1) = self.k_run;
        let k_run = quote!{ k_run: (#k_run_0, #k_run_1) };
		let (k_test_0, k_test_1) = self.k_test;
        let k_test = quote!{ k_test: (#k_test_0, #k_test_1) };
		let (k_info_0, k_info_1) = self.k_info;
        let k_info = quote!{ k_info: (#k_info_0, #k_info_1) };
		let (k_lint_0, k_lint_1) = self.k_lint;
        let k_lint = quote!{ k_lint: (#k_lint_0, #k_lint_1) };
		let (k_raw_0, k_raw_1) = self.k_raw;
        let k_raw = quote!{ k_raw: (#k_raw_0, #k_raw_1) };
		let (k_doc_0, k_doc_1) = self.k_doc;
        let k_doc = quote!{ k_doc: (#k_doc_0, #k_doc_1) };
		let (k_translate_0, k_translate_1) = self.k_translate;
        let k_translate = quote!{ k_translate: (#k_translate_0, #k_translate_1) };
        ser!(k_new, k_shell, k_build, k_run, k_test, k_info, k_lint, k_raw, k_doc, k_translate)
    }
}


impl Serialize for CLIArgs<'_> {
    fn serialize(self) -> TokenStream {
        macro_rules! ser {
            ($($name:ident),*$(,)?) => { quote!{
                CLIArgs { $(#$name),* }
            }};
        }
        let (k_help_0, k_help_1, k_help_2) = self.k_help;
        let k_help = quote!{ k_help: (#k_help_0, #k_help_1, #k_help_2) };
		let (k_path_0, k_path_1, k_path_2) = self.k_path;
        let k_path = quote!{ k_path: (#k_path_0, #k_path_1, #k_path_2) };
		let (k_git_0, k_git_1, k_git_2) = self.k_git;
        let k_git = quote!{ k_git: (#k_git_0, #k_git_1, #k_git_2) };
		let (k_dump_llvm_0, k_dump_llvm_1, k_dump_llvm_2) = self.k_dump_llvm;
        let k_dump_llvm = quote!{ k_dump_llvm: (#k_dump_llvm_0, #k_dump_llvm_1, #k_dump_llvm_2) };
		let (k_no_build_0, k_no_build_1, k_no_build_2) = self.k_no_build;
        let k_no_build = quote!{ k_no_build: (#k_no_build_0, #k_no_build_1, #k_no_build_2) };
		let (k_test_0, k_test_1, k_test_2) = self.k_test;
        let k_test = quote!{ k_test: (#k_test_0, #k_test_1, #k_test_2) };
		let (k_raw_0, k_raw_1, k_raw_2) = self.k_raw;
        let k_raw = quote!{ k_raw: (#k_raw_0, #k_raw_1, #k_raw_2) };
		let (k_target_0, k_target_1, k_target_2) = self.k_target;
        let k_target = quote!{ k_target: (#k_target_0, #k_target_1, #k_target_2) };
		let (k_output_0, k_output_1, k_output_2) = self.k_output;
        let k_output = quote!{ k_output: (#k_output_0, #k_output_1, #k_output_2) };
		let (k_comment_0, k_comment_1, k_comment_2) = self.k_comment;
        let k_comment = quote!{ k_comment: (#k_comment_0, #k_comment_1, #k_comment_2) };
        ser!(k_help, k_path, k_git, k_dump_llvm, k_no_build, k_test, k_raw, k_target, k_output, k_comment)
    }
}

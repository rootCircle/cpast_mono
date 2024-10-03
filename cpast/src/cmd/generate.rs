use std::process::exit;

use crate::cli::cli_parser::GenerateArgs;
use clex::generator;
#[cfg(feature = "clipboard")]
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use colored::Colorize;
use cpast::DEFAULT_FAIL_EXIT_CODE;

pub(crate) fn generate_call(args: GenerateArgs) {
    match args.generator {
        Some(language) => {
            match generator(language) {
                Ok(testcase) => {
                    let generated_testcases = testcase;
                    println!("=====================================");
                    println!("{}", &generated_testcases);
                    println!("=====================================");
                    if args.clipboard {
                        copy_content_to_clipboard(generated_testcases);
                    }
                }
                Err(err) => {
                    eprintln!("{}", err);
                    exit(DEFAULT_FAIL_EXIT_CODE);
                }
            };
        }
        None => {
            println!("{}", "[GENERATOR] Generator language is required!".red());
        }
    };
}

#[allow(unused_variables)]
fn copy_content_to_clipboard(generated_testcases: String) {
    #[cfg(all(
        any(target_os = "windows", target_os = "linux", target_os = "macos"),
        feature = "clipboard"
    ))]
    {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(generated_testcases).unwrap();

        // get_contents is required for set_contents to work
        // Refer https://github.com/aweinstock314/rust-clipboard/issues/86
        let _ = ctx.get_contents();
        println!("{}", "Copied to clipboard successfully!".green());
    }

    #[cfg(any(
        not(any(target_os = "windows", target_os = "linux", target_os = "macos")),
        not(feature = "clipboard")
    ))]
    println!(
        "{}",
        "Clipboard Features not enabled during compilation/device not supported!".yellow()
    );
}

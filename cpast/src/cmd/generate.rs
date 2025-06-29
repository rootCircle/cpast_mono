use std::process::exit;

use crate::cli::cli_parser::GenerateArgs;
use clex_gen::generator;
#[cfg(any(
    all(unix, not(any(target_os = "android", target_os = "emscripten"))),
    windows,
))]
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use colored::Colorize;
use cpast::DEFAULT_FAIL_EXIT_CODE;

pub(crate) fn generate_call(args: GenerateArgs) {
    match args.generator {
        Some(language) => {
            match generator(language) {
                Ok(testcase) => {
                    let generated_testcases = testcase;
                    eprintln!("=====================================");
                    println!("{}", &generated_testcases);
                    eprintln!("=====================================");
                    if args.clipboard {
                        copy_content_to_clipboard(generated_testcases);
                    }
                }
                Err(err) => {
                    eprintln!("{err}");
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
    #[cfg(any(
        all(unix, not(any(target_os = "android", target_os = "emscripten"))),
        windows,
    ))]
    {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(generated_testcases).unwrap();

        // get_contents is required for set_contents to work
        // Refer https://github.com/aweinstock314/rust-clipboard/issues/86
        let _ = ctx.get_contents();
        eprintln!("{}", "Copied to clipboard successfully!".green());
    }

    #[cfg(not(any(
        all(unix, not(any(target_os = "android", target_os = "emscripten"))),
        windows,
    )))]
    eprintln!(
        "{}",
        "Clipboard Features not enabled during compilation/device not supported!".yellow()
    );
}

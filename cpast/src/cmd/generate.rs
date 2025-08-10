use std::process::exit;

use crate::cli::cli_parser::GenerateArgs;
#[cfg(any(
    all(unix, not(any(target_os = "android", target_os = "emscripten"))),
    windows,
))]
use arboard::Clipboard;
use clex_gen::generator;
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
        let mut ctx = Clipboard::new().unwrap();
        ctx.set_text(generated_testcases).unwrap();

        let _ = ctx.get_text();
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

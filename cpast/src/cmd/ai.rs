#[cfg(any(
    all(unix, not(any(target_os = "android", target_os = "emscripten"))),
    windows,
))]
use cli_clipboard::{ClipboardContext, ClipboardProvider};

use colored::Colorize;
use cscrapper::qscrapper::ScraperError;

use crate::{cli::cli_parser::AiArgs, error_types::cli_error::CliErrorType};

pub(crate) async fn generate_clex_from_input_format_and_constraints(
    args: AiArgs,
) -> Result<(), Box<CliErrorType>> {
    if !((args.problem_url.is_some() && args.input_format.is_none() && args.constraints.is_none())
        || (args.problem_url.is_none()
            && args.input_format.is_some()
            && args.constraints.is_some()))
    {
        return Err(Box::new(CliErrorType::AiRequiredArgsMissing));
    }

    let api_key = std::env::var("GOOGLE_API_KEY").ok();

    match api_key {
        Some(api_key) => {
            let generator = clex_llm::create_clex_generator(&api_key)
                .map_err(|err| Box::new(CliErrorType::ClexLLMInitilizationError(err)))?;

            let (input_format, constraints) = match args.problem_url {
                Some(problem_url) => {
                    let parsed_url = cscrapper::parse_problem_url(&problem_url).ok_or(Box::new(
                        CliErrorType::CScrapperError(ScraperError::ProblemNotFound),
                    ))?;
                    let response = cscrapper::get_problem_statement(parsed_url)
                        .await
                        .map_err(|err| Box::new(CliErrorType::CScrapperError(err)))?;

                    (response.input_format, response.constraints)
                }
                None => {
                    let input_format = match args.input_format {
                        Some(input_format) => input_format,
                        None => {
                            return Err(Box::new(CliErrorType::InputFormatMissing));
                        }
                    };

                    let constraints = match args.constraints {
                        Some(constraints) => constraints,
                        None => {
                            return Err(Box::new(CliErrorType::ConstraintsMissing));
                        }
                    };

                    (input_format, constraints)
                }
            };

            let response =
                clex_llm::generate_clex_expression(&generator, &input_format, &constraints)
                    .await
                    .map_err(|err| Box::new(CliErrorType::ClexLLMGenerationError(Box::new(err))))?;

            println!("Clex Expression Generated:\n{}", response.green());
            println!("You can test using the following commands:\n");

            println!("1. {}:", "Generate test cases".bright_cyan());
            let cpast_generate = format!("cpast generate \"{response}\"");
            println!("   {}", cpast_generate.bright_yellow());

            println!("\n2. {}:", "Test two codes based on this".bright_cyan());
            let cpast_test =
                format!("cpast test -g \"{response}\" -c \"<correct code>\" -t \"<test code>\"");
            println!("   {}", cpast_test.bright_yellow());

            if args.clipboard {
                copy_content_to_clipboard(response);
            }
        }
        None => {
            return Err(Box::new(CliErrorType::GeminiAPIKeyMissing));
        }
    }

    Ok(())
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

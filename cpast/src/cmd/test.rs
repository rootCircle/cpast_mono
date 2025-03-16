use std::process::exit;

use crate::{cli::cli_parser::TestArgs, error_types::cli_error::CliErrorType};
use ccode_runner::lang_runner::language_name::LanguageName;
use colored::Colorize;
use cpast::{CodeOrPath, DEFAULT_FAIL_EXIT_CODE, compile_and_test};
use cscrapper::qscrapper::ScraperError;

pub(crate) async fn test_call(args: TestArgs) {
    let test_binding = args.test_file.unwrap_or_default();
    let iterations = args.iterations;
    let no_stop = args.no_stop;
    let do_force_compile = args.force_recompile;
    let debug = args.debug;

    if !((args.problem_url.is_some() && args.correct_file.is_none() && args.generator.is_none())
        || (args.problem_url.is_none() && args.correct_file.is_some() && args.generator.is_some()))
    {
        eprintln!(
            "{}",
            "[TEST] Either problem URL or test file & generator is required!".red()
        );
        exit(DEFAULT_FAIL_EXIT_CODE);
    }

    let (correct_binding, clex) = match args.problem_url {
        Some(problem_url) => {
            eprintln!(
                "{}",
                "Using --problem_url is unstable at this moment. It may or may not work!".magenta()
            );

            let (generated_clex, generated_code, generated_language) =
                match get_clex_code_input_format_constraints_from_problem_url(&problem_url).await {
                    Ok(response) => response,
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(DEFAULT_FAIL_EXIT_CODE);
                    }
                };

            println!("Clex Expression Generated:\n{}", generated_clex.green());

            if debug {
                // Print code and clex in formatted fashion
                println!(
                    "\nCorrect code is generated using Gemini in {}",
                    generated_language
                );
                println!("{}", generated_code);
            }

            (
                CodeOrPath::Code(generated_code, generated_language),
                generated_clex,
            )
        }
        None => {
            let language = args.generator.unwrap_or_default();

            let correct_binding = args.correct_file.unwrap_or_default();
            (CodeOrPath::Path(correct_binding), language)
        }
    };

    compile_and_test(
        correct_binding,
        test_binding,
        clex,
        iterations,
        no_stop,
        do_force_compile,
        debug,
    )
    .await
    .unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(DEFAULT_FAIL_EXIT_CODE);
    });
}

/// Generate Clex, Code from problem URL
async fn get_clex_code_input_format_constraints_from_problem_url(
    problem_url: &str,
) -> Result<(String, String, LanguageName), Box<CliErrorType>> {
    let parsed_url = cscrapper::parse_problem_url(problem_url).ok_or(Box::new(
        CliErrorType::CScrapperError(ScraperError::ProblemNotFound),
    ))?;
    let response = cscrapper::get_problem_statement(parsed_url)
        .await
        .map_err(|err| Box::new(CliErrorType::CScrapperError(err)))?;

    let api_key = std::env::var("GOOGLE_API_KEY").ok();
    if api_key.is_none() {
        return Err(Box::new(CliErrorType::GeminiAPIKeyMissing));
    }

    let (generated_code, generated_language, generated_clex) = match api_key {
        None => return Err(Box::new(CliErrorType::GeminiAPIKeyMissing)),
        Some(api_key) => {
            let code_llm_gen = clex_llm::create_code_generator(&api_key)
                .map_err(CliErrorType::ClexLLMInitilizationError)?;
            let (code_response, code_language) = clex_llm::generate_code_solution(
                &code_llm_gen,
                &response.statement,
                &response.input_format,
                &response.constraints,
            )
            .await
            .map_err(|err| CliErrorType::ClexLLMGenerationError(Box::new(err)))?;

            let generator = clex_llm::create_clex_generator(&api_key)
                .map_err(CliErrorType::ClexLLMInitilizationError)?;
            let generated_clex = clex_llm::generate_clex_expression(
                &generator,
                &response.input_format,
                &response.constraints,
            )
            .await
            .map_err(|err| Box::new(CliErrorType::ClexLLMGenerationError(Box::new(err))))?;

            (code_response, code_language, generated_clex)
        }
    };

    Ok((generated_clex, generated_code, generated_language))
}

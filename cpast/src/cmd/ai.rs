use colored::Colorize;

use crate::{cli::cli_parser::AiArgs, error_types::cli_error::CliErrorType};

pub(crate) async fn generate_clex_from_input_format_and_constraints(
    args: AiArgs,
) -> Result<(), Box<CliErrorType>> {
    let api_key = std::env::var("GEMINI_API_KEY").ok();

    match api_key {
        Some(api_key) => {
            let generator = clex_llm::create_generator(&api_key)
                .map_err(|err| Box::new(CliErrorType::ClexLLMInitilizationError(err)))?;

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

            let response =
                clex_llm::generate_clex_expression(&generator, &input_format, &constraints)
                    .await
                    .map_err(|err| Box::new(CliErrorType::ClexLLMGenerationError(Box::new(err))))?;
            println!("clex: {}", response.green());
        }
        None => {
            return Err(Box::new(CliErrorType::GeminiAPIKeyMissing));
        }
    }

    Ok(())
}

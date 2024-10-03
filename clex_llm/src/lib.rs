use generator::ClexPromptGenerator;
use google_generative_ai_rs::v1::errors::GoogleAPIError;

mod examples;
mod generator;

pub fn create_generator(api_key: &str) -> Result<ClexPromptGenerator, Box<dyn std::error::Error>> {
    generator::ClexPromptGenerator::new(api_key)
}

pub async fn generate_clex_expression(
    generator: &ClexPromptGenerator,
    input_format: &str,
    constraints: &str,
) -> Result<String, GoogleAPIError> {
    generator.generate_response(input_format, constraints).await
}

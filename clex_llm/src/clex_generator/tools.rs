use std::convert::Infallible;

use clex_gen as clex;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Tool arguments for validating a Clex expression.
#[derive(Debug, Deserialize)]
pub struct ValidateClexArgs {
    /// The Clex expression to validate.
    pub expression: String,
}

/// Tool output for Clex validation.
#[derive(Debug, Serialize)]
pub struct ValidateClexResult {
    /// Whether the expression is valid according to the Clex grammar.
    pub valid: bool,
    /// Error message if invalid.
    pub error: Option<String>,
}

/// A rig tool that validates Clex expressions by parsing them with `clex_gen`.
#[derive(Default)]
pub struct ValidateClex;

impl Tool for ValidateClex {
    const NAME: &'static str = "validate_clex";

    type Error = Infallible;
    type Args = ValidateClexArgs;
    type Output = ValidateClexResult;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        // JSON schema describing the tool's parameters
        serde_json::from_value(json!({
			"name": Self::NAME,
			"description": "Validate a Clex expression against the Clex grammar and return whether it is valid with an optional error message.",
			"parameters": {
				"type": "object",
				"properties": {
					"expression": {
						"type": "string",
						"description": "The Clex expression to validate."
					}
				},
				"required": ["expression"]
			}
		}))
		.expect("valid tool definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Try to parse the expression. If it parses, it's valid; otherwise return the error.
        let result = match clex::generator(args.expression) {
            Ok(_) => ValidateClexResult {
                valid: true,
                error: None,
            },
            Err(e) => ValidateClexResult {
                valid: false,
                error: Some(format!("{e:?}")),
            },
        };

        Ok(result)
    }
}

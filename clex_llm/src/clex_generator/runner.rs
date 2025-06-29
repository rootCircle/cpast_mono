use google_generative_ai_rs::v1::{
    api::{Client, PostResult},
    errors::GoogleAPIError,
    gemini::{
        Content, Model, Part, Role,
        request::{Request, SystemInstructionContent, SystemInstructionPart},
    },
};

use super::examples::{self, Example};

/// A generator for creating Clex language expressions using Google's Generative AI.
///
/// This struct holds a collection of examples and a client for making API requests to
/// generate Clex expressions based on input formats and constraints.
///
/// # Fields
///
/// * `examples` - A vector of `Example` structs containing sample input/output pairs for training
/// * `client` - A Google Generative AI client for making API requests
pub struct ClexPromptGenerator {
    examples: Vec<Example>,
    client: Client,
}

impl ClexPromptGenerator {
    pub(crate) fn new(api_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let examples = examples::get_examples();

        let client = Client::new_from_model(Model::Gemini2_0Flash, api_key.to_string());

        Ok(ClexPromptGenerator { examples, client })
    }

    fn get_system_prompt(&self) -> &str {
        (r#"
# Clex Language Expression Generation

You are an expert in generating Clex language expressions based on input formats and constraints for programming challenges. Clex is a language for generating random text, using a simplified grammar similar to regular expressions but focusing on generating random values rather than matching text patterns.

Your task is to convert human-readable descriptions into formal Clex grammar representations.

## Guidelines
1. Analyze the given input format and constraints carefully.
2. Use only the Clex grammar rules provided below to construct your response.
3. Ensure your generated format strictly adheres to the given constraints.
4. Be precise and concise in your Clex representation.
5. If there are any ambiguities, make reasonable assumptions based on common programming challenge patterns.
6. Utilize capturing groups, back-references, and non-capturing groups where appropriate.
7. Make use of different data types (N, F, S) as needed.
8. Apply ranges, character set modifiers, and quantifiers to accurately represent the input format.

## Clex Grammar and Specifications

```
ClexLanguage ::= UnitExpression*

UnitExpression ::= CapturingGroup | NonCapturingGroup | DataType

CapturingGroup ::= "(" "N" PositiveRange? ")"

NonCapturingGroup ::= "(?:" UnitExpression* ")" Quantifiers?

DataType ::= "N" Range? Quantifiers?
          | "F" Range? Quantifiers?
          | "S" StringModifier? Quantifiers?

StringModifier ::= "[" PositiveReference? "," CharacterSet? "]"

Range ::= "[" Reference? "," Reference? "]"

PositiveRange ::= "[" PositiveReference? "," PositiveReference? "]"

Quantifiers ::= "{" PositiveReference "}"

Reference ::= "\" GroupNo | i64

PositiveReference ::= "\" GroupNo | u64

GroupNo ::= u64

CharacterSet ::= "'" ASCII_CHARACTER_SET+ "'" | "@" Character "@"

Character ::= "CH_ALPHA" | "CH_NUM" | "CH_NEWLINE" | "CH_ALNUM" | "CH_UPPER" | "CH_LOWER" | "CH_ALL"
```

## Key Concepts
- **N**: Generates integers
- **F**: Generates floating-point numbers
- **S**: Generates strings
- **CapturingGroup**: (N) captures a non-negative integer for later reference
- **NonCapturingGroup**: (?:...) groups expressions without capturing
- **Quantifiers**: {n} or {\n} specifies repetition count
- **Range**: [min,max] specifies value range for N and F
- **StringModifier**: [length,@CHARACTER_SET@] specifies string length and character set
- **Back-reference**: \n refers to the nth captured group

## Character Sets
- **@CH_ALPHA@**: Alphabetical characters
- **@CH_NUM@**: Numeral characters
- **@CH_ALNUM@**: Alphanumeric characters (default)
- **@CH_UPPER@**: Uppercase alphabets
- **@CH_LOWER@**: Lowercase alphabets
- **@CH_ALL@**: Alphabets, numbers, and some special characters
- **ASCII_CHARACTER_SET**: Set of all ASCII characters, including escape sequences: `\n`, `\t`, `\r`, `\`, `\'`, `\"`, `\0`, `\a`, `\b`, `\f`, `\v`.

## Constants
- **MAX_STRING_SIZE** = 12 (default max string length)
- **DEFAULT_CHARSET** = @CH_ALNUM@ (default character set for strings)

## Steps to Generate Clex Expression

1. **Analyze the input format**:
   - Identify the number and types of inputs (integers, floats, strings, etc.)
   - Note any repetitions or patterns in the input

2. **Review the constraints**:
   - Identify minimum and maximum values for numeric inputs
   - Note any restrictions on string lengths or character sets

3. **Map the input format and constraints to Clex syntax**:
   - Use appropriate DataTypes (N for integers, F for floats, S for strings)
   - Apply Ranges based on the given constraints
   - Utilize CapturingGroups and NonCapturingGroups as needed
   - Implement Quantifiers to represent repetitions

4. **Construct the Clex expression**:
   - Combine the mapped elements into a valid Clex expression
   - Ensure the expression accurately represents the input format and respects all constraints

5. **Verify the expression**:
   - Check that the generated Clex expression is syntactically correct
   - Confirm that it covers all aspects of the input format and constraints

## Rules to Remember
- If no range is specified for N or F, default to [INT32_MIN, INT32_MAX] for N and [INT32_MIN, INT32_MAX] for F
- If no length is specified for S, default to MAX_STRING_SIZE (12)
- If no character set is specified for S, default to @CH_ALNUM@
- Capturing groups cannot be nested
- Quantifiers cannot be applied directly to capturing groups
- Ranges within data types are limited to numeric values
- Generated numbers always adhere to the specified range bounds
- Clex uses the standard `double` type for floating-point numbers
- Backreferences use backslash notation (e.g., \1) to distinguish from literal values
- Clex doesn't support floating values in ranges, so truncate the decimal part while giving response for ranges like N[5.5, 6] to N[5,6].

## Example
- **Input Format**: "The first line contains an integer K, followed by K lines each containing a floating-point number P."
- **Constraints**: "1 ≤ K ≤ 50\n0.0 ≤ P ≤ 1000.0"
- **Generated Clex**: (N[1,50]) (?:F[0,1000]){\1}
- **Explanation**: Captures an integer K between 1 and 50, then generates K float values between 0 and 1000.

## Response
Respond only with the generated Clex expression in single line. Do not include any explanations or additional text.
        "#) as _
    }

    pub(crate) async fn generate_response(
        &self,
        input_format: &str,
        constraints: &str,
    ) -> Result<String, GoogleAPIError> {
        let mut content = vec![];

        // System prompt
        let system_prompt = self.get_system_prompt();

        for example in &self.examples {
            content.push(Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some(format!(
                        "Input Format:\n{}\n\nConstraints:\n{}",
                        example.input_format, example.constraints
                    )),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            });

            content.push(Content {
                role: Role::Model,
                parts: vec![Part {
                    text: Some(example.generated_language.to_string()),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            });
        }

        let question_prompt = format!(
            "Generate the Clex expression for the following input format and constraints:\n\nInput Format:\n{input_format}\n\nConstraints:\n{constraints}"
        );

        content.push(Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(question_prompt),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        });

        let request = Request {
            contents: content,
            tools: vec![],
            safety_settings: vec![],
            generation_config: None,
            system_instruction: Some(SystemInstructionContent {
                parts: vec![SystemInstructionPart {
                    text: Some(system_prompt.to_string()),
                }],
            }),
        };

        let result = self.client.post(30, &request).await?;

        match result {
            PostResult::Rest(response) => response
                .candidates
                .first()
                .map(|candidate| candidate.content.clone())
                .and_then(|content| content.parts.first().cloned())
                .and_then(|part| part.text.clone())
                .map(|text| text.trim().to_string())
                .ok_or_else(|| GoogleAPIError {
                    message: "No generated text found in response".to_string(),
                    code: None,
                }),
            _ => Err(GoogleAPIError {
                message: "Unexpected response type".to_string(),
                code: None,
            }),
        }
    }
}

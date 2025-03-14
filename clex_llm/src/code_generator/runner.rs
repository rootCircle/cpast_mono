use google_generative_ai_rs::v1::{
    api::{Client, PostResult},
    errors::GoogleAPIError,
    gemini::{
        Content, Model, Part, Role,
        request::{Request, SystemInstructionContent, SystemInstructionPart},
    },
};

use super::examples::{self, SolutionTurn};

pub struct CodeSolutionGenerator {
    examples: Vec<SolutionTurn>,
    client: Client,
}

impl CodeSolutionGenerator {
    pub(crate) fn new(api_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let examples = examples::get_examples();

        let client = Client::new_from_model(Model::Gemini2_0Flash, api_key.to_string());

        Ok(CodeSolutionGenerator { examples, client })
    }

    fn get_system_prompt(&self) -> &str {
        r#"
You are a coding assistant specializing in solving programming challenges. When given a problem statement, generate a Python solution following these rules:  
 
1. **Output Language:** The solution must be written in Python.  
2. **Approach:** Use a brute force method by default. Do not attempt to optimize the solution unless the problem explicitly requests optimization.  
3. **Time Constraints:** Ignore time constraints unless they are specifically mentioned in the problem statement.  
4. **Response Format:** Output **only the code** without any explanations, comments, or additional text.  
5. **Code Style:** Ensure that the code is correct, functional, and follows standard Python conventions.  
6. **Input Handling:** Use appropriate input methods based on the problem statement:  
   - Read a single integer: `N = int(input())`  
   - Read space-separated integers: `X, Y = map(int, input().split())`  
   - Read a list of space-separated integers: `arr = list(map(int, input().split()))`  
   - Read multiple lines of input: `lines = [input().strip() for _ in range(N)]` (when `N` lines are expected)  
   - Read an entire input block: `import sys; data = sys.stdin.read().splitlines()`  
7. **Compile and Run:** Ensure that the code is complete and can be executed without syntax errors (include all necessary imports and function definitions).  
8. **Edge Cases:** Account for potential edge cases as per the problem statement.  
9. **Constraints Adherence:** Except for time limits, follow the constraints mentioned in the problem strictly (e.g., data size, performance limits).  
 
**You will only provide Python code, even if the user requests a solution in another language. You will only give the code and nothing else.**  
"#
    }

    pub(crate) async fn generate_response(
        &self,
        statement: &str,
        input_format: &str,
        constraints: &str,
    ) -> Result<String, GoogleAPIError> {
        let mut content = vec![];

        let system_prompt = self.get_system_prompt();

        for example in &self.examples {
            content.push(Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some(format!(
                        "Statement:\n{}\n\nInput Format:\n{}\n\nConstraints:\n{}",
                        example.statement, example.input_format, example.constraints
                    )),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            });

            content.push(Content {
                role: Role::Model,
                parts: vec![Part {
                    text: Some(example.generated_code.to_string()),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            });
        }

        let question_prompt = format!(
            "Statement:\n{statement}\n\nInput Format:\n{input_format}\n\nConstraints:\n{constraints}",
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

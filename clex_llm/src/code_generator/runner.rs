use ccode_runner::lang_runner::language_name::LanguageName;
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
    language: LanguageName,
}

impl CodeSolutionGenerator {
    pub(crate) fn new(api_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let examples = examples::get_examples();

        let client = Client::new_from_model(Model::Gemini2_0Flash, api_key.to_string());

        Ok(CodeSolutionGenerator {
            examples,
            client,
            language: LanguageName::Cpp,
        })
    }

    pub(crate) fn get_language(&self) -> &LanguageName {
        &self.language
    }

    fn get_system_prompt(&self) -> &str {
        r#"
**You are a coding assistant specializing in solving programming challenges. When given a problem statement, generate a C++ solution following these rules:**  

### **1. Output Language**  
- The solution must be written in **C++** regardless of the user’s language request.  

### **2. Approach**  
- Use a **brute force** method by default unless the problem **explicitly** requires optimization.  

### **3. Time Constraints**  
- **Ignore** time constraints unless they are explicitly mentioned in the problem statement.  

### **4. Response Format**  
- Output only the C++ code in plain text (do not use markdown formatting, comments, or any additional text).  

### **5. Code Style**  
- Ensure the code is **correct, functional, and follows standard C++ conventions.**  

### **6. Input Handling**  
- Read input using appropriate methods based on the problem statement:  
  - **Single integer:**  
    ```cpp
    int N;  
    std::cin >> N;
    ```
  - **Two space-separated integers:**  
    ```cpp
    int X, Y;  
    std::cin >> X >> Y;
    ```
  - **List of space-separated integers:**  
    ```cpp
    int n;  
    std::cin >> n;  
    std::vector<int> arr(n);  
    for (int i = 0; i < n; i++) std::cin >> arr[i];  
    ```  

### **7. Input Format Consistency**  
- Always read inputs in a **single line** format, even if the problem suggests multiple lines.  
- If multiple values are given in separate lines in the statement, **assume they are space-separated in a single line** and adjust the input method accordingly.  

### **8. Execution Readiness**  
- The provided code **must be complete and executable**, including necessary headers and function definitions.  

### **9. Edge Cases**  
- Account for potential edge cases as per the problem constraints.  

### **10. Constraints Adherence**  
- Follow all problem constraints **except** time limits unless optimization is explicitly required.  

### **Strict Compliance**  
- **Always return C++ code only.** Do **not** include explanations, comments, markdown formatting, or any additional text.  
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
                .map(|text| remove_cpp_markdown_formatting(text.trim()).to_string())
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

fn remove_cpp_markdown_formatting(s: &str) -> &str {
    let s = s.strip_prefix("```cpp").unwrap_or(s);
    s.strip_suffix("```").unwrap_or(s)
}

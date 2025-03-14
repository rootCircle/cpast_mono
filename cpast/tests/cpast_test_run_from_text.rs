use ccode_runner::lang_runner::{language_name::LanguageName, program_store::ProgramStore};
use clex_gen::clex_language::{self, code_generator::Generator, lexer};

#[allow(unused)]
struct EvaluateCodeInputDiff {
    input: String,
    expected_output: String,
    actual_output: String,
}

struct EvaluateCodeResponse {
    has_output_matched: bool,
    input_diffs: Vec<EvaluateCodeInputDiff>,
}

fn run_and_compare(
    correct_code: &str,
    test_code: &str,
    correct_code_language: LanguageName,
    test_code_language: LanguageName,
    clex_language: &str,
) -> EvaluateCodeResponse {
    let runner = ProgramStore::new_from_text(
        correct_code,
        test_code,
        correct_code_language.clone(),
        test_code_language.clone(),
        false,
    )
    .unwrap();

    let mut token = lexer::Tokens::new(clex_language.to_string());
    token.scan_tokens().unwrap();
    let mut parser = clex_language::parser::Parser::new_from_tokens(token);
    parser.parser().unwrap();
    let generator = Generator::new(&parser);

    let mut response = EvaluateCodeResponse {
        has_output_matched: true,
        input_diffs: Vec::new(),
    };

    for _ in 0..10 {
        let testcase = generator.generate_testcases().unwrap();
        let (matched, expected, actual) = runner.run_codes_and_compare_output(&testcase).unwrap();
        if !matched {
            response.has_output_matched = false;
            response.input_diffs.push(EvaluateCodeInputDiff {
                input: testcase,
                expected_output: expected,
                actual_output: actual,
            });
        }
    }

    response
}

#[test]
fn test_custom_text_works_python() {
    let correct_code = "print('Hello, world!')";
    let test_code = "print('Hello, worldd!')";
    let response = run_and_compare(
        correct_code,
        test_code,
        LanguageName::Python,
        LanguageName::Python,
        "",
    );
    assert!(!response.has_output_matched);
    assert_eq!(response.input_diffs.len(), 10);
}

#[test]
fn test_custom_text_works_java() {
    let correct_code = "public class Main { public static void main(String[] args) { System.out.println(\"Hello, world!\"); } }";
    let test_code = "public class \nMain { public static void main(String[] args) { System.out.println(\"Hello, worldd!\"); } }";
    let response = run_and_compare(
        correct_code,
        test_code,
        LanguageName::Java,
        LanguageName::Java,
        "",
    );
    assert!(!response.has_output_matched);
    assert_eq!(response.input_diffs.len(), 10);
}

#[test]
fn test_custom_text_works_cpp() {
    let correct_code = "#include <iostream>\nint main() { std::cout << \"Hello, world!\" << std::endl; return 0; }";
    let test_code = "#include <iostream>\nint main() { std::cout << \"Hello, worldd!\" << std::endl; return 0; }";
    let response = run_and_compare(
        correct_code,
        test_code,
        LanguageName::Cpp,
        LanguageName::Cpp,
        "",
    );
    assert!(!response.has_output_matched);
    assert_eq!(response.input_diffs.len(), 10);
}

#[test]
fn test_custom_text_works_c() {
    let correct_code = "#include <stdio.h>\nint main() { printf(\"Hello, world!\\n\"); return 0; }";
    let test_code = "#include <stdio.h>\nint main() { printf(\"Hello, worldd!\\n\"); return 0; }";
    let response = run_and_compare(
        correct_code,
        test_code,
        LanguageName::C,
        LanguageName::C,
        "",
    );
    assert!(!response.has_output_matched);
    assert_eq!(response.input_diffs.len(), 10);
}

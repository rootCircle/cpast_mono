use ccode_runner::lang_runner::{language_name::LanguageName, program_store::ProgramStore};

#[test]
fn test_run_c_program() {
    let program_text = r#"
        #include <stdio.h>
        int main() {
            int n = 0;
            scanf("%d", &n);
            printf("%d", n);
            return 0;
        }
    "#;
    run_test(program_text, program_text, LanguageName::C);
}

#[test]
fn test_run_cpp_program() {
    let program_text = r#"
        #include <iostream>
        int main() {
            int n;
            std::cin >> n;
            std::cout << n;
            return 0;
        }
    "#;
    run_test(program_text, program_text, LanguageName::Cpp);
}

#[test]
fn test_run_python_program() {
    let program_text = r#"
n = int(input())
print(n, end='')
    "#;
    run_test(program_text, program_text, LanguageName::Python);
}

#[test]
fn test_run_rust_program() {
    let program_text = r#"
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    print!("{}", input.trim());
}
    "#;
    run_test(program_text, program_text, LanguageName::Rust);
}

#[test]
fn test_run_ruby_program() {
    let program_text = r#"
n = gets.to_i
print n
    "#;
    run_test(program_text, program_text, LanguageName::Ruby);
}

#[test]
fn test_run_javascript_program() {
    let program_text = r#"
const readline = require('node:readline').createInterface({
    input: process.stdin,
    output: process.stdout
});

readline.question('', number => {
    console.log(number);
    readline.close();
});

    "#;

    let program = ProgramStore::new_from_text(
        program_text,
        program_text,
        LanguageName::Javascript,
        LanguageName::Javascript,
        false,
    )
    .unwrap();
    // Soundness bug: Readline in JS requires EOL to read from stdin
    let (matched, expected, actual) = program
        .run_codes_and_compare_output("10\n")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected, "10\n");
    assert_eq!(actual, "10\n");
}

#[test]
fn test_run_java_program() {
    let program_text = r#"
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int n = scanner.nextInt();
        System.out.print(n);
        scanner.close();
    }
}
    "#;
    run_test(program_text, program_text, LanguageName::Java);
}

fn run_test(correct_program: &str, test_program: &str, lang: LanguageName) {
    let program =
        ProgramStore::new_from_text(correct_program, test_program, lang.clone(), lang, false)
            .unwrap();

    let (matched, expected, actual) = program
        .run_codes_and_compare_output("10")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected, "10");
    assert_eq!(actual, "10");
}

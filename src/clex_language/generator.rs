use crate::clex_language::ast::{CharacterSet, DataType, Program, ReferenceType, UnitExpression};
use crate::clex_language::parser::Parser;
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};

use std::collections::HashMap;
use std::process::exit;

#[derive(Debug)]
pub(crate) struct Generator {
    syntax_tree: Program,
    pub output_text: String,
    groups: HashMap<u64, i64>, // group_no, repeat_count
}

impl Generator {
    pub fn new(syntax_tree: Parser) -> Self {
        Self {
            syntax_tree: syntax_tree.language,
            output_text: "".to_string(),
            groups: HashMap::new(),
        }
    }

    pub fn reset_output(&mut self) {
        self.output_text = "".to_string();
    }

    fn new_from_program(program: Program, groups: &HashMap<u64, i64>) -> Self {
        Self {
            syntax_tree: program,
            output_text: "".to_string(),
            groups: groups.clone(),
        }
    }

    pub fn traverse_ast(&mut self) {
        for unit_expression in self.syntax_tree.expression.iter() {
            match unit_expression {
                UnitExpression::Primitives {
                    data_type,
                    repetition,
                } => {
                    let repetition_count = if repetition != &ReferenceType::None {
                        self.get_value_from_reference(*repetition)
                    } else {
                        // Defaults repetition to 1
                        1
                    };

                    if repetition_count <= 0 {
                        eprintln!("[GENERATOR ERROR] Repetition Count({repetition_count}) can't be negative or zero");
                        eprintln!("[GENERATOR ERROR] If referencing count by group, then prefer setting minimum value to at-least 1.");
                        exit(1);
                    }

                    for _ in 0..repetition_count {
                        match data_type {
                            DataType::String(length, charset) => self
                                .output_text
                                .push_str(&self.generate_random_string(*length, *charset)),
                            DataType::Character => self
                                .output_text
                                .push_str(&Generator::generate_random_character()),
                            DataType::Float(min_reference, max_reference) => {
                                self.output_text.push_str(
                                    &self
                                        .generate_random_float(*min_reference, *max_reference)
                                        .to_string(),
                                )
                            }
                            DataType::Integer(min_reference, max_reference) => {
                                self.output_text.push_str(
                                    &self
                                        .generate_random_number(*min_reference, *max_reference)
                                        .to_string(),
                                )
                            }
                        }
                        self.output_text.push(' ');
                    }
                }
                UnitExpression::CapturingGroup {
                    group_number,
                    data_type: DataType::Integer(min_reference, max_reference),
                } => {
                    let mut min_reference = min_reference;
                    if self.get_value_from_reference(*min_reference) < 0 {
                        min_reference = &ReferenceType::ByLiteral(0);
                    }
                    let random_number = self.generate_random_number(*min_reference, *max_reference);
                    self.groups.insert(*group_number, random_number);

                    let mut random_number = random_number.to_string();
                    random_number.push(' ');

                    self.output_text.push_str(&random_number);
                }
                UnitExpression::NonCapturingGroup {
                    nest_exp,
                    repetition,
                } => {
                    let repetition_count = if repetition != &ReferenceType::None {
                        self.get_value_from_reference(*repetition)
                    } else {
                        // Defaults repetition to 1
                        1
                    };

                    if repetition_count <= 0 {
                        eprintln!("[GENERATOR ERROR] Repetition Count({repetition_count}) can't be negative or zero");
                        exit(1);
                    }

                    for _ in 0..repetition_count {
                        let mut nest_gen = Generator::new_from_program(
                            Program {
                                expression: nest_exp.clone(),
                            },
                            &self.groups,
                        );
                        nest_gen.traverse_ast();
                        self.groups = nest_gen.groups;
                        self.output_text.push_str(&nest_gen.output_text);
                        self.output_text.push(' ');
                    }
                }
                UnitExpression::Eof => {
                    break;
                }
                _ => {}
            }
        }

        self.post_generation_cleanup();
    }

    fn post_generation_cleanup(&mut self) {
        // Trims out extra whitespaces
        self.output_text = self.output_text.replace("  ", " ");
        self.output_text = self.output_text.trim().to_string()
    }

    fn generate_random_number(
        &self,
        min_reference: ReferenceType,
        max_reference: ReferenceType,
    ) -> i64 {
        let min = self.get_value_from_reference(min_reference);
        let max = self.get_value_from_reference(max_reference);

        if min > max {
            eprintln!("[GENERATOR ERROR] Upper bound should be greater than lower bound in [m({min}),n({max})]");
            exit(1);
        }

        rand::thread_rng().gen_range(min..=max)
    }

    fn generate_random_float(
        &self,
        min_reference: ReferenceType,
        max_reference: ReferenceType,
    ) -> f64 {
        let min = self.get_value_from_reference(min_reference) as f64;
        let max = self.get_value_from_reference(max_reference) as f64;

        if min > max {
            eprintln!("[GENERATOR ERROR] Upper bound should be greater than lower bound in [m({min}),n({max})]");
            exit(1);
        }

        rand::thread_rng().gen_range(min..=max)
    }
    fn get_value_from_reference(&self, reference_type: ReferenceType) -> i64 {
        match reference_type {
            ReferenceType::ByGroup { group_number: gn } => self.get_count_from_group(gn),
            ReferenceType::ByLiteral(value) => value,
            ReferenceType::None => {
                eprintln!("[GENERATOR ERROR] Error detecting Reference Type");
                exit(1);
            }
        }
    }

    fn generate_random_character() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), 1)
    }

    fn generate_random_string(&self, length: ReferenceType, character_set: CharacterSet) -> String {
        // CharacterSet::All => Alphanumeric.sample_string(
        //     &mut rand::thread_rng(),
        //     self.get_value_from_reference(length) as usize,

        Generator::generate_random_string_from_charset(
            character_set,
            self.get_value_from_reference(length) as usize,
        )
    }

    fn generate_random_string_from_charset(character_set: CharacterSet, length: usize) -> String {
        let charset = match character_set {
            CharacterSet::Alpha => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz"
            }
            CharacterSet::Numeric => "0123456789",
            CharacterSet::AlphaNumeric => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz\
            0123456789"
            }
            CharacterSet::UppercaseOnly => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            CharacterSet::LowerCaseOnly => "abcdefghijklmnopqrstuvwxyz",
            CharacterSet::All => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz\
            0123456789)(*&^%$#@!~"
            }
        };

        let charset = charset.as_bytes();

        let mut rng = rand::thread_rng();

        let generated_string: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();

        generated_string
    }

    fn get_count_from_group(&self, group_number: u64) -> i64 {
        match self.groups.get(&group_number) {
            Some(t) => *t,
            None => {
                eprintln!(
                    "Can't find specified Group no. {} in the language",
                    group_number
                );
                exit(1);
            }
        }
    }
}

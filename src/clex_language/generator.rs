use crate::clex_language::ast::{DataType, Program, RepetitionType, UnitExpression};
use crate::clex_language::parser::Parser;
use rand::{
    distributions::{Alphanumeric, DistString},
    prelude::*,
};
use std::collections::HashMap;
use std::process::exit;

const MAX_STRING_SIZE: usize = 12;

pub(crate) struct Generator {
    syntax_tree: Program,
    pub output_text: String,
    groups: HashMap<u64, u64>, // group_no, repeat_count
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

    fn new_from_program(program: Program, groups: &HashMap<u64, u64>) -> Self {
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
                    let repetition_count = match repetition {
                        RepetitionType::ByGroup { group_number } => {
                            self.get_count_from_group(*group_number)
                        }
                        RepetitionType::ByCount(count) => *count,
                        RepetitionType::None => 1,
                    };

                    for _ in 0..repetition_count {
                        match data_type {
                            DataType::String => self
                                .output_text
                                .push_str(&Generator::generate_random_string()),
                            DataType::Character => self
                                .output_text
                                .push_str(&Generator::generate_random_character()),
                            DataType::Float(min, max) => self.output_text.push_str(
                                &Generator::generate_random_float(*min, *max).to_string(),
                            ),
                            DataType::Integer(min, max) => self.output_text.push_str(
                                &Generator::generate_random_number(*min, *max).to_string(),
                            ),
                        }
                        self.output_text.push(' ');
                    }
                }
                UnitExpression::CapturingGroup {
                    group_number,
                    data_type: DataType::Integer(min, max),
                } => {
                    if *min <= 0 {
                        eprintln!("[GENERATOR ERROR] Lower Bound can't be negative or zero in Capturing Group");
                        exit(1);
                    }

                    let random_number = Generator::generate_random_number(*min, *max);
                    self.groups.insert(*group_number, random_number as u64);

                    let mut random_number = random_number.to_string();
                    random_number.push(' ');

                    self.output_text.push_str(&random_number);
                }
                UnitExpression::NonCapturingGroup {
                    nest_exp,
                    repetition,
                } => {
                    let repetition_count = match repetition {
                        RepetitionType::ByGroup { group_number } => {
                            self.get_count_from_group(*group_number)
                        }
                        RepetitionType::ByCount(count) => *count,
                        RepetitionType::None => 1,
                    };

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

    fn generate_random_number(min: i64, max: i64) -> i64 {
        rand::thread_rng().gen_range(min..=max)
    }

    fn generate_random_float(min: f64, max: f64) -> f64 {
        rand::thread_rng().gen_range(min..=max)
    }

    fn generate_random_character() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), 1)
    }

    fn generate_random_string() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), MAX_STRING_SIZE)
    }

    fn get_count_from_group(&self, group_number: u64) -> u64 {
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
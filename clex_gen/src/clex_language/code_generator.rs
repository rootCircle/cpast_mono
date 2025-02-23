use crate::clex_language::ast::{
    CharacterSet, ClexLanguageAST, DataType, PositiveReferenceType, ReferenceType, UnitExpression,
};
use crate::clex_language::parser::Parser;
use rand::Rng;

use crate::clex_language::clex_error_type::{ClexErrorType, ParentErrorType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Generator {
    syntax_tree: ClexLanguageAST,
    output_text: String,
    groups: HashMap<u64, u64>, // group_no, repeat_count
}

impl Generator {
    pub fn new(syntax_tree: Parser) -> Self {
        Self {
            syntax_tree: syntax_tree.get_language().clone(),
            output_text: String::new(),
            groups: HashMap::new(),
        }
    }

    fn reset_output(&mut self) {
        self.output_text = String::new();
    }

    fn new_from_program(program: ClexLanguageAST, groups: &HashMap<u64, u64>) -> Self {
        Self {
            syntax_tree: program,
            output_text: String::new(),
            groups: groups.clone(),
        }
    }

    pub fn generate_testcases(&mut self) -> Result<String, ClexErrorType> {
        if !self.output_text.is_empty() {
            return Ok(self.output_text.clone());
        }

        self.traverse_ast()?;

        let output = self.output_text.clone();

        self.reset_output();

        Ok(output)
    }

    fn traverse_ast(&mut self) -> Result<(), ClexErrorType> {
        for unit_expression in &self.syntax_tree.expression {
            match unit_expression {
                UnitExpression::Primitives {
                    data_type,
                    repetition,
                } => {
                    let repetition_count = self.get_positive_value_from_reference(repetition)?;

                    for _ in 1..=repetition_count {
                        match data_type {
                            DataType::String(min_length, max_length, charset) => {
                                self.output_text.push_str(
                                    &self
                                        .generate_random_string(min_length, max_length, charset)?,
                                )
                            }
                            DataType::Float(min_reference, max_reference) => {
                                self.output_text.push_str(
                                    &self
                                        .generate_random_float(min_reference, max_reference)?
                                        .to_string(),
                                );
                            }
                            DataType::Integer(min_reference, max_reference) => {
                                self.output_text.push_str(
                                    &self
                                        .generate_random_number(min_reference, max_reference)?
                                        .to_string(),
                                );
                            }
                        }
                        self.output_text.push(' ');
                    }
                }
                UnitExpression::CapturingGroup {
                    group_number,
                    range: (min_reference, max_reference),
                } => {
                    let random_number =
                        self.generate_positive_random_number(min_reference, max_reference)?;
                    self.groups.insert(*group_number, random_number);

                    let mut random_number = random_number.to_string();
                    random_number.push(' ');

                    self.output_text.push_str(&random_number);
                }
                UnitExpression::NonCapturingGroup {
                    nest_exp,
                    repetition,
                } => {
                    let repetition_count = self.get_positive_value_from_reference(repetition)?;

                    for _ in 1..=repetition_count {
                        let mut nest_gen = Self::new_from_program(
                            ClexLanguageAST {
                                expression: nest_exp.clone(),
                            },
                            &self.groups,
                        );
                        nest_gen.traverse_ast()?;
                        self.groups = nest_gen.groups;
                        self.output_text.push_str(&nest_gen.output_text);
                        self.output_text.push(' ');
                    }
                }
                UnitExpression::Eof => {
                    break;
                }
            }
        }

        self.post_generation_cleanup();

        Ok(())
    }

    fn post_generation_cleanup(&mut self) {
        // Trims out extra whitespaces
        self.output_text = self.output_text.replace("  ", " ");
        self.output_text = self.output_text.trim().to_string();
    }

    // Helper method for generating random integers
    fn generate_random_integer(&self, min: i64, max: i64) -> Result<i64, ClexErrorType> {
        if min > max {
            return Err(ClexErrorType::InvalidRangeValues(
                ParentErrorType::GeneratorError,
                min,
                max,
            ));
        }
        Ok(rand::thread_rng().gen_range(min..=max))
    }

    // Helper method for generating random positive integers
    fn generate_positive_random_integer(&self, min: u64, max: u64) -> Result<u64, ClexErrorType> {
        if min > max {
            return Err(ClexErrorType::InvalidRangeValues(
                ParentErrorType::GeneratorError,
                min as i64,
                max as i64,
            ));
        }
        Ok(rand::thread_rng().gen_range(min..=max))
    }

    fn generate_random_string(
        &self,
        min_length: &PositiveReferenceType,
        max_length: &PositiveReferenceType,
        character_set: &CharacterSet,
    ) -> Result<String, ClexErrorType> {
        let min_length = self.get_positive_value_from_reference(min_length)? as usize;
        let max_length = self.get_positive_value_from_reference(max_length)? as usize;
        let length = self.generate_positive_random_integer(min_length as u64, max_length as u64)?;
        let charset = character_set.get_character_domain();
        Ok(Self::generate_random_string_from_charset(&charset, length))
    }

    fn generate_random_string_from_charset(charset: &str, length: u64) -> String {
        let charset = charset.as_bytes();
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect()
    }

    fn generate_random_number(
        &self,
        min_reference: &ReferenceType,
        max_reference: &ReferenceType,
    ) -> Result<i64, ClexErrorType> {
        let min = self.get_value_from_reference(min_reference)?;
        let max = self.get_value_from_reference(max_reference)?;

        self.generate_random_integer(min, max)
    }

    fn generate_positive_random_number(
        &self,
        min_reference: &PositiveReferenceType,
        max_reference: &PositiveReferenceType,
    ) -> Result<u64, ClexErrorType> {
        let min = self.get_positive_value_from_reference(min_reference)?;
        let max = self.get_positive_value_from_reference(max_reference)?;

        self.generate_positive_random_integer(min, max)
    }

    fn generate_random_float(
        &self,
        min_reference: &ReferenceType,
        max_reference: &ReferenceType,
    ) -> Result<f64, ClexErrorType> {
        let min = self.get_value_from_reference(min_reference)? as f64;
        let max = self.get_value_from_reference(max_reference)? as f64;

        if min > max {
            return Err(ClexErrorType::InvalidRangeValues(
                ParentErrorType::GeneratorError,
                min as i64,
                max as i64,
            ));
        }

        Ok(rand::thread_rng().gen_range(min..=max))
    }

    fn get_value_from_reference(
        &self,
        reference_type: &ReferenceType,
    ) -> Result<i64, ClexErrorType> {
        Ok(match reference_type {
            ReferenceType::ByGroup { group_number: gn } => self.get_count_from_group(*gn)? as i64,
            ReferenceType::ByLiteral(value) => *value,
        })
    }

    fn get_positive_value_from_reference(
        &self,
        reference_type: &PositiveReferenceType,
    ) -> Result<u64, ClexErrorType> {
        Ok(match reference_type {
            PositiveReferenceType::ByGroup { group_number: gn } => {
                self.get_count_from_group(*gn)?
            }
            PositiveReferenceType::ByLiteral(value) => *value,
        })
    }

    fn get_count_from_group(&self, group_number: u64) -> Result<u64, ClexErrorType> {
        match self.groups.get(&group_number) {
            Some(value) => Ok(*value),
            None => Err(ClexErrorType::UnknownGroupNumber(
                ParentErrorType::GeneratorError,
                group_number,
            )),
        }
    }
}

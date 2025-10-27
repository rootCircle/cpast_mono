//! The `code_generator` module is responsible for generating test cases based on the Abstract Syntax Tree (AST)
//! produced by parsing `clex` language patterns. It converts the structured AST into concrete test data
//! following the specified patterns and constraints.
//!
//! # Core Components
//!
//! - `Generator`: The main struct that handles test case generation from an AST
//! - Random value generation for different data types (Integer, Float, String)
//! - Support for capturing and non-capturing groups
//! - Reference resolution for group values
//!
//! # Features
//!
//! - Random number generation within specified ranges
//! - String generation with customizable character sets
//! - Support for nested expressions and repetitions
//! - Group value tracking and reference resolution
//! - Float, Integer and String data type generation
//!
//! # Example
//!
//! ```rust
//! use clex_gen::clex_language::parser::Parser;
//! use clex_gen::clex_language::code_generator::Generator;
//!
//! let source = "N[1,100]";
//! let mut parser = Parser::new(source.to_string()).unwrap();
//! parser.parser().unwrap();
//!
//! let generator = Generator::new(&parser);
//! let test_case = generator.generate_testcases().unwrap();
//! ```
//!
//! The generator maintains state about:
//! - The AST being processed
//! - Currently defined capturing groups
//! - Generated values for references
//!
//! It provides comprehensive error handling for invalid ranges, unknown group references,
//! and other potential generation-time issues.

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
}

/// Iterator that generates test case data incrementally by processing each unit expression.
///
/// This iterator yields chunks of the generated test case as individual tokens/values,
/// allowing for memory-efficient handling of large test cases. Each iteration processes
/// one unit expression from the AST and yields its generated output.
///
/// # Example
///
/// ```rust
/// use clex_gen::clex_language::parser::Parser;
/// use clex_gen::clex_language::code_generator::Generator;
///
/// let source = "N[1,100] N[1,100]";
/// let mut parser = Parser::new(source.to_string()).unwrap();
/// parser.parser().unwrap();
///
/// let generator = Generator::new(&parser);
/// for chunk_result in generator.generate_testcases_iter() {
///     match chunk_result {
///         Ok(data) => print!("{}", data),
///         Err(e) => {
///             eprintln!("Error: {}", e);
///             break;
///         }
///     }
/// }
/// ```
pub struct TestCaseIterator {
    generator: Generator,
    groups: HashMap<u64, u64>,
    expr_index: usize,
    finished: bool,
}

impl TestCaseIterator {
    fn new(generator: Generator) -> Self {
        Self {
            generator,
            groups: HashMap::new(),
            expr_index: 0,
            finished: false,
        }
    }
}

impl Iterator for TestCaseIterator {
    type Item = Result<String, ClexErrorType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if self.expr_index >= self.generator.syntax_tree.expression.len() {
            self.finished = true;
            return None;
        }

        let unit_expression = &self.generator.syntax_tree.expression[self.expr_index].clone();
        self.expr_index += 1;

        match self.process_unit_expression(unit_expression) {
            Ok(output) => {
                // Check if this is EOF and handle trailing space
                if matches!(unit_expression, UnitExpression::Eof) {
                    self.finished = true;
                    // Return empty string for EOF since we already removed trailing space
                    return None;
                }
                Some(Ok(output))
            }
            Err(e) => {
                self.finished = true;
                Some(Err(e))
            }
        }
    }
}

impl TestCaseIterator {
    fn process_unit_expression(
        &mut self,
        unit_expression: &UnitExpression,
    ) -> Result<String, ClexErrorType> {
        let mut output_text = String::new();

        match unit_expression {
            UnitExpression::Primitives {
                data_type,
                repetition,
            } => {
                let repetition_count = self
                    .generator
                    .get_positive_value_from_reference(repetition, &self.groups)?;

                for _ in 1..=repetition_count {
                    let generated_text = match data_type {
                        DataType::String(min_length, max_length, charset) => {
                            self.generator.generate_random_string(
                                min_length,
                                max_length,
                                charset,
                                &self.groups,
                            )?
                        }
                        DataType::Float(min_reference, max_reference) => self
                            .generator
                            .generate_random_float(min_reference, max_reference, &self.groups)?
                            .to_string(),
                        DataType::Integer(min_reference, max_reference) => self
                            .generator
                            .generate_random_number(min_reference, max_reference, &self.groups)?
                            .to_string(),
                    };
                    output_text.push_str(&generated_text);
                    output_text.push(' ');
                }
            }
            UnitExpression::CapturingGroup {
                group_number,
                range: (min_reference, max_reference),
            } => {
                let random_number = self.generator.generate_positive_random_number(
                    min_reference,
                    max_reference,
                    &self.groups,
                )?;
                self.groups.insert(*group_number, random_number);

                output_text.push_str(&random_number.to_string());
                output_text.push(' ');
            }
            UnitExpression::NonCapturingGroup {
                nest_exp,
                repetition,
            } => {
                let repetition_count = self
                    .generator
                    .get_positive_value_from_reference(repetition, &self.groups)?;

                for _ in 1..=repetition_count {
                    let nest_gen = Generator::new_from_program(ClexLanguageAST {
                        expression: nest_exp.clone(),
                    });
                    let nested_output = nest_gen.traverse_ast(&mut self.groups)?;
                    output_text.push_str(&nested_output);
                }
            }
            UnitExpression::Eof => {
                // EOF marker - remove trailing space from last output
                // This is handled in the iterator's next() method
            }
        }

        Ok(output_text)
    }
}

impl Generator {
    pub fn new(syntax_tree: &Parser) -> Self {
        Self {
            syntax_tree: syntax_tree.get_language().clone(),
        }
    }

    fn new_from_program(program: ClexLanguageAST) -> Self {
        Self {
            syntax_tree: program,
        }
    }

    pub fn generate_testcases(&self) -> Result<String, ClexErrorType> {
        let mut groups = HashMap::new();
        self.traverse_ast(&mut groups)
    }

    /// Generate test cases as an iterator that yields chunks of data.
    ///
    /// This method returns an iterator that generates test case data incrementally,
    /// processing one unit expression at a time. This is more memory-efficient for
    /// large test cases compared to generating the entire test case at once.
    ///
    /// Each iteration yields the output from one unit expression in the AST.
    ///
    /// # Example
    ///
    /// ```rust
    /// use clex_gen::clex_language::parser::Parser;
    /// use clex_gen::clex_language::code_generator::Generator;
    ///
    /// let source = "N[1,100] N[1,100]";
    /// let mut parser = Parser::new(source.to_string()).unwrap();
    /// parser.parser().unwrap();
    ///
    /// let generator = Generator::new(&parser);
    /// for chunk in generator.generate_testcases_iter() {
    ///     match chunk {
    ///         Ok(data) => print!("{}", data),
    ///         Err(e) => {
    ///             eprintln!("Error: {}", e);
    ///             break;
    ///         }
    ///     }
    /// }
    /// // Need to manually remove trailing space in the final output if needed
    /// ```
    pub fn generate_testcases_iter(self) -> TestCaseIterator {
        TestCaseIterator::new(self)
    }

    fn traverse_ast(&self, groups: &mut HashMap<u64, u64>) -> Result<String, ClexErrorType> {
        let mut output_text = String::new();

        for unit_expression in &self.syntax_tree.expression {
            match unit_expression {
                UnitExpression::Primitives {
                    data_type,
                    repetition,
                } => {
                    let repetition_count =
                        self.get_positive_value_from_reference(repetition, groups)?;

                    for _ in 1..=repetition_count {
                        let generated_text = match data_type {
                            DataType::String(min_length, max_length, charset) => self
                                .generate_random_string(min_length, max_length, charset, groups)?,
                            DataType::Float(min_reference, max_reference) => self
                                .generate_random_float(min_reference, max_reference, groups)?
                                .to_string(),
                            DataType::Integer(min_reference, max_reference) => self
                                .generate_random_number(min_reference, max_reference, groups)?
                                .to_string(),
                        };
                        output_text.push_str(&generated_text);
                        output_text.push(' ');
                    }
                }
                UnitExpression::CapturingGroup {
                    group_number,
                    range: (min_reference, max_reference),
                } => {
                    let random_number =
                        self.generate_positive_random_number(min_reference, max_reference, groups)?;
                    groups.insert(*group_number, random_number);

                    output_text.push_str(&random_number.to_string());
                    output_text.push(' ');
                }
                UnitExpression::NonCapturingGroup {
                    nest_exp,
                    repetition,
                } => {
                    let repetition_count =
                        self.get_positive_value_from_reference(repetition, groups)?;

                    for _ in 1..=repetition_count {
                        let nest_gen = Self::new_from_program(ClexLanguageAST {
                            expression: nest_exp.clone(),
                        });
                        let nested_output = nest_gen.traverse_ast(groups)?;
                        output_text.push_str(&nested_output);
                    }
                }
                UnitExpression::Eof => {
                    // Removes the last character introduced by the last iteration before Eof
                    output_text.pop();
                    break;
                }
            }
        }

        Ok(output_text)
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
        Ok(rand::random_range(min..=max))
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
        Ok(rand::random_range(min..=max))
    }

    fn generate_random_string(
        &self,
        min_length: &PositiveReferenceType,
        max_length: &PositiveReferenceType,
        character_set: &CharacterSet,
        groups: &HashMap<u64, u64>,
    ) -> Result<String, ClexErrorType> {
        let min_length = self.get_positive_value_from_reference(min_length, groups)? as usize;
        let max_length = self.get_positive_value_from_reference(max_length, groups)? as usize;
        let length = self.generate_positive_random_integer(min_length as u64, max_length as u64)?;
        let charset = character_set.get_character_domain();
        Ok(Self::generate_random_string_from_charset(&charset, length))
    }

    fn generate_random_string_from_charset(charset: &str, length: u64) -> String {
        let charset = charset.as_bytes();
        let mut rng = rand::rng();
        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..charset.len());
                charset[idx] as char
            })
            .collect()
    }

    fn generate_random_number(
        &self,
        min_reference: &ReferenceType,
        max_reference: &ReferenceType,
        groups: &HashMap<u64, u64>,
    ) -> Result<i64, ClexErrorType> {
        let min = self.get_value_from_reference(min_reference, groups)?;
        let max = self.get_value_from_reference(max_reference, groups)?;

        self.generate_random_integer(min, max)
    }

    fn generate_positive_random_number(
        &self,
        min_reference: &PositiveReferenceType,
        max_reference: &PositiveReferenceType,
        groups: &HashMap<u64, u64>,
    ) -> Result<u64, ClexErrorType> {
        let min = self.get_positive_value_from_reference(min_reference, groups)?;
        let max = self.get_positive_value_from_reference(max_reference, groups)?;

        self.generate_positive_random_integer(min, max)
    }

    fn generate_random_float(
        &self,
        min_reference: &ReferenceType,
        max_reference: &ReferenceType,
        groups: &HashMap<u64, u64>,
    ) -> Result<f64, ClexErrorType> {
        let min = self.get_value_from_reference(min_reference, groups)? as f64;
        let max = self.get_value_from_reference(max_reference, groups)? as f64;

        if min > max {
            return Err(ClexErrorType::InvalidRangeValues(
                ParentErrorType::GeneratorError,
                min as i64,
                max as i64,
            ));
        }

        Ok(rand::random_range(min..=max))
    }

    fn get_value_from_reference(
        &self,
        reference_type: &ReferenceType,
        groups: &HashMap<u64, u64>,
    ) -> Result<i64, ClexErrorType> {
        Ok(match reference_type {
            ReferenceType::ByGroup { group_number: gn } => {
                self.get_count_from_group(groups, *gn)? as i64
            }
            ReferenceType::ByLiteral(value) => *value,
        })
    }

    fn get_positive_value_from_reference(
        &self,
        reference_type: &PositiveReferenceType,
        groups: &HashMap<u64, u64>,
    ) -> Result<u64, ClexErrorType> {
        Ok(match reference_type {
            PositiveReferenceType::ByGroup { group_number: gn } => {
                self.get_count_from_group(groups, *gn)?
            }
            PositiveReferenceType::ByLiteral(value) => *value,
        })
    }

    fn get_count_from_group(
        &self,
        groups: &HashMap<u64, u64>,
        group_number: u64,
    ) -> Result<u64, ClexErrorType> {
        match groups.get(&group_number) {
            Some(value) => Ok(*value),
            None => Err(ClexErrorType::UnknownGroupNumber(
                ParentErrorType::GeneratorError,
                group_number,
            )),
        }
    }
}

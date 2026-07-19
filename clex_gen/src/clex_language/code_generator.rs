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
    CharacterSet, ClexLanguageAST, DataType, FloatReferenceType, GraphKind, PositiveReferenceType,
    ReferenceType, Repetition, UnitExpression,
};
use crate::clex_language::clex_error_type::{ClexErrorType, ParentErrorType};
use crate::clex_language::lexer::Span;
use crate::clex_language::parser::Parser;
use rand::seq::SliceRandom;
use rand::RngExt;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Generator {
    syntax_tree: ClexLanguageAST,
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
        self.traverse_ast_with_loop(&mut groups, 1)
    }

    fn traverse_ast_with_loop(&self, groups: &mut HashMap<u64, u64>, current_loop_index: u64) -> Result<String, ClexErrorType> {
        let mut output_text = String::new();

        for unit_expression in &self.syntax_tree.expression {
            match unit_expression {
                UnitExpression::Primitives {
                    data_type,
                    repetition,
                    span,
                } => {
                    let repetition_count = self.get_repetition_count(repetition, groups, *span, current_loop_index)?;

                    for _ in 1..=repetition_count {
                        let generated_text = match data_type {
                            DataType::String(min_length, max_length, charset) => self
                                .generate_random_string(
                                    min_length, max_length, charset, groups, *span, current_loop_index,
                                )?,
                            DataType::Float(min_reference, max_reference) => self
                                .generate_random_float(min_reference, max_reference, groups, *span, current_loop_index)?
                                .to_string(),
                            DataType::Integer(min_reference, max_reference) => self
                                .generate_random_number(
                                    min_reference,
                                    max_reference,
                                    groups,
                                    *span,
                                    current_loop_index,
                                )?
                                .to_string(),
                        };
                        output_text.push_str(&generated_text);
                        output_text.push(' ');
                    }
                }
                UnitExpression::CapturingGroup {
                    group_number,
                    range: (min_reference, max_reference),
                    span,
                } => {
                    let random_number = self.generate_positive_random_number(
                        min_reference,
                        max_reference,
                        groups,
                        *span,
                        current_loop_index,
                    )?;
                    groups.insert(*group_number, random_number);

                    output_text.push_str(&random_number.to_string());
                    output_text.push(' ');
                }
                UnitExpression::NonCapturingGroup {
                    branches,
                    repetition,
                    span,
                } => {
                    let repetition_count = self.get_repetition_count(repetition, groups, *span, current_loop_index)?;

                    for step in 1..=repetition_count {
                        if branches.is_empty() {
                            continue;
                        }
                        let branch_idx = if branches.len() == 1 {
                            0
                        } else {
                            rand::rng().random_range(0..branches.len())
                        };

                        let nest_gen = Self::new_from_program(ClexLanguageAST {
                            expression: branches[branch_idx].clone(),
                        });
                        let nested_output = nest_gen.traverse_ast_with_loop(groups, step)?;
                        output_text.push_str(&nested_output);
                    }
                }
                UnitExpression::Literal { text, span: _ } => {
                    output_text.push_str(text);
                }
                UnitExpression::GraphPrimitive { kind, span } => {
                    let gen_text =
                        self.generate_graph_primitive(kind, groups, *span, current_loop_index)?;
                    output_text.push_str(&gen_text);
                }
                UnitExpression::Eof => {
                    if output_text.ends_with(' ') {
                        output_text.pop();
                    }
                    break;
                }
            }
        }

        Ok(output_text)
    }

    fn get_repetition_count(
        &self,
        repetition: &Repetition,
        groups: &HashMap<u64, u64>,
        span: Span,
        loop_index: u64,
    ) -> Result<u64, ClexErrorType> {
        match repetition {
            Repetition::Exact(pos_ref) => {
                self.get_positive_value_from_reference(pos_ref, groups, span, loop_index)
            }
            Repetition::Range(r1, r2) => {
                let min = self.get_positive_value_from_reference(r1, groups, span, loop_index)?;
                let max = self.get_positive_value_from_reference(r2, groups, span, loop_index)?;
                self.generate_positive_random_integer(min, max, span)
            }
        }
    }

    // Helper method for generating random integers
    fn generate_random_integer(
        &self,
        min: i64,
        max: i64,
        span: Span,
    ) -> Result<i64, ClexErrorType> {
        if min > max {
            return Err(ClexErrorType::InvalidRangeValues(
                ParentErrorType::GeneratorError,
                span,
                min,
                max,
            ));
        }
        Ok(rand::random_range(min..=max))
    }

    // Helper method for generating random positive integers
    fn generate_positive_random_integer(
        &self,
        min: u64,
        max: u64,
        span: Span,
    ) -> Result<u64, ClexErrorType> {
        if min > max {
            return Err(ClexErrorType::InvalidRangeValues(
                ParentErrorType::GeneratorError,
                span,
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
        span: Span,
        loop_index: u64,
    ) -> Result<String, ClexErrorType> {
        let min_length = self.get_positive_value_from_reference(min_length, groups, span, loop_index)? as usize;
        let max_length = self.get_positive_value_from_reference(max_length, groups, span, loop_index)? as usize;
        let length =
            self.generate_positive_random_integer(min_length as u64, max_length as u64, span)?;
        let charset = character_set.get_character_domain();
        Ok(Self::generate_random_string_from_charset(&charset, length))
    }

    fn generate_random_string_from_charset(charset: &str, length: u64) -> String {
        let chars: Vec<char> = charset.chars().collect();
        if chars.is_empty() {
            return String::new();
        }
        let mut rng = rand::rng();
        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..chars.len());
                chars[idx]
            })
            .collect()
    }

    fn generate_random_number(
        &self,
        min_reference: &ReferenceType,
        max_reference: &ReferenceType,
        groups: &HashMap<u64, u64>,
        span: Span,
        loop_index: u64,
    ) -> Result<i64, ClexErrorType> {
        let min = self.get_value_from_reference(min_reference, groups, span, loop_index)?;
        let max = self.get_value_from_reference(max_reference, groups, span, loop_index)?;

        self.generate_random_integer(min, max, span)
    }

    fn generate_positive_random_number(
        &self,
        min_reference: &PositiveReferenceType,
        max_reference: &PositiveReferenceType,
        groups: &HashMap<u64, u64>,
        span: Span,
        loop_index: u64,
    ) -> Result<u64, ClexErrorType> {
        let min = self.get_positive_value_from_reference(min_reference, groups, span, loop_index)?;
        let max = self.get_positive_value_from_reference(max_reference, groups, span, loop_index)?;

        self.generate_positive_random_integer(min, max, span)
    }

    fn generate_random_float(
        &self,
        min_reference: &FloatReferenceType,
        max_reference: &FloatReferenceType,
        groups: &HashMap<u64, u64>,
        span: Span,
        loop_index: u64,
    ) -> Result<f64, ClexErrorType> {
        let min = self.get_float_value_from_reference(min_reference, groups, span, loop_index)?;
        let max = self.get_float_value_from_reference(max_reference, groups, span, loop_index)?;

        if min > max {
            return Err(ClexErrorType::InvalidRangeValues(
                ParentErrorType::GeneratorError,
                span,
                min as i64,
                max as i64,
            ));
        }

        Ok(rand::random_range(min..=max))
    }

    fn get_float_value_from_reference(
        &self,
        reference_type: &FloatReferenceType,
        groups: &HashMap<u64, u64>,
        span: Span,
        _loop_index: u64,
    ) -> Result<f64, ClexErrorType> {
        Ok(match reference_type {
            FloatReferenceType::ByGroup { group_number: gn } => {
                self.get_count_from_group(groups, *gn, span)? as f64
            }
            FloatReferenceType::ByLiteral(value) => *value,
        })
    }

    fn get_value_from_reference(
        &self,
        reference_type: &ReferenceType,
        groups: &HashMap<u64, u64>,
        span: Span,
        loop_index: u64,
    ) -> Result<i64, ClexErrorType> {
        Ok(match reference_type {
            ReferenceType::ByGroup { group_number: gn } => {
                self.get_count_from_group(groups, *gn, span)? as i64
            }
            ReferenceType::ByLiteral(value) => *value,
            ReferenceType::ByLoopIndex { offset } => loop_index as i64 + offset,
        })
    }

    fn get_positive_value_from_reference(
        &self,
        reference_type: &PositiveReferenceType,
        groups: &HashMap<u64, u64>,
        span: Span,
        loop_index: u64,
    ) -> Result<u64, ClexErrorType> {
        Ok(match reference_type {
            PositiveReferenceType::ByGroup { group_number: gn } => {
                self.get_count_from_group(groups, *gn, span)?
            }
            PositiveReferenceType::ByLiteral(value) => *value,
            PositiveReferenceType::ByLoopIndex { offset } => {
                (loop_index as i64 + offset).max(0) as u64
            }
        })
    }

    fn get_count_from_group(
        &self,
        groups: &HashMap<u64, u64>,
        group_number: u64,
        span: Span,
    ) -> Result<u64, ClexErrorType> {
        match groups.get(&group_number) {
            Some(value) => Ok(*value),
            None => Err(ClexErrorType::UnknownGroupNumber(
                ParentErrorType::GeneratorError,
                span,
                group_number,
            )),
        }
    }

    fn generate_graph_primitive(
        &self,
        kind: &GraphKind,
        groups: &HashMap<u64, u64>,
        span: Span,
        loop_index: u64,
    ) -> Result<String, ClexErrorType> {
        let mut rng = rand::rng();
        match kind {
            GraphKind::Tree(n_ref) => {
                let n = self.get_positive_value_from_reference(n_ref, groups, span, loop_index)?;
                if n <= 1 {
                    return Ok(String::new());
                }
                let mut edges = Vec::new();
                for i in 2..=n {
                    let p = rng.random_range(1..i);
                    edges.push((i, p));
                }
                let mut perm: Vec<u64> = (1..=n).collect();
                perm.shuffle(&mut rng);

                let mut out = String::new();
                for (u, v) in edges {
                    let u_mapped = perm[(u - 1) as usize];
                    let v_mapped = perm[(v - 1) as usize];
                    out.push_str(&format!("{u_mapped} {v_mapped}\n"));
                }
                Ok(out)
            }
            GraphKind::Graph(n_ref, m_ref) => {
                let n = self.get_positive_value_from_reference(n_ref, groups, span, loop_index)?;
                let requested_m =
                    self.get_positive_value_from_reference(m_ref, groups, span, loop_index)?;
                if n <= 1 {
                    return Ok(String::new());
                }
                let max_edges = n * (n - 1) / 2;
                let m = requested_m.min(max_edges);

                let mut edge_set = HashSet::new();
                while (edge_set.len() as u64) < m {
                    let u = rng.random_range(1..=n);
                    let v = rng.random_range(1..=n);
                    if u != v {
                        let edge = if u < v { (u, v) } else { (v, u) };
                        edge_set.insert(edge);
                    }
                }
                let mut out = String::new();
                for (mut u, mut v) in edge_set {
                    if rng.random_bool(0.5) {
                        std::mem::swap(&mut u, &mut v);
                    }
                    out.push_str(&format!("{u} {v}\n"));
                }
                Ok(out)
            }
            GraphKind::Dag(n_ref, m_ref) => {
                let n = self.get_positive_value_from_reference(n_ref, groups, span, loop_index)?;
                let requested_m =
                    self.get_positive_value_from_reference(m_ref, groups, span, loop_index)?;
                if n <= 1 {
                    return Ok(String::new());
                }
                let max_edges = n * (n - 1) / 2;
                let m = requested_m.min(max_edges);

                let mut edge_set = HashSet::new();
                while (edge_set.len() as u64) < m {
                    let u = rng.random_range(1..=n);
                    let v = rng.random_range(1..=n);
                    if u != v {
                        let edge = if u < v { (u, v) } else { (v, u) };
                        edge_set.insert(edge);
                    }
                }
                let mut out = String::new();
                for (u, v) in edge_set {
                    out.push_str(&format!("{u} {v}\n"));
                }
                Ok(out)
            }
            GraphKind::Permutation(n_ref) => {
                let n = self.get_positive_value_from_reference(n_ref, groups, span, loop_index)?;
                if n == 0 {
                    return Ok(String::new());
                }
                let mut perm: Vec<u64> = (1..=n).collect();
                perm.shuffle(&mut rng);
                let perm_strs: Vec<String> = perm.into_iter().map(|x| x.to_string()).collect();
                let mut out = perm_strs.join(" ");
                out.push(' ');
                Ok(out)
            }
        }
    }
}

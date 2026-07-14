//! Private parser experiment for a deliberately small structured pseudocode.
//!
//! The input fixtures are not a registry format or public API. They exist only
//! to compare an editable textual notation with the current Rust AST builders.

use crate::ast::{
    AlgorithmAst, EffectSummary, Parameter, ParameterMode, SemanticOperation, Statement,
    experimental_expression, experimental_operation,
};
use crate::expressions::{Expression, ValueType, binary, variable};

#[derive(Debug, Eq, PartialEq)]
struct ParseError {
    line: usize,
    message: String,
}

impl ParseError {
    fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

type ParseResult<T> = Result<T, ParseError>;

struct Parser {
    lines: Vec<(usize, &'static str)>,
    cursor: usize,
}

impl Parser {
    fn new(source: &'static str) -> Self {
        Self {
            lines: source
                .lines()
                .enumerate()
                .filter_map(|(index, line)| {
                    let line = line.trim();
                    (!line.is_empty() && !line.starts_with('#')).then_some((index + 1, line))
                })
                .collect(),
            cursor: 0,
        }
    }

    fn parse(mut self) -> ParseResult<AlgorithmAst> {
        let ast_version = self.header("ast-version")?;
        let id = self.header("ast-id")?;
        let algorithm_id = self.header("algorithm")?;
        let mut parameters = Vec::new();
        while self.peek_starts_with("parameter ") {
            let (line, text) = self.next()?;
            let fields = fields(line, text.strip_prefix("parameter ").unwrap())?;
            if fields.len() != 4 {
                return Err(ParseError::new(line, "parameter needs four fields"));
            }
            parameters.push(Parameter {
                name: fields[0],
                data_type: fields[1],
                value_type: value_type(line, fields[2])?,
                mode: parameter_mode(line, fields[3])?,
            });
        }
        let effects = EffectSummary {
            mutates: self.effect("mutates")?,
            allocations: self.effect("allocations")?,
            copies: self.effect("copies")?,
        };
        self.expect("begin")?;
        let body = self.block()?;
        if self.cursor != self.lines.len() {
            let (line, text) = self.lines[self.cursor];
            return Err(ParseError::new(line, format!("unexpected {text:?}")));
        }
        let ast = AlgorithmAst {
            ast_version,
            id,
            algorithm_id,
            parameters,
            effects,
            body,
        };
        if let Some(error) = ast.validate().into_iter().next() {
            return Err(ParseError::new(
                0,
                format!("invalid resulting AST: {error}"),
            ));
        }
        Ok(ast)
    }

    fn block(&mut self) -> ParseResult<Vec<Statement>> {
        let mut statements = Vec::new();
        while let Some((line, text)) = self.peek() {
            if matches!(text, "end" | "else") {
                break;
            }
            self.cursor += 1;
            if let Some(binding) = text.strip_prefix("let ") {
                let fields = fields(line, binding)?;
                if fields.len() != 2 {
                    return Err(ParseError::new(line, "let needs a name and expression"));
                }
                statements.push(Statement::Let {
                    name: fields[0],
                    expression: expression(line, fields[1])?,
                });
            } else if let Some(rest) = text.strip_prefix("operation ") {
                let fields = fields(line, rest)?;
                if fields.len() != 3 {
                    return Err(ParseError::new(
                        line,
                        "operation needs ID, kind and description",
                    ));
                }
                statements.push(experimental_operation(
                    fields[0],
                    operation(line, fields[1])?,
                    fields[2],
                ));
            } else if let Some(condition) = text.strip_prefix("while ") {
                let body = self.block()?;
                self.expect("end")?;
                statements.push(Statement::While {
                    condition: expression(line, condition)?,
                    body,
                });
            } else if let Some(condition) = text.strip_prefix("if ") {
                let then_branch = self.block()?;
                let else_branch = if self.peek().is_some_and(|(_, text)| text == "else") {
                    self.cursor += 1;
                    self.block()?
                } else {
                    Vec::new()
                };
                self.expect("end")?;
                statements.push(Statement::If {
                    condition: condition_expression(line, condition)?,
                    then_branch,
                    else_branch,
                });
            } else if let Some(value) = text.strip_prefix("return ") {
                statements.push(Statement::Return {
                    expression: expression(line, value)?,
                });
            } else if text == "break" {
                statements.push(Statement::Break);
            } else {
                return Err(ParseError::new(line, format!("unknown statement {text:?}")));
            }
        }
        Ok(statements)
    }

    fn header(&mut self, name: &str) -> ParseResult<&'static str> {
        let (line, text) = self.next()?;
        text.strip_prefix(&format!("{name} "))
            .ok_or_else(|| ParseError::new(line, format!("expected {name}")))
    }

    fn effect(&mut self, name: &str) -> ParseResult<Vec<&'static str>> {
        let (line, text) = self.next()?;
        let value = text
            .strip_prefix(&format!("effects {name} "))
            .ok_or_else(|| ParseError::new(line, format!("expected effects {name}")))?;
        if value == "-" {
            Ok(Vec::new())
        } else {
            Ok(value.split(", ").collect())
        }
    }

    fn expect(&mut self, expected: &str) -> ParseResult<()> {
        let (line, text) = self.next()?;
        if text == expected {
            Ok(())
        } else {
            Err(ParseError::new(line, format!("expected {expected:?}")))
        }
    }

    fn peek(&self) -> Option<(usize, &'static str)> {
        self.lines.get(self.cursor).copied()
    }

    fn peek_starts_with(&self, prefix: &str) -> bool {
        self.peek()
            .is_some_and(|(_, text)| text.starts_with(prefix))
    }

    fn next(&mut self) -> ParseResult<(usize, &'static str)> {
        let line = self
            .peek()
            .ok_or_else(|| ParseError::new(0, "unexpected end of input"))?;
        self.cursor += 1;
        Ok(line)
    }
}

fn fields(line: usize, text: &'static str) -> ParseResult<Vec<&'static str>> {
    let fields: Vec<_> = text.split(" | ").collect();
    if fields.iter().any(|field| field.is_empty()) {
        return Err(ParseError::new(line, "empty field"));
    }
    Ok(fields)
}

fn value_type(line: usize, value: &str) -> ParseResult<ValueType> {
    match value {
        "Bool" => Ok(ValueType::Bool),
        "Comparator" => Ok(ValueType::Comparator),
        "Index" => Ok(ValueType::Index),
        "OptionalElement" => Ok(ValueType::OptionalElement),
        "Predicate" => Ok(ValueType::Predicate),
        "Sequence" => Ok(ValueType::Sequence),
        _ => Err(ParseError::new(
            line,
            format!("unknown value type {value:?}"),
        )),
    }
}

fn parameter_mode(line: usize, value: &str) -> ParseResult<ParameterMode> {
    match value {
        "Read" => Ok(ParameterMode::Read),
        "ReadWrite" => Ok(ParameterMode::ReadWrite),
        "Output" => Ok(ParameterMode::Output),
        _ => Err(ParseError::new(
            line,
            format!("unknown parameter mode {value:?}"),
        )),
    }
}

fn operation(line: usize, value: &str) -> ParseResult<SemanticOperation> {
    match value {
        "Read" => Ok(SemanticOperation::Read),
        "Predicate" => Ok(SemanticOperation::Predicate),
        "Swap" => Ok(SemanticOperation::Swap),
        "Partition" => Ok(SemanticOperation::Partition),
        "Compare" => Ok(SemanticOperation::Compare),
        "Assert" => Ok(SemanticOperation::Assert),
        _ => Err(ParseError::new(
            line,
            format!("unknown operation {value:?}"),
        )),
    }
}

fn expression(line: usize, value: &'static str) -> ParseResult<Expression> {
    match value {
        "true" => Ok(Expression::Boolean(true)),
        "false" => Ok(Expression::Boolean(false)),
        "none" => Ok(Expression::NoneElement),
        "1" => Ok(Expression::Integer(1)),
        _ => known_expression(line, value),
    }
}

fn known_expression(line: usize, value: &'static str) -> ParseResult<Expression> {
    const KNOWN: &[&str] = &[
        "0",
        "values",
        "boundary",
        "minimum_index",
        "length(values)",
        "length(values) = 0",
        "left < right",
        "left + 1",
        "right - 1",
        "index < length(values)",
        "index + 1",
        "index",
        "current > 0",
        "current - 1",
        "left < floor(length(values) / 2)",
        "length(values) - 1 - left",
        "predicate result is true",
        "predicate result is false",
        "some(values[minimum_index])",
    ];
    if KNOWN.contains(&value) {
        Ok(experimental_expression(value))
    } else {
        Err(ParseError::new(
            line,
            format!("unknown expression {value:?}"),
        ))
    }
}

fn condition_expression(line: usize, value: &'static str) -> ParseResult<Expression> {
    if value == "adjacent inversion" {
        let values = variable("values", ValueType::Sequence);
        let index = variable("index", ValueType::Index);
        let left = Expression::Index {
            sequence: Box::new(values.clone()),
            index: Box::new(binary(
                crate::expressions::BinaryOperator::Subtract,
                index.clone(),
                Expression::Integer(1),
            )),
        };
        let right = Expression::Index {
            sequence: Box::new(values),
            index: Box::new(index),
        };
        Ok(Expression::Call {
            function: "adjacent_inversion",
            arguments: vec![left, right],
            result_type: ValueType::Bool,
        })
    } else if value == "current value is not less" {
        let values = variable("values", ValueType::Sequence);
        let current = variable("current", ValueType::Index);
        Ok(Expression::Call {
            function: "current_is_not_less",
            arguments: vec![
                Expression::Index {
                    sequence: Box::new(values.clone()),
                    index: Box::new(current.clone()),
                },
                Expression::Index {
                    sequence: Box::new(values),
                    index: Box::new(binary(
                        crate::expressions::BinaryOperator::Subtract,
                        current,
                        Expression::Integer(1),
                    )),
                },
            ],
            result_type: ValueType::Bool,
        })
    } else if value == "candidate is less than minimum" {
        let values = variable("values", ValueType::Sequence);
        Ok(Expression::Call {
            function: "candidate_is_less",
            arguments: vec![
                Expression::Index {
                    sequence: Box::new(values.clone()),
                    index: Box::new(variable("index", ValueType::Index)),
                },
                Expression::Index {
                    sequence: Box::new(values),
                    index: Box::new(variable("minimum_index", ValueType::Index)),
                },
            ],
            result_type: ValueType::Bool,
        })
    } else {
        expression(line, value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Parser, known_expression};

    const IS_SORTED: &str = include_str!("../pseudocode/is_sorted.atlas-pseudo");
    const PARTITION: &str = include_str!("../pseudocode/partition.atlas-pseudo");
    const INSERTION: &str = include_str!("../pseudocode/insertion_sort.atlas-pseudo");
    const REVERSE: &str = include_str!("../pseudocode/reverse.atlas-pseudo");
    const MINIMUM: &str = include_str!("../pseudocode/minimum.atlas-pseudo");

    #[test]
    fn editable_sources_match_the_rust_ast_builders() {
        assert_eq!(
            Parser::new(IS_SORTED).parse(),
            Ok(crate::ast::is_sorted_ast())
        );
        assert_eq!(
            Parser::new(PARTITION).parse(),
            Ok(crate::ast::partition_ast())
        );
        assert_eq!(
            Parser::new(INSERTION).parse(),
            Ok(crate::ast::insertion_sort_ast())
        );
        assert_eq!(Parser::new(REVERSE).parse(), Ok(crate::ast::reverse_ast()));
        assert_eq!(Parser::new(MINIMUM).parse(), Ok(crate::ast::minimum_ast()));
    }

    #[test]
    fn parser_rejects_an_unmodeled_expression_with_its_line_number() {
        let error = known_expression(17, "values[index] + 1").expect_err("unknown expression");

        assert_eq!(error.line, 17);
        assert!(error.message.contains("unknown expression"));
    }
}

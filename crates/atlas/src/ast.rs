use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use crate::expressions::{BinaryOperator as Bin, Expression, ValueType, binary, variable};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticOperation {
    Read,
    Write,
    Compare,
    Swap,
    Recurse,
    Allocate,
    Copy,
    Partition,
    Predicate,
    Assert,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub name: &'static str,
    pub data_type: &'static str,
    pub value_type: ValueType,
    pub mode: ParameterMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterMode {
    Read,
    ReadWrite,
    Output,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectSummary {
    pub mutates: Vec<&'static str>,
    pub allocations: Vec<&'static str>,
    pub copies: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlgorithmAst {
    pub ast_version: &'static str,
    pub id: &'static str,
    pub algorithm_id: &'static str,
    pub parameters: Vec<Parameter>,
    pub effects: EffectSummary,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Let {
        name: &'static str,
        expression: Expression,
    },
    Operation {
        id: &'static str,
        operation: SemanticOperation,
        description: &'static str,
        operands: Vec<Expression>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    ForEach {
        binding: &'static str,
        collection: Expression,
        body: Vec<Statement>,
    },
    Return {
        expression: Expression,
    },
    Break,
}

impl AlgorithmAst {
    pub fn render(&self) -> String {
        let mut output = String::new();
        writeln!(&mut output, "algorithm {}", self.algorithm_id).unwrap();
        for parameter in &self.parameters {
            writeln!(
                &mut output,
                "  parameter {}: {} [{:?}]",
                parameter.name, parameter.data_type, parameter.mode
            )
            .unwrap();
        }
        writeln!(
            &mut output,
            "  effects mutate={:?} allocate={:?} copy={:?}",
            self.effects.mutates, self.effects.allocations, self.effects.copies
        )
        .unwrap();
        render_block(&self.body, 1, &mut output);
        output
    }

    pub fn operation_kinds(&self) -> HashSet<SemanticOperation> {
        let mut operations = HashSet::new();
        collect_operations(&self.body, &mut operations);
        operations
    }

    pub fn operation_by_id(&self, id: &str) -> Option<SemanticOperation> {
        find_operation(&self.body, id)
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let mut ids = HashSet::new();
        validate_statements(&self.body, &mut ids, &mut errors);
        let mut variables: HashMap<&str, ValueType> = self
            .parameters
            .iter()
            .map(|parameter| (parameter.name, parameter.value_type))
            .collect();
        validate_typed_statements(&self.body, &mut variables, &mut errors);
        let modes: HashMap<_, _> = self
            .parameters
            .iter()
            .map(|parameter| (parameter.name, parameter.mode))
            .collect();
        validate_operation_modes(&self.body, &modes, &mut errors);
        if self.effects.mutates.is_empty() {
            errors.push("at least one mutated value must be explicit".to_owned());
        }
        errors
    }
}

fn validate_operation_modes(
    statements: &[Statement],
    modes: &HashMap<&str, ParameterMode>,
    errors: &mut Vec<String>,
) {
    for statement in statements {
        match statement {
            Statement::Operation {
                id,
                operation,
                operands,
                ..
            } => {
                let targets: &[usize] = match operation {
                    SemanticOperation::Write => &[0],
                    SemanticOperation::Swap => &[0, 1],
                    SemanticOperation::Copy => &[1],
                    SemanticOperation::Partition => &[0],
                    _ => &[],
                };
                for target in targets {
                    let Some(name) = operands.get(*target).and_then(Expression::root_variable)
                    else {
                        errors.push(format!("operation {id:?} has no explicit write target"));
                        continue;
                    };
                    if !matches!(
                        modes.get(name),
                        Some(ParameterMode::ReadWrite | ParameterMode::Output)
                    ) {
                        errors.push(format!(
                            "operation {id:?} writes {name:?}, which is not writable"
                        ));
                    }
                }
            }
            Statement::If {
                then_branch,
                else_branch,
                ..
            } => {
                validate_operation_modes(then_branch, modes, errors);
                validate_operation_modes(else_branch, modes, errors);
            }
            Statement::While { body, .. } | Statement::ForEach { body, .. } => {
                validate_operation_modes(body, modes, errors);
            }
            Statement::Let { .. } | Statement::Return { .. } | Statement::Break => {}
        }
    }
}

fn validate_typed_statements(
    statements: &[Statement],
    variables: &mut HashMap<&'static str, ValueType>,
    errors: &mut Vec<String>,
) {
    for statement in statements {
        match statement {
            Statement::Let { name, expression } => {
                errors.extend(expression.validate(variables));
                if let Ok(value_type) = expression.value_type() {
                    if let Some(previous) = variables.insert(name, value_type)
                        && previous != value_type
                    {
                        errors.push(format!(
                            "assignment to {name:?} changes type from {previous:?} to {value_type:?}"
                        ));
                    }
                }
            }
            Statement::Operation {
                operation,
                operands,
                ..
            } => {
                for operand in operands {
                    errors.extend(operand.validate(variables));
                }
                if *operation == SemanticOperation::Predicate {
                    variables.insert("predicate_result", ValueType::Bool);
                }
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                validate_boolean(condition, variables, "if condition", errors);
                let mut then_variables = variables.clone();
                validate_typed_statements(then_branch, &mut then_variables, errors);
                let mut else_variables = variables.clone();
                validate_typed_statements(else_branch, &mut else_variables, errors);
            }
            Statement::While { condition, body } => {
                validate_boolean(condition, variables, "while condition", errors);
                let mut body_variables = variables.clone();
                validate_typed_statements(body, &mut body_variables, errors);
                if let Some(result_type) = body_variables.get("predicate_result") {
                    variables.insert("predicate_result", *result_type);
                }
            }
            Statement::ForEach {
                binding,
                collection,
                body,
            } => {
                errors.extend(collection.validate(variables));
                if collection.value_type() != Ok(ValueType::Range) {
                    errors.push(format!("for-each collection must be Range: {binding:?}"));
                }
                let mut body_variables = variables.clone();
                body_variables.insert(binding, ValueType::Index);
                validate_typed_statements(body, &mut body_variables, errors);
            }
            Statement::Return { expression } => errors.extend(expression.validate(variables)),
            Statement::Break => {}
        }
    }
}

fn validate_boolean(
    expression: &Expression,
    variables: &HashMap<&str, ValueType>,
    role: &str,
    errors: &mut Vec<String>,
) {
    errors.extend(expression.validate(variables));
    if expression.value_type() != Ok(ValueType::Bool) {
        errors.push(format!("{role} must have type Bool"));
    }
}

fn find_operation(statements: &[Statement], id: &str) -> Option<SemanticOperation> {
    for statement in statements {
        match statement {
            Statement::Operation {
                id: candidate,
                operation,
                ..
            } if *candidate == id => return Some(*operation),
            Statement::If {
                then_branch,
                else_branch,
                ..
            } => {
                if let Some(operation) =
                    find_operation(then_branch, id).or_else(|| find_operation(else_branch, id))
                {
                    return Some(operation);
                }
            }
            Statement::While { body, .. } | Statement::ForEach { body, .. } => {
                if let Some(operation) = find_operation(body, id) {
                    return Some(operation);
                }
            }
            Statement::Let { .. }
            | Statement::Operation { .. }
            | Statement::Return { .. }
            | Statement::Break => {}
        }
    }
    None
}

fn render_block(statements: &[Statement], depth: usize, output: &mut String) {
    let indent = "  ".repeat(depth);
    for statement in statements {
        match statement {
            Statement::Let { name, expression } => {
                writeln!(output, "{indent}let {name} = {}", expression.render()).unwrap();
            }
            Statement::Operation {
                id,
                operation,
                description,
                operands,
            } => {
                let rendered = operands
                    .iter()
                    .map(Expression::render)
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(
                    output,
                    "{indent}{id}: {operation:?} {description} [{rendered}]"
                )
                .unwrap();
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                writeln!(output, "{indent}if {}", condition.render()).unwrap();
                render_block(then_branch, depth + 1, output);
                if !else_branch.is_empty() {
                    writeln!(output, "{indent}else").unwrap();
                    render_block(else_branch, depth + 1, output);
                }
                writeln!(output, "{indent}end if").unwrap();
            }
            Statement::While { condition, body } => {
                writeln!(output, "{indent}while {}", condition.render()).unwrap();
                render_block(body, depth + 1, output);
                writeln!(output, "{indent}end while").unwrap();
            }
            Statement::ForEach {
                binding,
                collection,
                body,
            } => {
                writeln!(
                    output,
                    "{indent}for each {binding} in {}",
                    collection.render()
                )
                .unwrap();
                render_block(body, depth + 1, output);
                writeln!(output, "{indent}end for").unwrap();
            }
            Statement::Return { expression } => {
                writeln!(output, "{indent}return {}", expression.render()).unwrap();
            }
            Statement::Break => {
                writeln!(output, "{indent}break").unwrap();
            }
        }
    }
}

fn collect_operations(statements: &[Statement], operations: &mut HashSet<SemanticOperation>) {
    for statement in statements {
        match statement {
            Statement::Operation { operation, .. } => {
                operations.insert(*operation);
            }
            Statement::If {
                then_branch,
                else_branch,
                ..
            } => {
                collect_operations(then_branch, operations);
                collect_operations(else_branch, operations);
            }
            Statement::While { body, .. } | Statement::ForEach { body, .. } => {
                collect_operations(body, operations);
            }
            Statement::Let { .. } | Statement::Return { .. } | Statement::Break => {}
        }
    }
}

fn validate_statements(
    statements: &[Statement],
    ids: &mut HashSet<&'static str>,
    errors: &mut Vec<String>,
) {
    for statement in statements {
        match statement {
            Statement::Operation { id, .. } => {
                if id.is_empty() {
                    errors.push("operation ID must not be empty".to_owned());
                } else if !ids.insert(id) {
                    errors.push(format!("duplicate operation ID {id:?}"));
                }
            }
            Statement::If {
                then_branch,
                else_branch,
                ..
            } => {
                validate_statements(then_branch, ids, errors);
                validate_statements(else_branch, ids, errors);
            }
            Statement::While { body, .. } | Statement::ForEach { body, .. } => {
                validate_statements(body, ids, errors);
            }
            Statement::Let { .. } | Statement::Return { .. } | Statement::Break => {}
        }
    }
}

fn operation(
    id: &'static str,
    operation: SemanticOperation,
    description: &'static str,
) -> Statement {
    Statement::Operation {
        id,
        operation,
        description,
        operands: operation_operands(id),
    }
}

fn e(source: &'static str) -> Expression {
    let values = || variable("values", ValueType::Sequence);
    let index = |name| variable(name, ValueType::Index);
    match source {
        "0" => Expression::Integer(0),
        "values" => values(),
        "middle" | "boundary" => index(source),
        "length(values)" => Expression::Length(Box::new(values())),
        "floor(length(values) / 2)" => binary(
            Bin::Divide,
            Expression::Length(Box::new(values())),
            Expression::Integer(2),
        ),
        "length(values) < 2" => binary(
            Bin::LessThan,
            Expression::Length(Box::new(values())),
            Expression::Integer(2),
        ),
        "left < right" => binary(Bin::LessThan, index("left"), index("right")),
        "left + 1" => binary(Bin::Add, index("left"), Expression::Integer(1)),
        "right - 1" => binary(Bin::Subtract, index("right"), Expression::Integer(1)),
        "left maximum <= right minimum" => Expression::Call {
            function: "runs_are_ordered",
            arguments: vec![values(), index("middle")],
            result_type: ValueType::Bool,
        },
        "predicate result is true" => variable("predicate_result", ValueType::Bool),
        "predicate result is false" => {
            Expression::Not(Box::new(variable("predicate_result", ValueType::Bool)))
        }
        "scratch[0..length(values)]" => Expression::Range {
            sequence: Box::new(variable("scratch", ValueType::Sequence)),
            start: Box::new(Expression::Integer(0)),
            end: Box::new(Expression::Length(Box::new(values()))),
        },
        _ => panic!("unmodeled experimental expression {source:?}"),
    }
}

fn operation_operands(id: &str) -> Vec<Expression> {
    let values = || variable("values", ValueType::Sequence);
    let scratch = || variable("scratch", ValueType::Sequence);
    let index = |name| variable(name, ValueType::Index);
    let at = |sequence: Expression, position: Expression| Expression::Index {
        sequence: Box::new(sequence),
        index: Box::new(position),
    };
    match id {
        "merge.allocate" => vec![scratch(), Expression::Length(Box::new(values()))],
        "merge.invoke" => vec![values()],
        "merge.recurse.left" => vec![Expression::Range {
            sequence: Box::new(values()),
            start: Box::new(Expression::Integer(0)),
            end: Box::new(index("middle")),
        }],
        "merge.recurse.right" => vec![Expression::Range {
            sequence: Box::new(values()),
            start: Box::new(index("middle")),
            end: Box::new(Expression::Length(Box::new(values()))),
        }],
        "merge.boundary.read" | "merge.boundary.compare" => vec![
            at(
                values(),
                binary(Bin::Subtract, index("middle"), Expression::Integer(1)),
            ),
            at(values(), index("middle")),
        ],
        "merge.select.read" | "merge.select.compare" => {
            vec![at(values(), index("left")), at(values(), index("right"))]
        }
        "merge.output.write" => vec![scratch(), variable("output", ValueType::Index)],
        "merge.copy" => vec![scratch(), values()],
        "partition.left.read" | "partition.left.predicate" => {
            vec![at(values(), index("left"))]
        }
        "partition.right.read" | "partition.right.predicate" => vec![at(
            values(),
            binary(Bin::Subtract, index("right"), Expression::Integer(1)),
        )],
        "partition.swap" => vec![
            at(values(), index("left")),
            at(
                values(),
                binary(Bin::Subtract, index("right"), Expression::Integer(1)),
            ),
        ],
        "partition.boundary" => vec![variable("boundary", ValueType::Index), index("left")],
        id if id.ends_with(".assert") => vec![Expression::Call {
            function: "invariant",
            arguments: vec![values()],
            result_type: ValueType::Bool,
        }],
        _ => vec![],
    }
}

pub fn merge_sort_ast() -> AlgorithmAst {
    use SemanticOperation as Op;
    AlgorithmAst {
        ast_version: "experimental-0",
        id: "ast.sort.merge.top_down.v0",
        algorithm_id: "sort.merge.top_down",
        parameters: vec![
            Parameter {
                name: "values",
                data_type: "MutableSequence<T>",
                value_type: ValueType::Sequence,
                mode: ParameterMode::ReadWrite,
            },
            Parameter {
                name: "scratch",
                data_type: "MutableSequence<T>",
                value_type: ValueType::Sequence,
                mode: ParameterMode::ReadWrite,
            },
            Parameter {
                name: "order",
                data_type: "TotalOrder<T>",
                value_type: ValueType::Comparator,
                mode: ParameterMode::Read,
            },
        ],
        effects: EffectSummary {
            mutates: vec!["values", "scratch"],
            allocations: vec!["caller allocates scratch[length(values)]"],
            copies: vec!["scratch range -> values range after merge"],
        },
        body: vec![
            operation(
                "merge.allocate",
                Op::Allocate,
                "caller provides scratch[length(values)]",
            ),
            operation(
                "merge.invoke",
                Op::Recurse,
                "enter sort for the current range",
            ),
            Statement::If {
                condition: e("length(values) < 2"),
                then_branch: vec![Statement::Return {
                    expression: e("values"),
                }],
                else_branch: vec![],
            },
            Statement::Let {
                name: "middle",
                expression: e("floor(length(values) / 2)"),
            },
            operation("merge.recurse.left", Op::Recurse, "sort values[0..middle]"),
            operation(
                "merge.recurse.right",
                Op::Recurse,
                "sort values[middle..length]",
            ),
            operation(
                "merge.boundary.read",
                Op::Read,
                "values[middle - 1], values[middle]",
            ),
            operation(
                "merge.boundary.compare",
                Op::Compare,
                "order(values[middle - 1], values[middle])",
            ),
            Statement::If {
                condition: e("left maximum <= right minimum"),
                then_branch: vec![
                    operation(
                        "merge.ordered.assert",
                        Op::Assert,
                        "adjacent sorted runs already ordered",
                    ),
                    Statement::Return {
                        expression: e("values"),
                    },
                ],
                else_branch: vec![],
            },
            Statement::Let {
                name: "left",
                expression: e("0"),
            },
            Statement::Let {
                name: "right",
                expression: e("middle"),
            },
            Statement::ForEach {
                binding: "output",
                collection: e("scratch[0..length(values)]"),
                body: vec![
                    operation(
                        "merge.select.read",
                        Op::Read,
                        "values[left], values[right] when present",
                    ),
                    operation(
                        "merge.select.compare",
                        Op::Compare,
                        "choose right only when strictly less",
                    ),
                    operation(
                        "merge.output.write",
                        Op::Write,
                        "scratch[output] = selected value",
                    ),
                ],
            },
            operation("merge.copy", Op::Copy, "copy scratch into values"),
            operation(
                "merge.result.assert",
                Op::Assert,
                "merged range is sorted and stable",
            ),
            operation(
                "merge.permutation.assert",
                Op::Assert,
                "result is a permutation of input",
            ),
            Statement::Return {
                expression: e("values"),
            },
        ],
    }
}

pub fn partition_ast() -> AlgorithmAst {
    use SemanticOperation as Op;
    AlgorithmAst {
        ast_version: "experimental-0",
        id: "ast.partition.two_pointer.in_place.v0",
        algorithm_id: "partition.two_pointer.in_place",
        parameters: vec![
            Parameter {
                name: "values",
                data_type: "MutableSequence<T>",
                value_type: ValueType::Sequence,
                mode: ParameterMode::ReadWrite,
            },
            Parameter {
                name: "predicate",
                data_type: "Predicate<T>",
                value_type: ValueType::Predicate,
                mode: ParameterMode::Read,
            },
            Parameter {
                name: "boundary",
                data_type: "Index<values>",
                value_type: ValueType::Index,
                mode: ParameterMode::Output,
            },
        ],
        effects: EffectSummary {
            mutates: vec!["values"],
            allocations: vec![],
            copies: vec![],
        },
        body: vec![
            Statement::Let {
                name: "left",
                expression: e("0"),
            },
            Statement::Let {
                name: "right",
                expression: e("length(values)"),
            },
            Statement::While {
                condition: e("left < right"),
                body: vec![
                    Statement::While {
                        condition: e("left < right"),
                        body: vec![
                            operation("partition.left.read", Op::Read, "values[left]"),
                            operation(
                                "partition.left.predicate",
                                Op::Predicate,
                                "predicate(values[left])",
                            ),
                            Statement::If {
                                condition: e("predicate result is true"),
                                then_branch: vec![Statement::Let {
                                    name: "left",
                                    expression: e("left + 1"),
                                }],
                                else_branch: vec![Statement::Break],
                            },
                        ],
                    },
                    Statement::While {
                        condition: e("left < right"),
                        body: vec![
                            operation("partition.right.read", Op::Read, "values[right - 1]"),
                            operation(
                                "partition.right.predicate",
                                Op::Predicate,
                                "predicate(values[right - 1])",
                            ),
                            Statement::If {
                                condition: e("predicate result is false"),
                                then_branch: vec![Statement::Let {
                                    name: "right",
                                    expression: e("right - 1"),
                                }],
                                else_branch: vec![Statement::Break],
                            },
                        ],
                    },
                    Statement::If {
                        condition: e("left < right"),
                        then_branch: vec![
                            operation(
                                "partition.swap",
                                Op::Swap,
                                "values[left] <-> values[right - 1]",
                            ),
                            Statement::Let {
                                name: "left",
                                expression: e("left + 1"),
                            },
                            Statement::Let {
                                name: "right",
                                expression: e("right - 1"),
                            },
                        ],
                        else_branch: vec![],
                    },
                ],
            },
            operation("partition.boundary", Op::Partition, "boundary = left"),
            operation(
                "partition.left.assert",
                Op::Assert,
                "all values before boundary match",
            ),
            operation(
                "partition.right.assert",
                Op::Assert,
                "all values after boundary are rejected",
            ),
            operation(
                "partition.permutation.assert",
                Op::Assert,
                "result is a permutation of input",
            ),
            Statement::Return {
                expression: e("boundary"),
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{SemanticOperation, Statement, merge_sort_ast, partition_ast};

    #[test]
    fn both_models_are_valid_and_render_deterministically() {
        for ast in [merge_sort_ast(), partition_ast()] {
            assert!(ast.validate().is_empty());
            assert_eq!(ast.render(), ast.render());
            assert!(ast.render().contains("effects mutate="));
        }
    }

    #[test]
    fn merge_model_makes_memory_and_stability_operations_explicit() {
        let ast = merge_sort_ast();
        let operations = ast.operation_kinds();

        for required in [
            SemanticOperation::Allocate,
            SemanticOperation::Read,
            SemanticOperation::Compare,
            SemanticOperation::Write,
            SemanticOperation::Copy,
            SemanticOperation::Recurse,
            SemanticOperation::Assert,
        ] {
            assert!(operations.contains(&required), "missing {required:?}");
        }
        assert!(
            ast.render()
                .contains("choose right only when strictly less")
        );
    }

    #[test]
    fn partition_model_has_nested_control_flow_and_no_hidden_allocation() {
        let ast = partition_ast();
        let operations = ast.operation_kinds();

        assert!(matches!(ast.body[2], Statement::While { .. }));
        assert!(operations.contains(&SemanticOperation::Swap));
        assert!(operations.contains(&SemanticOperation::Predicate));
        assert!(operations.contains(&SemanticOperation::Partition));
        assert!(!operations.contains(&SemanticOperation::Allocate));
        assert!(ast.effects.allocations.is_empty());
    }

    #[test]
    fn validation_rejects_writes_to_read_only_parameters() {
        let mut ast = partition_ast();
        ast.parameters[0].mode = super::ParameterMode::Read;

        let errors = ast.validate();

        assert!(
            errors
                .iter()
                .any(|error| error.contains("values") && error.contains("not writable"))
        );
    }
}

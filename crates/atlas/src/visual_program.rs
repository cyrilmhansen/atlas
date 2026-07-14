use serde::Serialize;

use crate::ast::{
    AlgorithmAst, SemanticOperation, insertion_sort_ast, is_sorted_ast, minimum_ast, partition_ast,
    reverse_ast,
};

pub const VISUAL_PROGRAM_FORMAT: &str = "atlas-visual-bytecode-private-v0";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VisualProgram {
    pub format: &'static str,
    pub algorithm_id: &'static str,
    pub ast_id: &'static str,
    pub registers: Vec<VisualRegister>,
    pub instructions: Vec<VisualInstruction>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VisualRegister {
    pub name: &'static str,
    pub initial: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum VisualInstruction {
    HaltIfEmpty,
    BranchIndexLessThanLength {
        register: usize,
        when_true: usize,
        when_false: usize,
    },
    BranchIndexLessThanIndex {
        left_register: usize,
        right_register: usize,
        when_true: usize,
        when_false: usize,
    },
    BranchPredicate {
        when_true: usize,
        when_false: usize,
    },
    BranchIfGreater {
        when_true: usize,
        when_false: usize,
    },
    BranchIfLess {
        when_true: usize,
        when_false: usize,
    },
    BranchRegisterNonZero {
        register: usize,
        when_true: usize,
        when_false: usize,
    },
    SetRegisterToLength {
        register: usize,
    },
    Read {
        node_id: &'static str,
        register: usize,
    },
    ReadPrevious {
        node_id: &'static str,
        register: usize,
    },
    PredicateEven {
        node_id: &'static str,
        register: usize,
        previous: bool,
    },
    CompareLess {
        node_id: &'static str,
        left_register: usize,
        right_register: usize,
    },
    CompareGreater {
        node_id: &'static str,
        left_register: usize,
        right_register: usize,
        left_previous: bool,
    },
    CompareLessPrevious {
        node_id: &'static str,
        register: usize,
    },
    CopyIfLess {
        target_register: usize,
        source_register: usize,
    },
    CopyRegister {
        target_register: usize,
        source_register: usize,
    },
    Increment {
        register: usize,
    },
    Decrement {
        register: usize,
    },
    SwapPrevious {
        node_id: &'static str,
        left_register: usize,
        right_register: usize,
    },
    SwapRegisters {
        node_id: &'static str,
        left_register: usize,
        right_register: usize,
    },
    Jump {
        target: usize,
    },
    ReturnOptionalIndex {
        register: usize,
    },
    ReturnNone,
    ReturnIndex {
        node_id: &'static str,
        register: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VisualProgramError(pub String);

impl std::fmt::Display for VisualProgramError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for VisualProgramError {}

pub fn compile_minimum_visual_program(
    ast: &AlgorithmAst,
) -> Result<VisualProgram, VisualProgramError> {
    let errors = ast.validate();
    if !errors.is_empty() {
        return Err(VisualProgramError(format!(
            "cannot compile invalid AST: {}",
            errors.join("; ")
        )));
    }
    if ast != &minimum_ast() {
        return Err(VisualProgramError(
            "minimum visual lowering accepts only the reviewed minimum AST shape".to_owned(),
        ));
    }

    let program = VisualProgram {
        format: VISUAL_PROGRAM_FORMAT,
        algorithm_id: ast.algorithm_id,
        ast_id: ast.id,
        registers: vec![
            VisualRegister {
                name: "minimum_index",
                initial: 0,
            },
            VisualRegister {
                name: "index",
                initial: 1,
            },
        ],
        instructions: vec![
            VisualInstruction::HaltIfEmpty,
            VisualInstruction::BranchIndexLessThanLength {
                register: 1,
                when_true: 2,
                when_false: 8,
            },
            VisualInstruction::Read {
                node_id: "minimum.candidate.read",
                register: 1,
            },
            VisualInstruction::Read {
                node_id: "minimum.best.read",
                register: 0,
            },
            VisualInstruction::CompareLess {
                node_id: "minimum.compare",
                left_register: 1,
                right_register: 0,
            },
            VisualInstruction::CopyIfLess {
                target_register: 0,
                source_register: 1,
            },
            VisualInstruction::Increment { register: 1 },
            VisualInstruction::Jump { target: 1 },
            VisualInstruction::ReturnOptionalIndex { register: 0 },
        ],
    };
    validate_visual_program(&program, ast)?;
    Ok(program)
}

pub fn compile_partition_even_visual_program(
    ast: &AlgorithmAst,
) -> Result<VisualProgram, VisualProgramError> {
    let errors = ast.validate();
    if !errors.is_empty() {
        return Err(VisualProgramError(format!(
            "cannot compile invalid AST: {}",
            errors.join("; ")
        )));
    }
    if ast != &partition_ast() {
        return Err(VisualProgramError(
            "even-partition visual lowering accepts only the reviewed partition AST shape"
                .to_owned(),
        ));
    }

    let program = VisualProgram {
        format: VISUAL_PROGRAM_FORMAT,
        algorithm_id: ast.algorithm_id,
        ast_id: ast.id,
        registers: vec![
            VisualRegister {
                name: "left",
                initial: 0,
            },
            VisualRegister {
                name: "right",
                initial: 0,
            },
        ],
        instructions: vec![
            VisualInstruction::SetRegisterToLength { register: 1 },
            VisualInstruction::BranchIndexLessThanIndex {
                left_register: 0,
                right_register: 1,
                when_true: 2,
                when_false: 18,
            },
            VisualInstruction::Read {
                node_id: "partition.left.read",
                register: 0,
            },
            VisualInstruction::PredicateEven {
                node_id: "partition.left.predicate",
                register: 0,
                previous: false,
            },
            VisualInstruction::BranchPredicate {
                when_true: 5,
                when_false: 7,
            },
            VisualInstruction::Increment { register: 0 },
            VisualInstruction::Jump { target: 1 },
            VisualInstruction::BranchIndexLessThanIndex {
                left_register: 0,
                right_register: 1,
                when_true: 8,
                when_false: 18,
            },
            VisualInstruction::ReadPrevious {
                node_id: "partition.right.read",
                register: 1,
            },
            VisualInstruction::PredicateEven {
                node_id: "partition.right.predicate",
                register: 1,
                previous: true,
            },
            VisualInstruction::BranchPredicate {
                when_true: 13,
                when_false: 11,
            },
            VisualInstruction::Decrement { register: 1 },
            VisualInstruction::Jump { target: 7 },
            VisualInstruction::BranchIndexLessThanIndex {
                left_register: 0,
                right_register: 1,
                when_true: 14,
                when_false: 18,
            },
            VisualInstruction::SwapPrevious {
                node_id: "partition.swap",
                left_register: 0,
                right_register: 1,
            },
            VisualInstruction::Increment { register: 0 },
            VisualInstruction::Decrement { register: 1 },
            VisualInstruction::Jump { target: 1 },
            VisualInstruction::ReturnIndex {
                node_id: "partition.boundary",
                register: 0,
            },
        ],
    };
    validate_visual_program(&program, ast)?;
    Ok(program)
}

pub fn compile_is_sorted_visual_program(
    ast: &AlgorithmAst,
) -> Result<VisualProgram, VisualProgramError> {
    let errors = ast.validate();
    if !errors.is_empty() {
        return Err(VisualProgramError(format!(
            "cannot compile invalid AST: {}",
            errors.join("; ")
        )));
    }
    if ast != &is_sorted_ast() {
        return Err(VisualProgramError(
            "is-sorted visual lowering accepts only the reviewed adjacent-scan AST shape"
                .to_owned(),
        ));
    }

    let program = VisualProgram {
        format: VISUAL_PROGRAM_FORMAT,
        algorithm_id: ast.algorithm_id,
        ast_id: ast.id,
        registers: vec![VisualRegister {
            name: "index",
            initial: 1,
        }],
        instructions: vec![
            VisualInstruction::BranchIndexLessThanLength {
                register: 0,
                when_true: 1,
                when_false: 7,
            },
            VisualInstruction::ReadPrevious {
                node_id: "is-sorted.left.read",
                register: 0,
            },
            VisualInstruction::Read {
                node_id: "is-sorted.right.read",
                register: 0,
            },
            VisualInstruction::CompareGreater {
                node_id: "is-sorted.adjacent.compare",
                left_register: 0,
                right_register: 0,
                left_previous: true,
            },
            VisualInstruction::BranchIfGreater {
                when_true: 8,
                when_false: 5,
            },
            VisualInstruction::Increment { register: 0 },
            VisualInstruction::Jump { target: 0 },
            VisualInstruction::ReturnNone,
            VisualInstruction::ReturnOptionalIndex { register: 0 },
        ],
    };
    validate_visual_program(&program, ast)?;
    Ok(program)
}

pub fn compile_insertion_visual_program(
    ast: &AlgorithmAst,
) -> Result<VisualProgram, VisualProgramError> {
    let errors = ast.validate();
    if !errors.is_empty() {
        return Err(VisualProgramError(format!(
            "cannot compile invalid AST: {}",
            errors.join("; ")
        )));
    }
    if ast != &insertion_sort_ast() {
        return Err(VisualProgramError(
            "insertion visual lowering accepts only the reviewed stable insertion AST shape"
                .to_owned(),
        ));
    }

    let program = VisualProgram {
        format: VISUAL_PROGRAM_FORMAT,
        algorithm_id: ast.algorithm_id,
        ast_id: ast.id,
        registers: vec![
            VisualRegister {
                name: "index",
                initial: 1,
            },
            VisualRegister {
                name: "current",
                initial: 1,
            },
        ],
        instructions: vec![
            VisualInstruction::BranchIndexLessThanLength {
                register: 0,
                when_true: 1,
                when_false: 12,
            },
            VisualInstruction::CopyRegister {
                target_register: 1,
                source_register: 0,
            },
            VisualInstruction::BranchRegisterNonZero {
                register: 1,
                when_true: 3,
                when_false: 10,
            },
            VisualInstruction::Read {
                node_id: "insertion.current.read",
                register: 1,
            },
            VisualInstruction::ReadPrevious {
                node_id: "insertion.previous.read",
                register: 1,
            },
            VisualInstruction::CompareLessPrevious {
                node_id: "insertion.adjacent.compare",
                register: 1,
            },
            VisualInstruction::BranchIfLess {
                when_true: 7,
                when_false: 10,
            },
            VisualInstruction::SwapPrevious {
                node_id: "insertion.adjacent.swap",
                left_register: 1,
                right_register: 1,
            },
            VisualInstruction::Decrement { register: 1 },
            VisualInstruction::Jump { target: 2 },
            VisualInstruction::Increment { register: 0 },
            VisualInstruction::Jump { target: 0 },
            VisualInstruction::ReturnNone,
        ],
    };
    validate_visual_program(&program, ast)?;
    Ok(program)
}

pub fn compile_reverse_visual_program(
    ast: &AlgorithmAst,
) -> Result<VisualProgram, VisualProgramError> {
    let errors = ast.validate();
    if !errors.is_empty() {
        return Err(VisualProgramError(format!(
            "cannot compile invalid AST: {}",
            errors.join("; ")
        )));
    }
    if ast != &reverse_ast() {
        return Err(VisualProgramError(
            "reverse visual lowering accepts only the reviewed symmetric reverse AST shape"
                .to_owned(),
        ));
    }

    let program = VisualProgram {
        format: VISUAL_PROGRAM_FORMAT,
        algorithm_id: ast.algorithm_id,
        ast_id: ast.id,
        registers: vec![
            VisualRegister {
                name: "left",
                initial: 0,
            },
            VisualRegister {
                name: "right",
                initial: 0,
            },
        ],
        instructions: vec![
            VisualInstruction::HaltIfEmpty,
            VisualInstruction::SetRegisterToLength { register: 1 },
            VisualInstruction::Decrement { register: 1 },
            VisualInstruction::BranchIndexLessThanIndex {
                left_register: 0,
                right_register: 1,
                when_true: 4,
                when_false: 10,
            },
            VisualInstruction::Read {
                node_id: "reverse.left.read",
                register: 0,
            },
            VisualInstruction::Read {
                node_id: "reverse.right.read",
                register: 1,
            },
            VisualInstruction::SwapRegisters {
                node_id: "reverse.symmetric.swap",
                left_register: 0,
                right_register: 1,
            },
            VisualInstruction::Increment { register: 0 },
            VisualInstruction::Decrement { register: 1 },
            VisualInstruction::Jump { target: 3 },
            VisualInstruction::ReturnNone,
        ],
    };
    validate_visual_program(&program, ast)?;
    Ok(program)
}

pub fn validate_visual_program(
    program: &VisualProgram,
    ast: &AlgorithmAst,
) -> Result<(), VisualProgramError> {
    if program.format != VISUAL_PROGRAM_FORMAT {
        return Err(VisualProgramError(
            "unknown visual program format".to_owned(),
        ));
    }
    if program.algorithm_id != ast.algorithm_id || program.ast_id != ast.id {
        return Err(VisualProgramError(
            "visual program identity does not match its AST".to_owned(),
        ));
    }
    if program.registers.is_empty() || program.registers.len() > 16 {
        return Err(VisualProgramError(
            "visual program must declare between 1 and 16 registers".to_owned(),
        ));
    }
    if program.instructions.is_empty() || program.instructions.len() > 256 {
        return Err(VisualProgramError(
            "visual program must contain between 1 and 256 instructions".to_owned(),
        ));
    }

    let register_count = program.registers.len();
    let instruction_count = program.instructions.len();
    let register = |index: usize| {
        (index < register_count).then_some(()).ok_or_else(|| {
            VisualProgramError(format!("instruction references unknown register {index}"))
        })
    };
    let target = |index: usize| {
        (index < instruction_count).then_some(()).ok_or_else(|| {
            VisualProgramError(format!("instruction references unknown target {index}"))
        })
    };

    for instruction in &program.instructions {
        match instruction {
            VisualInstruction::HaltIfEmpty => {}
            VisualInstruction::BranchIndexLessThanLength {
                register: source,
                when_true,
                when_false,
            } => {
                register(*source)?;
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstruction::BranchIndexLessThanIndex {
                left_register,
                right_register,
                when_true,
                when_false,
            } => {
                register(*left_register)?;
                register(*right_register)?;
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstruction::BranchPredicate {
                when_true,
                when_false,
            } => {
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstruction::BranchIfGreater {
                when_true,
                when_false,
            } => {
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstruction::BranchIfLess {
                when_true,
                when_false,
            } => {
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstruction::BranchRegisterNonZero {
                register: source,
                when_true,
                when_false,
            } => {
                register(*source)?;
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstruction::SetRegisterToLength {
                register: destination,
            } => register(*destination)?,
            VisualInstruction::Read {
                node_id,
                register: source,
            } => {
                register(*source)?;
                validate_node(ast, node_id, SemanticOperation::Read)?;
            }
            VisualInstruction::ReadPrevious {
                node_id,
                register: source,
            } => {
                register(*source)?;
                validate_node(ast, node_id, SemanticOperation::Read)?;
            }
            VisualInstruction::PredicateEven {
                node_id,
                register: source,
                ..
            } => {
                register(*source)?;
                validate_node(ast, node_id, SemanticOperation::Predicate)?;
            }
            VisualInstruction::CompareLess {
                node_id,
                left_register,
                right_register,
            } => {
                register(*left_register)?;
                register(*right_register)?;
                validate_node(ast, node_id, SemanticOperation::Compare)?;
            }
            VisualInstruction::CompareGreater {
                node_id,
                left_register,
                right_register,
                ..
            } => {
                register(*left_register)?;
                register(*right_register)?;
                validate_node(ast, node_id, SemanticOperation::Compare)?;
            }
            VisualInstruction::CompareLessPrevious {
                node_id,
                register: source,
            } => {
                register(*source)?;
                validate_node(ast, node_id, SemanticOperation::Compare)?;
            }
            VisualInstruction::CopyIfLess {
                target_register,
                source_register,
            } => {
                register(*target_register)?;
                register(*source_register)?;
            }
            VisualInstruction::CopyRegister {
                target_register,
                source_register,
            } => {
                register(*target_register)?;
                register(*source_register)?;
            }
            VisualInstruction::Increment {
                register: target_register,
            }
            | VisualInstruction::Decrement {
                register: target_register,
            }
            | VisualInstruction::ReturnOptionalIndex {
                register: target_register,
            } => {
                register(*target_register)?;
            }
            VisualInstruction::SwapPrevious {
                node_id,
                left_register,
                right_register,
            }
            | VisualInstruction::SwapRegisters {
                node_id,
                left_register,
                right_register,
            } => {
                register(*left_register)?;
                register(*right_register)?;
                validate_node(ast, node_id, SemanticOperation::Swap)?;
            }
            VisualInstruction::ReturnIndex {
                node_id,
                register: source,
            } => {
                register(*source)?;
                validate_node(ast, node_id, SemanticOperation::Partition)?;
            }
            VisualInstruction::ReturnNone => {}
            VisualInstruction::Jump {
                target: jump_target,
            } => target(*jump_target)?,
        }
    }
    Ok(())
}

fn validate_node(
    ast: &AlgorithmAst,
    node_id: &str,
    expected: SemanticOperation,
) -> Result<(), VisualProgramError> {
    match ast.operation_by_id(node_id) {
        Some(actual) if actual == expected => Ok(()),
        Some(actual) => Err(VisualProgramError(format!(
            "AST node {node_id:?} is {actual:?}, not {expected:?}"
        ))),
        None => Err(VisualProgramError(format!(
            "AST node {node_id:?} does not exist"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        Statement, insertion_sort_ast, is_sorted_ast, minimum_ast, partition_ast, reverse_ast,
    };

    use super::{
        VisualInstruction, compile_insertion_visual_program, compile_is_sorted_visual_program,
        compile_minimum_visual_program, compile_partition_even_visual_program,
        compile_reverse_visual_program, validate_visual_program,
    };

    #[test]
    fn minimum_lowering_is_deterministic_and_keeps_exact_ast_nodes() {
        let ast = minimum_ast();
        let first = compile_minimum_visual_program(&ast).unwrap();
        let second = compile_minimum_visual_program(&ast).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.registers.len(), 2);
        assert_eq!(first.instructions.len(), 9);
        assert!(first.instructions.iter().any(|instruction| matches!(
            instruction,
            VisualInstruction::CompareLess {
                node_id: "minimum.compare",
                ..
            }
        )));
        validate_visual_program(&first, &ast).unwrap();
    }

    #[test]
    fn minimum_lowering_rejects_an_unreviewed_ast_shape() {
        let mut ast = minimum_ast();
        ast.body.push(Statement::Break);

        let error = compile_minimum_visual_program(&ast).unwrap_err();

        assert!(error.0.contains("reviewed minimum AST shape"));
    }

    #[test]
    fn even_partition_lowering_is_deterministic_and_ast_linked() {
        let ast = partition_ast();
        let first = compile_partition_even_visual_program(&ast).unwrap();
        let second = compile_partition_even_visual_program(&ast).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.registers.len(), 2);
        assert_eq!(first.instructions.len(), 19);
        assert!(first.instructions.iter().any(|instruction| matches!(
            instruction,
            VisualInstruction::SwapPrevious {
                node_id: "partition.swap",
                ..
            }
        )));
        validate_visual_program(&first, &ast).unwrap();
    }

    #[test]
    fn is_sorted_lowering_is_deterministic_and_ast_linked() {
        let ast = is_sorted_ast();
        let first = compile_is_sorted_visual_program(&ast).unwrap();
        let second = compile_is_sorted_visual_program(&ast).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.registers.len(), 1);
        assert_eq!(first.instructions.len(), 9);
        assert!(first.instructions.iter().any(|instruction| matches!(
            instruction,
            VisualInstruction::CompareGreater {
                node_id: "is-sorted.adjacent.compare",
                ..
            }
        )));
        validate_visual_program(&first, &ast).unwrap();
    }

    #[test]
    fn insertion_lowering_is_deterministic_and_ast_linked() {
        let ast = insertion_sort_ast();
        let first = compile_insertion_visual_program(&ast).unwrap();
        let second = compile_insertion_visual_program(&ast).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.registers.len(), 2);
        assert_eq!(first.instructions.len(), 13);
        assert!(first.instructions.iter().any(|instruction| matches!(
            instruction,
            VisualInstruction::SwapPrevious {
                node_id: "insertion.adjacent.swap",
                ..
            }
        )));
        validate_visual_program(&first, &ast).unwrap();
    }

    #[test]
    fn reverse_lowering_is_deterministic_and_ast_linked() {
        let ast = reverse_ast();
        let first = compile_reverse_visual_program(&ast).unwrap();
        let second = compile_reverse_visual_program(&ast).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.registers.len(), 2);
        assert_eq!(first.instructions.len(), 11);
        assert!(first.instructions.iter().any(|instruction| matches!(
            instruction,
            VisualInstruction::SwapRegisters {
                node_id: "reverse.symmetric.swap",
                ..
            }
        )));
        validate_visual_program(&first, &ast).unwrap();
    }

    #[test]
    fn validation_rejects_unknown_registers_targets_and_ast_nodes() {
        let ast = minimum_ast();
        let mut program = compile_minimum_visual_program(&ast).unwrap();
        program.instructions[2] = VisualInstruction::Read {
            node_id: "minimum.missing",
            register: 99,
        };

        let error = validate_visual_program(&program, &ast).unwrap_err();

        assert!(error.0.contains("unknown register"));
    }
}

use core::cmp::Ordering;

use atlas_algorithms::insertion_sort::insertion_sort_by;
use atlas_algorithms::is_sorted::is_sorted_by;
use atlas_algorithms::reverse::reverse_in_place;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

pub const MAX_INPUT_LENGTH: usize = 4_096;
pub const MAX_TRACE_INPUT_LENGTH: usize = 64;
pub const MAX_INSERTION_TRACE_INPUT_LENGTH: usize = 32;
pub const MAX_INSERTION_STEPPER_INPUT_LENGTH: usize = 64;

const LEFT_READ_NODE: &str = "is-sorted.left.read";
const RIGHT_READ_NODE: &str = "is-sorted.right.read";
const COMPARE_NODE: &str = "is-sorted.adjacent.compare";
const INSERTION_CURRENT_READ_NODE: &str = "insertion.current.read";
const INSERTION_PREVIOUS_READ_NODE: &str = "insertion.previous.read";
const INSERTION_COMPARE_NODE: &str = "insertion.adjacent.compare";
const INSERTION_SWAP_NODE: &str = "insertion.adjacent.swap";
const REVERSE_LEFT_READ_NODE: &str = "reverse.left.read";
const REVERSE_RIGHT_READ_NODE: &str = "reverse.right.read";
const REVERSE_SWAP_NODE: &str = "reverse.symmetric.swap";
const VISUAL_PROGRAM_FORMAT: &str = "atlas-visual-bytecode-private-v0";
const MAX_VISUAL_PROGRAM_LENGTH: usize = 32 * 1024;
const MAX_VISUAL_INSTRUCTIONS: usize = 256;
const MAX_VISUAL_REGISTERS: usize = 16;

#[derive(Clone, Debug, Deserialize)]
struct VisualProgramSpec {
    format: String,
    algorithm_id: String,
    ast_id: String,
    registers: Vec<VisualRegisterSpec>,
    instructions: Vec<VisualInstructionSpec>,
}

#[derive(Clone, Debug, Deserialize)]
struct VisualRegisterSpec {
    name: String,
    initial: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum VisualInstructionSpec {
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
        node_id: String,
        register: usize,
    },
    ReadPrevious {
        node_id: String,
        register: usize,
    },
    PredicateEven {
        node_id: String,
        register: usize,
        previous: bool,
    },
    CompareLess {
        node_id: String,
        left_register: usize,
        right_register: usize,
    },
    CompareGreater {
        node_id: String,
        left_register: usize,
        right_register: usize,
        left_previous: bool,
    },
    CompareLessPrevious {
        node_id: String,
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
        node_id: String,
        left_register: usize,
        right_register: usize,
    },
    SwapRegisters {
        node_id: String,
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
        node_id: String,
        register: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct VisualOperation {
    node_id: String,
    kind: &'static str,
    left_index: usize,
    right_index: Option<usize>,
    ordering: Option<i8>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VisualMachine {
    program: VisualProgramSpec,
    values: Vec<i32>,
    original_indices: Vec<u32>,
    initial_registers: Vec<usize>,
    registers: Vec<usize>,
    pc: usize,
    comparison_less: bool,
    comparison_greater: bool,
    predicate_result: bool,
    result_index: Option<usize>,
    done: bool,
    steps: u32,
    comparisons: u32,
    predicate_evaluations: u32,
    reads: u32,
    writes: u32,
    swaps: u32,
    operation: Option<VisualOperation>,
}

#[wasm_bindgen]
impl VisualMachine {
    #[wasm_bindgen(constructor)]
    pub fn new(program_json: &str, values: &[i32]) -> Result<VisualMachine, JsError> {
        Self::from_json(program_json, values).map_err(|error| JsError::new(&error))
    }

    pub fn reset(&mut self, values: &[i32]) -> Result<(), JsError> {
        self.reset_values(values)
            .map_err(|error| JsError::new(&error))
    }

    pub fn step(&mut self) -> Result<bool, JsError> {
        self.step_checked().map_err(|error| JsError::new(&error))
    }

    #[wasm_bindgen(getter)]
    pub fn algorithm_id(&self) -> String {
        self.program.algorithm_id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn ast_id(&self) -> String {
        self.program.ast_id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Box<[i32]> {
        self.values.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn original_indices(&self) -> Box<[u32]> {
        if self.original_indices.is_empty() {
            (0..self.values.len() as u32)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        } else {
            self.original_indices.clone().into_boxed_slice()
        }
    }

    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.done
    }

    #[wasm_bindgen(getter)]
    pub fn steps(&self) -> u32 {
        self.steps
    }

    #[wasm_bindgen(getter)]
    pub fn comparisons(&self) -> u32 {
        self.comparisons
    }

    #[wasm_bindgen(getter)]
    pub fn predicate_evaluations(&self) -> u32 {
        self.predicate_evaluations
    }

    #[wasm_bindgen(getter)]
    pub fn reads(&self) -> u32 {
        self.reads
    }

    #[wasm_bindgen(getter)]
    pub fn writes(&self) -> u32 {
        self.writes
    }

    #[wasm_bindgen(getter)]
    pub fn predicate_result(&self) -> bool {
        self.predicate_result
    }

    #[wasm_bindgen(getter)]
    pub fn swaps(&self) -> u32 {
        self.swaps
    }

    #[wasm_bindgen(getter)]
    pub fn has_result(&self) -> bool {
        self.result_index.is_some()
    }

    #[wasm_bindgen(getter)]
    pub fn result_index(&self) -> Option<u32> {
        self.result_index.map(|index| index as u32)
    }

    #[wasm_bindgen(getter)]
    pub fn result_value(&self) -> Option<i32> {
        self.result_index
            .and_then(|index| self.values.get(index))
            .copied()
    }

    pub fn register_value(&self, name: &str) -> Option<u32> {
        self.program
            .registers
            .iter()
            .position(|register| register.name == name)
            .map(|index| self.registers[index] as u32)
    }

    #[wasm_bindgen(getter)]
    pub fn operation_node_id(&self) -> Option<String> {
        self.operation
            .as_ref()
            .map(|operation| operation.node_id.clone())
    }

    #[wasm_bindgen(getter)]
    pub fn operation_kind(&self) -> Option<String> {
        self.operation
            .as_ref()
            .map(|operation| operation.kind.to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn operation_left_index(&self) -> Option<u32> {
        self.operation
            .as_ref()
            .map(|operation| operation.left_index as u32)
    }

    #[wasm_bindgen(getter)]
    pub fn operation_right_index(&self) -> Option<u32> {
        self.operation
            .as_ref()
            .and_then(|operation| operation.right_index)
            .map(|index| index as u32)
    }

    #[wasm_bindgen(getter)]
    pub fn operation_ordering(&self) -> Option<i8> {
        self.operation
            .as_ref()
            .and_then(|operation| operation.ordering)
    }
}

impl VisualMachine {
    fn from_json(program_json: &str, values: &[i32]) -> Result<Self, String> {
        if program_json.len() > MAX_VISUAL_PROGRAM_LENGTH {
            return Err("visual program exceeds the private 32 KiB limit".to_owned());
        }
        if values.len() > MAX_INPUT_LENGTH {
            return Err(format!(
                "visual machine input length {} exceeds the Atlas browser limit of {MAX_INPUT_LENGTH}",
                values.len()
            ));
        }
        let program: VisualProgramSpec = serde_json::from_str(program_json)
            .map_err(|error| format!("invalid visual program JSON: {error}"))?;
        validate_visual_program_spec(&program)?;
        let initial_registers = program
            .registers
            .iter()
            .map(|register| register.initial)
            .collect::<Vec<_>>();
        let mut machine = Self {
            program,
            values: values.to_vec(),
            original_indices: Vec::new(),
            registers: initial_registers.clone(),
            initial_registers,
            pc: 0,
            comparison_less: false,
            comparison_greater: false,
            predicate_result: false,
            result_index: None,
            done: false,
            steps: 0,
            comparisons: 0,
            predicate_evaluations: 0,
            reads: 0,
            writes: 0,
            swaps: 0,
            operation: None,
        };
        machine.settle_control()?;
        Ok(machine)
    }

    fn reset_values(&mut self, values: &[i32]) -> Result<(), String> {
        if values.len() > MAX_INPUT_LENGTH {
            return Err(format!(
                "visual machine input length {} exceeds the Atlas browser limit of {MAX_INPUT_LENGTH}",
                values.len()
            ));
        }
        self.values.clear();
        self.values.extend_from_slice(values);
        self.original_indices.clear();
        self.registers.clone_from(&self.initial_registers);
        self.pc = 0;
        self.comparison_less = false;
        self.comparison_greater = false;
        self.predicate_result = false;
        self.result_index = None;
        self.done = false;
        self.steps = 0;
        self.comparisons = 0;
        self.predicate_evaluations = 0;
        self.reads = 0;
        self.writes = 0;
        self.swaps = 0;
        self.operation = None;
        self.settle_control()
    }

    fn step_checked(&mut self) -> Result<bool, String> {
        if self.done {
            return Ok(false);
        }
        let execution_budget = MAX_VISUAL_INSTRUCTIONS
            .checked_mul(self.values.len().max(1))
            .ok_or_else(|| "visual execution budget overflow".to_owned())?;
        if self.steps as usize >= execution_budget {
            return Err(format!(
                "visual program exceeded its semantic-step budget of {execution_budget}"
            ));
        }
        self.operation = None;
        self.settle_control()?;
        if self.done {
            return Ok(false);
        }
        let instruction = self.program.instructions[self.pc].clone();
        match instruction {
            VisualInstructionSpec::Read { node_id, register } => {
                let index = self.read_register(register)?;
                self.value(index)?;
                self.reads += 1;
                self.operation = Some(VisualOperation {
                    node_id,
                    kind: "Read",
                    left_index: index,
                    right_index: None,
                    ordering: None,
                });
                self.pc += 1;
            }
            VisualInstructionSpec::ReadPrevious { node_id, register } => {
                let index = self.previous_index(register)?;
                self.value(index)?;
                self.reads += 1;
                self.operation = Some(VisualOperation {
                    node_id,
                    kind: "Read",
                    left_index: index,
                    right_index: None,
                    ordering: None,
                });
                self.pc += 1;
            }
            VisualInstructionSpec::PredicateEven {
                node_id,
                register,
                previous,
            } => {
                let index = if previous {
                    self.previous_index(register)?
                } else {
                    self.read_register(register)?
                };
                self.predicate_result = self.value(index)? % 2 == 0;
                self.predicate_evaluations += 1;
                self.operation = Some(VisualOperation {
                    node_id,
                    kind: "Predicate",
                    left_index: index,
                    right_index: None,
                    ordering: None,
                });
                self.pc += 1;
            }
            VisualInstructionSpec::CompareLess {
                node_id,
                left_register,
                right_register,
            } => {
                let left_index = self.read_register(left_register)?;
                let right_index = self.read_register(right_register)?;
                let ordering = self.value(left_index)?.cmp(self.value(right_index)?);
                self.comparison_less = ordering == Ordering::Less;
                self.comparisons += 1;
                self.operation = Some(VisualOperation {
                    node_id,
                    kind: "Compare",
                    left_index,
                    right_index: Some(right_index),
                    ordering: Some(ordering_value(ordering)),
                });
                self.pc += 1;
            }
            VisualInstructionSpec::CompareGreater {
                node_id,
                left_register,
                right_register,
                left_previous,
            } => {
                let left_index = if left_previous {
                    self.previous_index(left_register)?
                } else {
                    self.read_register(left_register)?
                };
                let right_index = self.read_register(right_register)?;
                let ordering = self.value(left_index)?.cmp(self.value(right_index)?);
                self.comparison_greater = ordering == Ordering::Greater;
                self.comparisons += 1;
                self.operation = Some(VisualOperation {
                    node_id,
                    kind: "Compare",
                    left_index,
                    right_index: Some(right_index),
                    ordering: Some(ordering_value(ordering)),
                });
                self.pc += 1;
            }
            VisualInstructionSpec::CompareLessPrevious { node_id, register } => {
                let left_index = self.read_register(register)?;
                let right_index = self.previous_index(register)?;
                let ordering = self.value(left_index)?.cmp(self.value(right_index)?);
                self.comparison_less = ordering == Ordering::Less;
                self.comparisons += 1;
                self.operation = Some(VisualOperation {
                    node_id,
                    kind: "Compare",
                    left_index,
                    right_index: Some(right_index),
                    ordering: Some(ordering_value(ordering)),
                });
                self.pc += 1;
            }
            VisualInstructionSpec::SwapPrevious {
                node_id,
                left_register,
                right_register,
            } => {
                let left_index = self.read_register(left_register)?;
                let right_index = self.previous_index(right_register)?;
                self.value(left_index)?;
                self.value(right_index)?;
                if self.original_indices.is_empty() {
                    self.original_indices = (0..self.values.len() as u32).collect();
                }
                self.values.swap(left_index, right_index);
                self.original_indices.swap(left_index, right_index);
                self.writes += 2;
                self.swaps += 1;
                self.operation = Some(VisualOperation {
                    node_id,
                    kind: "Swap",
                    left_index,
                    right_index: Some(right_index),
                    ordering: None,
                });
                self.pc += 1;
            }
            VisualInstructionSpec::SwapRegisters {
                node_id,
                left_register,
                right_register,
            } => {
                let left_index = self.read_register(left_register)?;
                let right_index = self.read_register(right_register)?;
                self.value(left_index)?;
                self.value(right_index)?;
                if self.original_indices.is_empty() {
                    self.original_indices = (0..self.values.len() as u32).collect();
                }
                self.values.swap(left_index, right_index);
                self.original_indices.swap(left_index, right_index);
                self.writes += 2;
                self.swaps += 1;
                self.operation = Some(VisualOperation {
                    node_id,
                    kind: "Swap",
                    left_index,
                    right_index: Some(right_index),
                    ordering: None,
                });
                self.pc += 1;
            }
            VisualInstructionSpec::ReturnIndex { node_id, register } => {
                let index = self.read_register(register)?;
                if index > self.values.len() {
                    return Err(format!(
                        "visual program returns out-of-bounds boundary {index}"
                    ));
                }
                self.result_index = Some(index);
                self.done = true;
                self.operation = Some(VisualOperation {
                    node_id,
                    kind: "Partition",
                    left_index: index,
                    right_index: None,
                    ordering: None,
                });
            }
            _ => return Err("visual machine did not settle on a semantic instruction".to_owned()),
        }
        self.steps += 1;
        self.settle_control()?;
        Ok(true)
    }

    fn settle_control(&mut self) -> Result<(), String> {
        for _ in 0..MAX_VISUAL_INSTRUCTIONS {
            if self.done {
                return Ok(());
            }
            let instruction = self
                .program
                .instructions
                .get(self.pc)
                .ok_or_else(|| format!("visual program counter {} is out of bounds", self.pc))?
                .clone();
            match instruction {
                VisualInstructionSpec::Read { .. }
                | VisualInstructionSpec::ReadPrevious { .. }
                | VisualInstructionSpec::PredicateEven { .. }
                | VisualInstructionSpec::CompareLess { .. }
                | VisualInstructionSpec::CompareGreater { .. }
                | VisualInstructionSpec::CompareLessPrevious { .. }
                | VisualInstructionSpec::SwapPrevious { .. }
                | VisualInstructionSpec::SwapRegisters { .. }
                | VisualInstructionSpec::ReturnIndex { .. } => {
                    return Ok(());
                }
                VisualInstructionSpec::HaltIfEmpty => {
                    self.pc += 1;
                    if self.values.is_empty() {
                        self.done = true;
                        self.result_index = None;
                    }
                }
                VisualInstructionSpec::BranchIndexLessThanLength {
                    register,
                    when_true,
                    when_false,
                } => {
                    self.pc = if self.read_register(register)? < self.values.len() {
                        when_true
                    } else {
                        when_false
                    };
                }
                VisualInstructionSpec::BranchIndexLessThanIndex {
                    left_register,
                    right_register,
                    when_true,
                    when_false,
                } => {
                    self.pc = if self.read_register(left_register)?
                        < self.read_register(right_register)?
                    {
                        when_true
                    } else {
                        when_false
                    };
                }
                VisualInstructionSpec::BranchPredicate {
                    when_true,
                    when_false,
                } => {
                    self.pc = if self.predicate_result {
                        when_true
                    } else {
                        when_false
                    };
                }
                VisualInstructionSpec::BranchIfGreater {
                    when_true,
                    when_false,
                } => {
                    self.pc = if self.comparison_greater {
                        when_true
                    } else {
                        when_false
                    };
                }
                VisualInstructionSpec::BranchIfLess {
                    when_true,
                    when_false,
                } => {
                    self.pc = if self.comparison_less {
                        when_true
                    } else {
                        when_false
                    };
                }
                VisualInstructionSpec::BranchRegisterNonZero {
                    register,
                    when_true,
                    when_false,
                } => {
                    self.pc = if self.read_register(register)? != 0 {
                        when_true
                    } else {
                        when_false
                    };
                }
                VisualInstructionSpec::SetRegisterToLength { register } => {
                    self.registers[register] = self.values.len();
                    self.pc += 1;
                }
                VisualInstructionSpec::CopyIfLess {
                    target_register,
                    source_register,
                } => {
                    if self.comparison_less {
                        self.registers[target_register] = self.read_register(source_register)?;
                    }
                    self.pc += 1;
                }
                VisualInstructionSpec::CopyRegister {
                    target_register,
                    source_register,
                } => {
                    self.registers[target_register] = self.read_register(source_register)?;
                    self.pc += 1;
                }
                VisualInstructionSpec::Increment { register } => {
                    self.registers[register] = self.registers[register]
                        .checked_add(1)
                        .ok_or_else(|| "visual register overflow".to_owned())?;
                    self.pc += 1;
                }
                VisualInstructionSpec::Decrement { register } => {
                    self.registers[register] = self.registers[register]
                        .checked_sub(1)
                        .ok_or_else(|| "visual register underflow".to_owned())?;
                    self.pc += 1;
                }
                VisualInstructionSpec::Jump { target } => self.pc = target,
                VisualInstructionSpec::ReturnOptionalIndex { register } => {
                    let index = self.read_register(register)?;
                    self.value(index)?;
                    self.result_index = Some(index);
                    self.done = true;
                }
                VisualInstructionSpec::ReturnNone => {
                    self.result_index = None;
                    self.done = true;
                }
            }
        }
        Err("visual program exceeded its control-step budget".to_owned())
    }

    fn read_register(&self, register: usize) -> Result<usize, String> {
        self.registers
            .get(register)
            .copied()
            .ok_or_else(|| format!("visual program references unknown register {register}"))
    }

    fn value(&self, index: usize) -> Result<&i32, String> {
        self.values
            .get(index)
            .ok_or_else(|| format!("visual program reads out-of-bounds index {index}"))
    }

    fn previous_index(&self, register: usize) -> Result<usize, String> {
        self.read_register(register)?
            .checked_sub(1)
            .ok_or_else(|| "visual program index underflow".to_owned())
    }
}

fn validate_visual_program_spec(program: &VisualProgramSpec) -> Result<(), String> {
    if program.format != VISUAL_PROGRAM_FORMAT {
        return Err(format!(
            "unsupported visual program format {:?}",
            program.format
        ));
    }
    if program.algorithm_id.is_empty() || program.ast_id.is_empty() {
        return Err("visual program identity must not be empty".to_owned());
    }
    if program.registers.is_empty() || program.registers.len() > MAX_VISUAL_REGISTERS {
        return Err(format!(
            "visual program must declare between 1 and {MAX_VISUAL_REGISTERS} registers"
        ));
    }
    if program.instructions.is_empty() || program.instructions.len() > MAX_VISUAL_INSTRUCTIONS {
        return Err(format!(
            "visual program must contain between 1 and {MAX_VISUAL_INSTRUCTIONS} instructions"
        ));
    }
    let register_count = program.registers.len();
    let instruction_count = program.instructions.len();
    let register = |index: usize| {
        (index < register_count)
            .then_some(())
            .ok_or_else(|| format!("visual program references unknown register {index}"))
    };
    let target = |index: usize| {
        (index < instruction_count)
            .then_some(())
            .ok_or_else(|| format!("visual program references unknown target {index}"))
    };
    for instruction in &program.instructions {
        match instruction {
            VisualInstructionSpec::HaltIfEmpty => {}
            VisualInstructionSpec::BranchIndexLessThanLength {
                register: source,
                when_true,
                when_false,
            } => {
                register(*source)?;
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstructionSpec::BranchIndexLessThanIndex {
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
            VisualInstructionSpec::BranchPredicate {
                when_true,
                when_false,
            } => {
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstructionSpec::BranchIfGreater {
                when_true,
                when_false,
            } => {
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstructionSpec::BranchIfLess {
                when_true,
                when_false,
            } => {
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstructionSpec::BranchRegisterNonZero {
                register: source,
                when_true,
                when_false,
            } => {
                register(*source)?;
                target(*when_true)?;
                target(*when_false)?;
            }
            VisualInstructionSpec::SetRegisterToLength {
                register: destination,
            } => register(*destination)?,
            VisualInstructionSpec::Read {
                node_id,
                register: source,
            }
            | VisualInstructionSpec::ReadPrevious {
                node_id,
                register: source,
            } => {
                if node_id.is_empty() {
                    return Err("visual read node ID must not be empty".to_owned());
                }
                register(*source)?;
            }
            VisualInstructionSpec::PredicateEven {
                node_id,
                register: source,
                ..
            } => {
                if node_id.is_empty() {
                    return Err("visual predicate node ID must not be empty".to_owned());
                }
                register(*source)?;
            }
            VisualInstructionSpec::CompareLess {
                node_id,
                left_register,
                right_register,
            } => {
                if node_id.is_empty() {
                    return Err("visual compare node ID must not be empty".to_owned());
                }
                register(*left_register)?;
                register(*right_register)?;
            }
            VisualInstructionSpec::CompareGreater {
                node_id,
                left_register,
                right_register,
                ..
            } => {
                if node_id.is_empty() {
                    return Err("visual compare node ID must not be empty".to_owned());
                }
                register(*left_register)?;
                register(*right_register)?;
            }
            VisualInstructionSpec::CompareLessPrevious {
                node_id,
                register: source,
            } => {
                if node_id.is_empty() {
                    return Err("visual compare node ID must not be empty".to_owned());
                }
                register(*source)?;
            }
            VisualInstructionSpec::CopyIfLess {
                target_register,
                source_register,
            } => {
                register(*target_register)?;
                register(*source_register)?;
            }
            VisualInstructionSpec::CopyRegister {
                target_register,
                source_register,
            } => {
                register(*target_register)?;
                register(*source_register)?;
            }
            VisualInstructionSpec::Increment {
                register: destination,
            }
            | VisualInstructionSpec::Decrement {
                register: destination,
            }
            | VisualInstructionSpec::ReturnOptionalIndex {
                register: destination,
            } => {
                register(*destination)?;
            }
            VisualInstructionSpec::SwapPrevious {
                node_id,
                left_register,
                right_register,
            }
            | VisualInstructionSpec::SwapRegisters {
                node_id,
                left_register,
                right_register,
            } => {
                if node_id.is_empty() {
                    return Err("visual swap node ID must not be empty".to_owned());
                }
                register(*left_register)?;
                register(*right_register)?;
            }
            VisualInstructionSpec::ReturnIndex {
                node_id,
                register: source,
            } => {
                if node_id.is_empty() {
                    return Err("visual return node ID must not be empty".to_owned());
                }
                register(*source)?;
            }
            VisualInstructionSpec::ReturnNone => {}
            VisualInstructionSpec::Jump {
                target: destination,
            } => target(*destination)?,
        }
    }
    if !program.instructions.iter().any(|instruction| {
        matches!(
            instruction,
            VisualInstructionSpec::ReturnOptionalIndex { .. }
                | VisualInstructionSpec::ReturnIndex { .. }
                | VisualInstructionSpec::ReturnNone
        )
    }) {
        return Err("visual program has no return instruction".to_owned());
    }
    Ok(())
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IsSortedObservation {
    sorted: bool,
    comparisons: u32,
    first_inversion: Option<u32>,
}

#[wasm_bindgen]
impl IsSortedObservation {
    #[wasm_bindgen(getter)]
    pub fn sorted(&self) -> bool {
        self.sorted
    }

    #[wasm_bindgen(getter)]
    pub fn comparisons(&self) -> u32 {
        self.comparisons
    }

    #[wasm_bindgen(getter)]
    pub fn first_inversion(&self) -> Option<u32> {
        self.first_inversion
    }
}

#[wasm_bindgen]
pub fn observe_is_sorted_i32(values: &[i32]) -> Result<IsSortedObservation, JsError> {
    observe_is_sorted(values).map_err(|length| {
        JsError::new(&format!(
            "input length {length} exceeds the Atlas browser limit of {MAX_INPUT_LENGTH}"
        ))
    })
}

pub fn observe_is_sorted(values: &[i32]) -> Result<IsSortedObservation, usize> {
    if values.len() > MAX_INPUT_LENGTH {
        return Err(values.len());
    }

    let mut comparisons = 0_u32;
    let mut first_inversion = None;
    let sorted = is_sorted_by(values, |left, right| {
        comparisons += 1;
        let ordering = left.cmp(right);
        if ordering == Ordering::Greater {
            first_inversion = Some(comparisons);
        }
        ordering
    });

    Ok(IsSortedObservation {
        sorted,
        comparisons,
        first_inversion,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TraceOperation {
    Read,
    Compare,
    Swap,
}

impl TraceOperation {
    fn name(self) -> &'static str {
        match self {
            Self::Read => "Read",
            Self::Compare => "Compare",
            Self::Swap => "Swap",
        }
    }
}

fn ordering_value(ordering: Ordering) -> i8 {
    match ordering {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TraceEvent {
    node_id: &'static str,
    operation: TraceOperation,
    left_index: u32,
    right_index: Option<u32>,
    ordering: Option<i8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IsSortedStepPhase {
    LeftRead,
    RightRead,
    Compare,
    Complete,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IsSortedStepper {
    values: Vec<i32>,
    index: usize,
    phase: IsSortedStepPhase,
    sorted: bool,
    first_inversion: Option<u32>,
    comparisons: u32,
    steps: u32,
    operation: Option<TraceEvent>,
}

#[wasm_bindgen]
impl IsSortedStepper {
    #[wasm_bindgen(constructor)]
    pub fn new(values: &[i32]) -> Result<IsSortedStepper, JsError> {
        Self::from_values(values).map_err(|length| {
            JsError::new(&format!(
                "stepper input length {length} exceeds the Atlas Explore limit of {MAX_TRACE_INPUT_LENGTH}"
            ))
        })
    }

    pub fn reset(&mut self, values: &[i32]) -> Result<(), JsError> {
        *self = Self::new(values)?;
        Ok(())
    }

    pub fn step(&mut self) -> bool {
        self.operation = None;
        match self.phase {
            IsSortedStepPhase::LeftRead => {
                self.operation = Some(TraceEvent {
                    node_id: LEFT_READ_NODE,
                    operation: TraceOperation::Read,
                    left_index: self.index as u32 - 1,
                    right_index: None,
                    ordering: None,
                });
                self.phase = IsSortedStepPhase::RightRead;
            }
            IsSortedStepPhase::RightRead => {
                self.operation = Some(TraceEvent {
                    node_id: RIGHT_READ_NODE,
                    operation: TraceOperation::Read,
                    left_index: self.index as u32,
                    right_index: None,
                    ordering: None,
                });
                self.phase = IsSortedStepPhase::Compare;
            }
            IsSortedStepPhase::Compare => {
                let ordering = self.values[self.index - 1].cmp(&self.values[self.index]);
                self.comparisons += 1;
                self.operation = Some(TraceEvent {
                    node_id: COMPARE_NODE,
                    operation: TraceOperation::Compare,
                    left_index: self.index as u32 - 1,
                    right_index: Some(self.index as u32),
                    ordering: Some(ordering_value(ordering)),
                });
                if ordering == Ordering::Greater {
                    self.sorted = false;
                    self.first_inversion = Some(self.index as u32);
                    self.phase = IsSortedStepPhase::Complete;
                } else {
                    self.index += 1;
                    self.phase = if self.index >= self.values.len() {
                        IsSortedStepPhase::Complete
                    } else {
                        IsSortedStepPhase::LeftRead
                    };
                }
            }
            IsSortedStepPhase::Complete => return false,
        }
        self.steps += 1;
        true
    }

    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Box<[i32]> {
        self.values.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn sorted(&self) -> bool {
        self.sorted
    }

    #[wasm_bindgen(getter)]
    pub fn first_inversion(&self) -> Option<u32> {
        self.first_inversion
    }

    #[wasm_bindgen(getter)]
    pub fn comparisons(&self) -> u32 {
        self.comparisons
    }

    #[wasm_bindgen(getter)]
    pub fn steps(&self) -> u32 {
        self.steps
    }

    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.phase == IsSortedStepPhase::Complete
    }

    #[wasm_bindgen(getter)]
    pub fn operation_node_id(&self) -> Option<String> {
        self.operation.map(|operation| operation.node_id.to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn operation_kind(&self) -> Option<String> {
        self.operation
            .map(|operation| operation.operation.name().to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn operation_left_index(&self) -> Option<u32> {
        self.operation.map(|operation| operation.left_index)
    }

    #[wasm_bindgen(getter)]
    pub fn operation_right_index(&self) -> Option<u32> {
        self.operation.and_then(|operation| operation.right_index)
    }

    #[wasm_bindgen(getter)]
    pub fn operation_ordering(&self) -> Option<i8> {
        self.operation.and_then(|operation| operation.ordering)
    }
}

impl IsSortedStepper {
    fn from_values(values: &[i32]) -> Result<Self, usize> {
        if values.len() > MAX_TRACE_INPUT_LENGTH {
            return Err(values.len());
        }
        Ok(Self {
            values: values.to_vec(),
            index: 1,
            phase: if values.len() < 2 {
                IsSortedStepPhase::Complete
            } else {
                IsSortedStepPhase::LeftRead
            },
            sorted: true,
            first_inversion: None,
            comparisons: 0,
            steps: 0,
            operation: None,
        })
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IsSortedTrace {
    sorted: bool,
    first_inversion: Option<u32>,
    events: Vec<TraceEvent>,
}

#[wasm_bindgen]
impl IsSortedTrace {
    #[wasm_bindgen(getter)]
    pub fn sorted(&self) -> bool {
        self.sorted
    }

    #[wasm_bindgen(getter)]
    pub fn first_inversion(&self) -> Option<u32> {
        self.first_inversion
    }

    #[wasm_bindgen(getter)]
    pub fn event_count(&self) -> u32 {
        self.events.len() as u32
    }

    pub fn event_node_id(&self, index: u32) -> Option<String> {
        self.events
            .get(index as usize)
            .map(|event| event.node_id.to_owned())
    }

    pub fn event_operation(&self, index: u32) -> Option<String> {
        self.events
            .get(index as usize)
            .map(|event| event.operation.name().to_owned())
    }

    pub fn event_left_index(&self, index: u32) -> Option<u32> {
        self.events
            .get(index as usize)
            .map(|event| event.left_index)
    }

    pub fn event_right_index(&self, index: u32) -> Option<u32> {
        self.events
            .get(index as usize)
            .and_then(|event| event.right_index)
    }

    pub fn event_ordering(&self, index: u32) -> Option<i8> {
        self.events
            .get(index as usize)
            .and_then(|event| event.ordering)
    }
}

#[wasm_bindgen]
pub fn trace_is_sorted_i32(values: &[i32]) -> Result<IsSortedTrace, JsError> {
    trace_is_sorted(values).map_err(|length| {
        JsError::new(&format!(
            "trace input length {length} exceeds the Atlas Explore limit of {MAX_TRACE_INPUT_LENGTH}"
        ))
    })
}

pub fn trace_is_sorted(values: &[i32]) -> Result<IsSortedTrace, usize> {
    if values.len() > MAX_TRACE_INPUT_LENGTH {
        return Err(values.len());
    }

    let mut events = Vec::with_capacity(values.len().saturating_sub(1) * 3);
    let mut first_inversion = None;
    let sorted = is_sorted_by(values, |left, right| {
        let right_index = (events.len() / 3 + 1) as u32;
        let left_index = right_index - 1;
        events.push(TraceEvent {
            node_id: LEFT_READ_NODE,
            operation: TraceOperation::Read,
            left_index,
            right_index: None,
            ordering: None,
        });
        events.push(TraceEvent {
            node_id: RIGHT_READ_NODE,
            operation: TraceOperation::Read,
            left_index: right_index,
            right_index: None,
            ordering: None,
        });
        let ordering = left.cmp(right);
        events.push(TraceEvent {
            node_id: COMPARE_NODE,
            operation: TraceOperation::Compare,
            left_index,
            right_index: Some(right_index),
            ordering: Some(match ordering {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }),
        });
        if ordering == Ordering::Greater {
            first_inversion = Some(right_index);
        }
        ordering
    });

    Ok(IsSortedTrace {
        sorted,
        first_inversion,
        events,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TaggedValue {
    value: i32,
    original_index: u32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsertionSortObservation {
    values: Vec<i32>,
    original_indices: Vec<u32>,
    comparisons: u32,
    swaps: u32,
}

#[wasm_bindgen]
impl InsertionSortObservation {
    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Box<[i32]> {
        self.values.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn original_indices(&self) -> Box<[u32]> {
        self.original_indices.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn comparisons(&self) -> u32 {
        self.comparisons
    }

    #[wasm_bindgen(getter)]
    pub fn swaps(&self) -> u32 {
        self.swaps
    }
}

#[wasm_bindgen]
pub fn observe_insertion_sort_i32(values: &[i32]) -> Result<InsertionSortObservation, JsError> {
    observe_insertion_sort(values).map_err(|length| {
        JsError::new(&format!(
            "input length {length} exceeds the Atlas browser limit of {MAX_INPUT_LENGTH}"
        ))
    })
}

pub fn observe_insertion_sort(values: &[i32]) -> Result<InsertionSortObservation, usize> {
    if values.len() > MAX_INPUT_LENGTH {
        return Err(values.len());
    }

    let mut tagged: Vec<_> = values
        .iter()
        .copied()
        .enumerate()
        .map(|(original_index, value)| TaggedValue {
            value,
            original_index: original_index as u32,
        })
        .collect();
    let mut comparisons = 0_u32;
    let mut swaps = 0_u32;
    insertion_sort_by(&mut tagged, |left, right| {
        comparisons += 1;
        let ordering = left.value.cmp(&right.value);
        if ordering == Ordering::Less {
            swaps += 1;
        }
        ordering
    });

    Ok(InsertionSortObservation {
        values: tagged.iter().map(|item| item.value).collect(),
        original_indices: tagged.iter().map(|item| item.original_index).collect(),
        comparisons,
        swaps,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InsertionStepPhase {
    CurrentRead,
    PreviousRead,
    Compare,
    Swap,
    Complete,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsertionSortStepper {
    tagged: Vec<TaggedValue>,
    outer_index: usize,
    current_index: usize,
    phase: InsertionStepPhase,
    comparisons: u32,
    swaps: u32,
    steps: u32,
    operation: Option<TraceEvent>,
}

#[wasm_bindgen]
impl InsertionSortStepper {
    #[wasm_bindgen(constructor)]
    pub fn new(values: &[i32]) -> Result<InsertionSortStepper, JsError> {
        Self::from_values(values).map_err(|length| {
            JsError::new(&format!(
                "stepper input length {length} exceeds the Atlas insertion Explore limit of {MAX_INSERTION_STEPPER_INPUT_LENGTH}"
            ))
        })
    }

    pub fn reset(&mut self, values: &[i32]) -> Result<(), JsError> {
        *self = Self::new(values)?;
        Ok(())
    }

    /// Executes one semantic AST operation, returning false only after completion.
    pub fn step(&mut self) -> bool {
        self.operation = None;
        match self.phase {
            InsertionStepPhase::CurrentRead => {
                self.operation = Some(TraceEvent {
                    node_id: INSERTION_CURRENT_READ_NODE,
                    operation: TraceOperation::Read,
                    left_index: self.current_index as u32,
                    right_index: None,
                    ordering: None,
                });
                self.phase = InsertionStepPhase::PreviousRead;
            }
            InsertionStepPhase::PreviousRead => {
                self.operation = Some(TraceEvent {
                    node_id: INSERTION_PREVIOUS_READ_NODE,
                    operation: TraceOperation::Read,
                    left_index: self.current_index as u32 - 1,
                    right_index: None,
                    ordering: None,
                });
                self.phase = InsertionStepPhase::Compare;
            }
            InsertionStepPhase::Compare => {
                let previous = self.current_index - 1;
                let ordering = self.tagged[self.current_index]
                    .value
                    .cmp(&self.tagged[previous].value);
                self.comparisons += 1;
                self.operation = Some(TraceEvent {
                    node_id: INSERTION_COMPARE_NODE,
                    operation: TraceOperation::Compare,
                    left_index: self.current_index as u32,
                    right_index: Some(previous as u32),
                    ordering: Some(ordering_value(ordering)),
                });
                if ordering == Ordering::Less {
                    self.phase = InsertionStepPhase::Swap;
                } else {
                    self.advance_outer_index();
                }
            }
            InsertionStepPhase::Swap => {
                let previous = self.current_index - 1;
                self.tagged.swap(self.current_index, previous);
                self.swaps += 1;
                self.operation = Some(TraceEvent {
                    node_id: INSERTION_SWAP_NODE,
                    operation: TraceOperation::Swap,
                    left_index: self.current_index as u32,
                    right_index: Some(previous as u32),
                    ordering: None,
                });
                self.current_index = previous;
                if self.current_index == 0 {
                    self.advance_outer_index();
                } else {
                    self.phase = InsertionStepPhase::CurrentRead;
                }
            }
            InsertionStepPhase::Complete => return false,
        }
        self.steps += 1;
        true
    }

    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Box<[i32]> {
        self.tagged
            .iter()
            .map(|item| item.value)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn original_indices(&self) -> Box<[u32]> {
        self.tagged
            .iter()
            .map(|item| item.original_index)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn comparisons(&self) -> u32 {
        self.comparisons
    }

    #[wasm_bindgen(getter)]
    pub fn swaps(&self) -> u32 {
        self.swaps
    }

    #[wasm_bindgen(getter)]
    pub fn steps(&self) -> u32 {
        self.steps
    }

    #[wasm_bindgen(getter)]
    pub fn outer_index(&self) -> u32 {
        self.outer_index as u32
    }

    #[wasm_bindgen(getter)]
    pub fn current_index(&self) -> u32 {
        self.current_index as u32
    }

    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.phase == InsertionStepPhase::Complete
    }

    #[wasm_bindgen(getter)]
    pub fn operation_node_id(&self) -> Option<String> {
        self.operation.map(|operation| operation.node_id.to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn operation_kind(&self) -> Option<String> {
        self.operation
            .map(|operation| operation.operation.name().to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn operation_left_index(&self) -> Option<u32> {
        self.operation.map(|operation| operation.left_index)
    }

    #[wasm_bindgen(getter)]
    pub fn operation_right_index(&self) -> Option<u32> {
        self.operation.and_then(|operation| operation.right_index)
    }

    #[wasm_bindgen(getter)]
    pub fn operation_ordering(&self) -> Option<i8> {
        self.operation.and_then(|operation| operation.ordering)
    }
}

impl InsertionSortStepper {
    fn from_values(values: &[i32]) -> Result<Self, usize> {
        if values.len() > MAX_INSERTION_STEPPER_INPUT_LENGTH {
            return Err(values.len());
        }
        let tagged = values
            .iter()
            .copied()
            .enumerate()
            .map(|(original_index, value)| TaggedValue {
                value,
                original_index: original_index as u32,
            })
            .collect();
        let phase = if values.len() < 2 {
            InsertionStepPhase::Complete
        } else {
            InsertionStepPhase::CurrentRead
        };
        Ok(Self {
            tagged,
            outer_index: 1,
            current_index: 1,
            phase,
            comparisons: 0,
            swaps: 0,
            steps: 0,
            operation: None,
        })
    }

    fn advance_outer_index(&mut self) {
        self.outer_index += 1;
        if self.outer_index >= self.tagged.len() {
            self.phase = InsertionStepPhase::Complete;
        } else {
            self.current_index = self.outer_index;
            self.phase = InsertionStepPhase::CurrentRead;
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsertionSortTrace {
    values: Vec<i32>,
    original_indices: Vec<u32>,
    comparisons: u32,
    swaps: u32,
    events: Vec<TraceEvent>,
}

#[wasm_bindgen]
impl InsertionSortTrace {
    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Box<[i32]> {
        self.values.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn original_indices(&self) -> Box<[u32]> {
        self.original_indices.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn comparisons(&self) -> u32 {
        self.comparisons
    }

    #[wasm_bindgen(getter)]
    pub fn swaps(&self) -> u32 {
        self.swaps
    }

    #[wasm_bindgen(getter)]
    pub fn event_count(&self) -> u32 {
        self.events.len() as u32
    }

    pub fn event_node_id(&self, index: u32) -> Option<String> {
        self.events
            .get(index as usize)
            .map(|event| event.node_id.to_owned())
    }

    pub fn event_operation(&self, index: u32) -> Option<String> {
        self.events
            .get(index as usize)
            .map(|event| event.operation.name().to_owned())
    }

    pub fn event_left_index(&self, index: u32) -> Option<u32> {
        self.events
            .get(index as usize)
            .map(|event| event.left_index)
    }

    pub fn event_right_index(&self, index: u32) -> Option<u32> {
        self.events
            .get(index as usize)
            .and_then(|event| event.right_index)
    }

    pub fn event_ordering(&self, index: u32) -> Option<i8> {
        self.events
            .get(index as usize)
            .and_then(|event| event.ordering)
    }
}

#[wasm_bindgen]
pub fn trace_insertion_sort_i32(values: &[i32]) -> Result<InsertionSortTrace, JsError> {
    trace_insertion_sort(values).map_err(|length| {
        JsError::new(&format!(
            "trace input length {length} exceeds the Atlas insertion Explore limit of {MAX_INSERTION_TRACE_INPUT_LENGTH}"
        ))
    })
}

pub fn trace_insertion_sort(values: &[i32]) -> Result<InsertionSortTrace, usize> {
    if values.len() > MAX_INSERTION_TRACE_INPUT_LENGTH {
        return Err(values.len());
    }

    let mut tagged: Vec<_> = values
        .iter()
        .copied()
        .enumerate()
        .map(|(original_index, value)| TaggedValue {
            value,
            original_index: original_index as u32,
        })
        .collect();
    let mut comparisons = 0_u32;
    let mut swaps = 0_u32;
    let mut events = Vec::new();
    for index in 1..tagged.len() {
        let mut current = index;
        while current > 0 {
            let previous = current - 1;
            events.push(TraceEvent {
                node_id: INSERTION_CURRENT_READ_NODE,
                operation: TraceOperation::Read,
                left_index: current as u32,
                right_index: None,
                ordering: None,
            });
            events.push(TraceEvent {
                node_id: INSERTION_PREVIOUS_READ_NODE,
                operation: TraceOperation::Read,
                left_index: previous as u32,
                right_index: None,
                ordering: None,
            });
            let ordering = tagged[current].value.cmp(&tagged[previous].value);
            comparisons += 1;
            events.push(TraceEvent {
                node_id: INSERTION_COMPARE_NODE,
                operation: TraceOperation::Compare,
                left_index: current as u32,
                right_index: Some(previous as u32),
                ordering: Some(match ordering {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                }),
            });
            if ordering != Ordering::Less {
                break;
            }
            events.push(TraceEvent {
                node_id: INSERTION_SWAP_NODE,
                operation: TraceOperation::Swap,
                left_index: current as u32,
                right_index: Some(previous as u32),
                ordering: None,
            });
            tagged.swap(current, previous);
            swaps += 1;
            current = previous;
        }
    }

    Ok(InsertionSortTrace {
        values: tagged.iter().map(|item| item.value).collect(),
        original_indices: tagged.iter().map(|item| item.original_index).collect(),
        comparisons,
        swaps,
        events,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReverseStepPhase {
    LeftRead,
    RightRead,
    Swap,
    Complete,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReverseStepper {
    values: Vec<i32>,
    original_indices: Vec<u32>,
    left_index: usize,
    right_index: usize,
    phase: ReverseStepPhase,
    reads: u32,
    writes: u32,
    swaps: u32,
    steps: u32,
    operation: Option<TraceEvent>,
}

#[wasm_bindgen]
impl ReverseStepper {
    #[wasm_bindgen(constructor)]
    pub fn new(values: &[i32]) -> Result<ReverseStepper, JsError> {
        Self::from_values(values).map_err(|length| {
            JsError::new(&format!(
                "stepper input length {length} exceeds the Atlas Explore limit of {MAX_TRACE_INPUT_LENGTH}"
            ))
        })
    }

    pub fn reset(&mut self, values: &[i32]) -> Result<(), JsError> {
        *self = Self::new(values)?;
        Ok(())
    }

    pub fn step(&mut self) -> bool {
        self.operation = None;
        match self.phase {
            ReverseStepPhase::LeftRead => {
                self.reads += 1;
                self.operation = Some(TraceEvent {
                    node_id: REVERSE_LEFT_READ_NODE,
                    operation: TraceOperation::Read,
                    left_index: self.left_index as u32,
                    right_index: None,
                    ordering: None,
                });
                self.phase = ReverseStepPhase::RightRead;
            }
            ReverseStepPhase::RightRead => {
                self.reads += 1;
                self.operation = Some(TraceEvent {
                    node_id: REVERSE_RIGHT_READ_NODE,
                    operation: TraceOperation::Read,
                    left_index: self.right_index as u32,
                    right_index: None,
                    ordering: None,
                });
                self.phase = ReverseStepPhase::Swap;
            }
            ReverseStepPhase::Swap => {
                self.values.swap(self.left_index, self.right_index);
                self.original_indices
                    .swap(self.left_index, self.right_index);
                self.writes += 2;
                self.swaps += 1;
                self.operation = Some(TraceEvent {
                    node_id: REVERSE_SWAP_NODE,
                    operation: TraceOperation::Swap,
                    left_index: self.left_index as u32,
                    right_index: Some(self.right_index as u32),
                    ordering: None,
                });
                self.left_index += 1;
                self.right_index = self.right_index.saturating_sub(1);
                self.phase = if self.left_index >= self.right_index {
                    ReverseStepPhase::Complete
                } else {
                    ReverseStepPhase::LeftRead
                };
            }
            ReverseStepPhase::Complete => return false,
        }
        self.steps += 1;
        true
    }

    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Box<[i32]> {
        self.values.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn original_indices(&self) -> Box<[u32]> {
        self.original_indices.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn reads(&self) -> u32 {
        self.reads
    }

    #[wasm_bindgen(getter)]
    pub fn writes(&self) -> u32 {
        self.writes
    }

    #[wasm_bindgen(getter)]
    pub fn swaps(&self) -> u32 {
        self.swaps
    }

    #[wasm_bindgen(getter)]
    pub fn steps(&self) -> u32 {
        self.steps
    }

    #[wasm_bindgen(getter)]
    pub fn left_index(&self) -> u32 {
        self.left_index as u32
    }

    #[wasm_bindgen(getter)]
    pub fn right_index(&self) -> u32 {
        self.right_index as u32
    }

    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.phase == ReverseStepPhase::Complete
    }

    #[wasm_bindgen(getter)]
    pub fn operation_node_id(&self) -> Option<String> {
        self.operation.map(|operation| operation.node_id.to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn operation_kind(&self) -> Option<String> {
        self.operation
            .map(|operation| operation.operation.name().to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn operation_left_index(&self) -> Option<u32> {
        self.operation.map(|operation| operation.left_index)
    }

    #[wasm_bindgen(getter)]
    pub fn operation_right_index(&self) -> Option<u32> {
        self.operation.and_then(|operation| operation.right_index)
    }

    #[wasm_bindgen(getter)]
    pub fn operation_ordering(&self) -> Option<i8> {
        None
    }
}

impl ReverseStepper {
    fn from_values(values: &[i32]) -> Result<Self, usize> {
        if values.len() > MAX_TRACE_INPUT_LENGTH {
            return Err(values.len());
        }
        Ok(Self {
            values: values.to_vec(),
            original_indices: (0..values.len() as u32).collect(),
            left_index: 0,
            right_index: values.len().saturating_sub(1),
            phase: if values.len() < 2 {
                ReverseStepPhase::Complete
            } else {
                ReverseStepPhase::LeftRead
            },
            reads: 0,
            writes: 0,
            swaps: 0,
            steps: 0,
            operation: None,
        })
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReverseObservation {
    values: Vec<i32>,
    reads: u32,
    writes: u32,
    swaps: u32,
}

#[wasm_bindgen]
impl ReverseObservation {
    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Box<[i32]> {
        self.values.clone().into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn reads(&self) -> u32 {
        self.reads
    }

    #[wasm_bindgen(getter)]
    pub fn writes(&self) -> u32 {
        self.writes
    }

    #[wasm_bindgen(getter)]
    pub fn swaps(&self) -> u32 {
        self.swaps
    }
}

#[wasm_bindgen]
pub fn observe_reverse_i32(values: &[i32]) -> Result<ReverseObservation, JsError> {
    observe_reverse(values).map_err(|length| {
        JsError::new(&format!(
            "input length {length} exceeds the Atlas browser limit of {MAX_INPUT_LENGTH}"
        ))
    })
}

pub fn observe_reverse(values: &[i32]) -> Result<ReverseObservation, usize> {
    if values.len() > MAX_INPUT_LENGTH {
        return Err(values.len());
    }

    let mut output = values.to_vec();
    reverse_in_place(&mut output);
    let swaps = (values.len() / 2) as u32;

    Ok(ReverseObservation {
        values: output,
        reads: swaps * 2,
        writes: swaps * 2,
        swaps,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        COMPARE_NODE, INSERTION_COMPARE_NODE, INSERTION_CURRENT_READ_NODE,
        INSERTION_PREVIOUS_READ_NODE, INSERTION_SWAP_NODE, InsertionSortObservation,
        InsertionSortStepper, IsSortedObservation, IsSortedStepper, LEFT_READ_NODE,
        MAX_INPUT_LENGTH, MAX_INSERTION_STEPPER_INPUT_LENGTH, MAX_INSERTION_TRACE_INPUT_LENGTH,
        MAX_TRACE_INPUT_LENGTH, REVERSE_LEFT_READ_NODE, REVERSE_RIGHT_READ_NODE, REVERSE_SWAP_NODE,
        RIGHT_READ_NODE, ReverseObservation, ReverseStepper, TraceOperation,
        observe_insertion_sort, observe_is_sorted, observe_reverse, trace_insertion_sort,
        trace_is_sorted,
    };

    fn minimum_visual_machine(values: &[i32]) -> super::VisualMachine {
        let program =
            atlas::visual_program::compile_minimum_visual_program(&atlas::ast::minimum_ast())
                .unwrap();
        super::VisualMachine::from_json(&serde_json::to_string(&program).unwrap(), values).unwrap()
    }

    fn partition_visual_machine(values: &[i32]) -> super::VisualMachine {
        let program = atlas::visual_program::compile_partition_even_visual_program(
            &atlas::ast::partition_ast(),
        )
        .unwrap();
        super::VisualMachine::from_json(&serde_json::to_string(&program).unwrap(), values).unwrap()
    }

    fn is_sorted_visual_machine(values: &[i32]) -> super::VisualMachine {
        let program =
            atlas::visual_program::compile_is_sorted_visual_program(&atlas::ast::is_sorted_ast())
                .unwrap();
        super::VisualMachine::from_json(&serde_json::to_string(&program).unwrap(), values).unwrap()
    }

    fn insertion_visual_machine(values: &[i32]) -> super::VisualMachine {
        let program = atlas::visual_program::compile_insertion_visual_program(
            &atlas::ast::insertion_sort_ast(),
        )
        .unwrap();
        super::VisualMachine::from_json(&serde_json::to_string(&program).unwrap(), values).unwrap()
    }

    fn reverse_visual_machine(values: &[i32]) -> super::VisualMachine {
        let program =
            atlas::visual_program::compile_reverse_visual_program(&atlas::ast::reverse_ast())
                .unwrap();
        super::VisualMachine::from_json(&serde_json::to_string(&program).unwrap(), values).unwrap()
    }

    #[test]
    fn generated_minimum_machine_matches_native_and_preserves_first_tie() {
        use atlas_algorithms::minimum::minimum_by;

        for input in [vec![7, -2, 4, -1, 9], vec![2, 1, 1, 3], vec![42], vec![]] {
            let native = minimum_by(&input, i32::cmp).copied();
            let mut machine = minimum_visual_machine(&input);
            while machine.step_checked().unwrap() {}

            assert_eq!(machine.result_value(), native);
            assert_eq!(machine.has_result(), native.is_some());
            if input == [2, 1, 1, 3] {
                assert_eq!(machine.result_index(), Some(1));
            }
            assert_eq!(machine.comparisons, input.len().saturating_sub(1) as u32);
            assert_eq!(machine.steps, machine.comparisons * 3);
        }
    }

    #[test]
    fn generated_minimum_machine_emits_exact_ast_linked_operations() {
        let mut machine = minimum_visual_machine(&[3, 1, 2]);
        let mut operations = Vec::new();

        while machine.step_checked().unwrap() {
            let operation = machine.operation.as_ref().unwrap();
            operations.push((operation.node_id.clone(), operation.kind));
        }

        assert_eq!(
            operations,
            [
                ("minimum.candidate.read".to_owned(), "Read"),
                ("minimum.best.read".to_owned(), "Read"),
                ("minimum.compare".to_owned(), "Compare"),
                ("minimum.candidate.read".to_owned(), "Read"),
                ("minimum.best.read".to_owned(), "Read"),
                ("minimum.compare".to_owned(), "Compare"),
            ]
        );
        assert!(machine.done);
        assert_eq!(machine.result_index, Some(1));
    }

    #[test]
    fn generated_even_partition_matches_native_mutation_and_boundary() {
        use atlas_algorithms::partition::partition_in_place;

        for input in [
            vec![],
            vec![2, 4, 6],
            vec![1, 3, 5],
            vec![1, 2, 3, 4, 5, 6],
            vec![0, 1, 0, 1, 0, 1],
        ] {
            let mut native = input.clone();
            let mut native_predicates = 0;
            let native_boundary = partition_in_place(&mut native, |value| {
                native_predicates += 1;
                value % 2 == 0
            });
            let mut machine = partition_visual_machine(&input);
            while machine.step_checked().unwrap() {}

            assert_eq!(machine.values, native);
            assert_eq!(machine.result_index(), Some(native_boundary as u32));
            assert_eq!(machine.predicate_evaluations, native_predicates);
            assert!(
                machine.values[..native_boundary]
                    .iter()
                    .all(|value| value % 2 == 0)
            );
            assert!(
                machine.values[native_boundary..]
                    .iter()
                    .all(|value| value % 2 != 0)
            );
            let mut origins = machine.original_indices().into_vec();
            origins.sort_unstable();
            assert_eq!(origins, (0..input.len() as u32).collect::<Vec<_>>());
        }
    }

    #[test]
    fn generated_even_partition_exposes_mutation_and_exact_ast_nodes() {
        use atlas::ast::{SemanticOperation, partition_ast};

        let mut machine = partition_visual_machine(&[1, 2, 3, 4, 5, 6]);
        let mut operations = Vec::new();
        while machine.step_checked().unwrap() {
            let operation = machine.operation.as_ref().unwrap();
            operations.push((operation.node_id.clone(), operation.kind));
        }

        assert_eq!(machine.values, [6, 2, 4, 3, 5, 1]);
        assert_eq!(machine.result_index(), Some(3));
        assert_eq!(machine.predicate_evaluations, 6);
        assert_eq!(machine.swaps, 2);
        assert_eq!(machine.steps, 15);
        assert_eq!(
            operations
                .iter()
                .filter(|(_, kind)| *kind == "Swap")
                .count(),
            2
        );
        assert_eq!(operations.last().unwrap().0, "partition.boundary");

        let ast = partition_ast();
        for (node_id, kind) in operations {
            let expected = match kind {
                "Read" => SemanticOperation::Read,
                "Predicate" => SemanticOperation::Predicate,
                "Swap" => SemanticOperation::Swap,
                "Partition" => SemanticOperation::Partition,
                _ => panic!("unexpected operation kind {kind}"),
            };
            assert_eq!(ast.operation_by_id(&node_id), Some(expected));
        }
    }

    #[test]
    fn generated_is_sorted_matches_native_and_retained_stepper_exactly() {
        for input in [
            vec![],
            vec![7],
            vec![1, 2, 2, 5],
            vec![5, 4, 3],
            vec![1, 2, 5, 4, 6],
        ] {
            let native = super::observe_is_sorted(&input).unwrap();
            let mut retained = super::IsSortedStepper::from_values(&input).unwrap();
            let mut generated = is_sorted_visual_machine(&input);
            let mut retained_operations = Vec::new();
            let mut generated_operations = Vec::new();

            while retained.step() {
                let operation = retained.operation.unwrap();
                retained_operations.push((
                    operation.node_id.to_owned(),
                    operation.operation.name(),
                    operation.left_index,
                    operation.right_index,
                    operation.ordering,
                ));
            }
            while generated.step_checked().unwrap() {
                let operation = generated.operation.as_ref().unwrap();
                generated_operations.push((
                    operation.node_id.clone(),
                    operation.kind,
                    operation.left_index as u32,
                    operation.right_index.map(|index| index as u32),
                    operation.ordering,
                ));
            }

            assert_eq!(generated_operations, retained_operations);
            assert_eq!(generated.comparisons, native.comparisons);
            assert_eq!(generated.steps, retained.steps);
            assert_eq!(generated.result_index(), native.first_inversion);
            assert_eq!(generated.has_result(), !native.sorted);
            assert_eq!(generated.values, input);
        }
    }

    #[test]
    fn generated_insertion_matches_retained_stepper_operation_for_operation() {
        for input in [
            vec![],
            vec![7],
            vec![1, 2, 3, 4],
            vec![4, 3, 2, 1],
            vec![5, -1, 5, 3, 0, -8, 3],
        ] {
            let mut retained = super::InsertionSortStepper::from_values(&input).unwrap();
            let mut generated = insertion_visual_machine(&input);
            let mut retained_operations = Vec::new();
            let mut generated_operations = Vec::new();

            while retained.step() {
                let operation = retained.operation.unwrap();
                retained_operations.push((
                    operation.node_id.to_owned(),
                    operation.operation.name(),
                    operation.left_index,
                    operation.right_index,
                    operation.ordering,
                ));
            }
            while generated.step_checked().unwrap() {
                let operation = generated.operation.as_ref().unwrap();
                generated_operations.push((
                    operation.node_id.clone(),
                    operation.kind,
                    operation.left_index as u32,
                    operation.right_index.map(|index| index as u32),
                    operation.ordering,
                ));
            }

            assert_eq!(generated_operations, retained_operations);
            assert_eq!(generated.values(), retained.values());
            assert_eq!(generated.original_indices(), retained.original_indices());
            assert_eq!(generated.comparisons, retained.comparisons);
            assert_eq!(generated.swaps, retained.swaps);
            assert_eq!(generated.steps, retained.steps);
            assert!(generated.done);
        }
    }

    #[test]
    fn generated_reverse_matches_retained_stepper_operation_for_operation() {
        for input in [
            vec![],
            vec![7],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![5, -1, 5, 3, 0, -8, 3],
        ] {
            let mut retained = super::ReverseStepper::from_values(&input).unwrap();
            let mut generated = reverse_visual_machine(&input);
            let mut retained_operations = Vec::new();
            let mut generated_operations = Vec::new();

            while retained.step() {
                let operation = retained.operation.unwrap();
                retained_operations.push((
                    operation.node_id.to_owned(),
                    operation.operation.name(),
                    operation.left_index,
                    operation.right_index,
                    operation.ordering,
                ));
            }
            while generated.step_checked().unwrap() {
                let operation = generated.operation.as_ref().unwrap();
                generated_operations.push((
                    operation.node_id.clone(),
                    operation.kind,
                    operation.left_index as u32,
                    operation.right_index.map(|index| index as u32),
                    operation.ordering,
                ));
            }

            assert_eq!(generated_operations, retained_operations);
            assert_eq!(generated.values(), retained.values());
            assert_eq!(generated.original_indices(), retained.original_indices());
            assert_eq!(generated.reads, retained.reads);
            assert_eq!(generated.writes, retained.writes);
            assert_eq!(generated.swaps, retained.swaps);
            assert_eq!(generated.steps, retained.steps);
            assert!(generated.done);
        }
    }

    #[test]
    fn visual_machine_rejects_unknown_formats_and_out_of_bounds_programs() {
        let mut value = serde_json::to_value(
            atlas::visual_program::compile_minimum_visual_program(&atlas::ast::minimum_ast())
                .unwrap(),
        )
        .unwrap();
        value["format"] = "unknown".into();
        assert!(
            super::VisualMachine::from_json(&value.to_string(), &[1])
                .unwrap_err()
                .contains("unsupported visual program format")
        );

        value["format"] = super::VISUAL_PROGRAM_FORMAT.into();
        value["instructions"][1]["when_true"] = 999.into();
        assert!(
            super::VisualMachine::from_json(&value.to_string(), &[1])
                .unwrap_err()
                .contains("unknown target")
        );

        let program =
            atlas::visual_program::compile_minimum_visual_program(&atlas::ast::minimum_ast())
                .unwrap();
        let program = serde_json::to_string(&program).unwrap();
        assert!(super::VisualMachine::from_json(&program, &[0; MAX_INPUT_LENGTH]).is_ok());
        assert!(
            super::VisualMachine::from_json(&program, &[0; MAX_INPUT_LENGTH + 1])
                .unwrap_err()
                .contains("browser limit")
        );

        let mut looping = serde_json::from_str::<serde_json::Value>(&program).unwrap();
        looping["instructions"][3] = serde_json::json!({"op": "jump", "target": 2});
        let input = [1, 2];
        let mut machine = super::VisualMachine::from_json(&looping.to_string(), &input).unwrap();
        for _ in 0..super::MAX_VISUAL_INSTRUCTIONS * input.len() {
            assert!(machine.step_checked().unwrap());
        }
        assert!(
            machine
                .step_checked()
                .unwrap_err()
                .contains("semantic-step budget")
        );
    }

    #[test]
    fn matches_native_result_and_counts_adjacent_comparisons() {
        let fixtures: &[(&[i32], IsSortedObservation)] = &[
            (
                &[],
                IsSortedObservation {
                    sorted: true,
                    comparisons: 0,
                    first_inversion: None,
                },
            ),
            (
                &[7],
                IsSortedObservation {
                    sorted: true,
                    comparisons: 0,
                    first_inversion: None,
                },
            ),
            (
                &[-2, 0, 0, 4],
                IsSortedObservation {
                    sorted: true,
                    comparisons: 3,
                    first_inversion: None,
                },
            ),
            (
                &[1, 2, 5, 4, 6],
                IsSortedObservation {
                    sorted: false,
                    comparisons: 3,
                    first_inversion: Some(3),
                },
            ),
        ];

        for (values, expected) in fixtures {
            assert_eq!(observe_is_sorted(values), Ok(*expected));
        }
    }

    #[test]
    fn rejects_inputs_beyond_the_browser_limit() {
        let values = vec![0; MAX_INPUT_LENGTH + 1];
        assert_eq!(observe_is_sorted(&values), Err(MAX_INPUT_LENGTH + 1));
        assert_eq!(observe_insertion_sort(&values), Err(MAX_INPUT_LENGTH + 1));
        assert_eq!(observe_reverse(&values), Err(MAX_INPUT_LENGTH + 1));
    }

    #[test]
    fn insertion_sort_matches_native_order_and_exposes_stability() {
        assert_eq!(
            observe_insertion_sort(&[2, 1, 2, 1]),
            Ok(InsertionSortObservation {
                values: vec![1, 1, 2, 2],
                original_indices: vec![1, 3, 0, 2],
                comparisons: 5,
                swaps: 3,
            })
        );
        assert_eq!(
            observe_insertion_sort(&[]),
            Ok(InsertionSortObservation {
                values: vec![],
                original_indices: vec![],
                comparisons: 0,
                swaps: 0,
            })
        );
        assert_eq!(
            observe_insertion_sort(&[1, 2, 3]),
            Ok(InsertionSortObservation {
                values: vec![1, 2, 3],
                original_indices: vec![0, 1, 2],
                comparisons: 2,
                swaps: 0,
            })
        );
    }

    #[test]
    fn reverse_matches_native_result_and_exact_structural_counts() {
        let fixtures: &[(&[i32], &[i32], u32)] = &[
            (&[], &[], 0),
            (&[7], &[7], 0),
            (&[1, 2], &[2, 1], 1),
            (&[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1], 2),
            (&[1, 2, 3, 4, 5, 6], &[6, 5, 4, 3, 2, 1], 3),
        ];

        for (input, expected, swaps) in fixtures {
            assert_eq!(
                observe_reverse(input),
                Ok(ReverseObservation {
                    values: expected.to_vec(),
                    reads: swaps * 2,
                    writes: swaps * 2,
                    swaps: *swaps,
                })
            );
        }
    }

    #[test]
    fn reverse_stepper_matches_native_ast_and_structural_counts() {
        use atlas::ast::{SemanticOperation, reverse_ast};

        let input = [1, 2, 3, 4, 5];
        let native = observe_reverse(&input).unwrap();
        let mut stepper = ReverseStepper::from_values(&input).unwrap();
        let ast = reverse_ast();
        let expected = [
            (REVERSE_LEFT_READ_NODE, TraceOperation::Read),
            (REVERSE_RIGHT_READ_NODE, TraceOperation::Read),
            (REVERSE_SWAP_NODE, TraceOperation::Swap),
            (REVERSE_LEFT_READ_NODE, TraceOperation::Read),
            (REVERSE_RIGHT_READ_NODE, TraceOperation::Read),
            (REVERSE_SWAP_NODE, TraceOperation::Swap),
        ];

        for (node_id, operation) in expected {
            assert!(stepper.step());
            assert_eq!(stepper.operation.unwrap().node_id, node_id);
            assert_eq!(stepper.operation.unwrap().operation, operation);
            let ast_operation = match operation {
                TraceOperation::Read => SemanticOperation::Read,
                TraceOperation::Swap => SemanticOperation::Swap,
                TraceOperation::Compare => unreachable!(),
            };
            assert_eq!(ast.operation_by_id(node_id), Some(ast_operation));
        }

        assert!(stepper.done());
        assert!(!stepper.step());
        assert_eq!(stepper.values, native.values);
        assert_eq!(stepper.reads, native.reads);
        assert_eq!(stepper.writes, native.writes);
        assert_eq!(stepper.swaps, native.swaps);
        assert_eq!(stepper.steps, 6);
        assert_eq!(stepper.original_indices, [4, 3, 2, 1, 0]);
    }

    #[test]
    fn reverse_stepper_handles_zero_steps_and_is_an_involution() {
        for input in [&[][..], &[7][..]] {
            let mut stepper = ReverseStepper::from_values(input).unwrap();
            assert!(stepper.done());
            assert!(!stepper.step());
            assert_eq!(stepper.steps, 0);
        }

        let input = [8, 3, -1, 4];
        let mut first = ReverseStepper::from_values(&input).unwrap();
        while first.step() {}
        let mut second = ReverseStepper::from_values(&first.values).unwrap();
        while second.step() {}
        assert_eq!(second.values, input);
    }

    #[test]
    fn observed_reverse_is_an_involution() {
        let input = [4, -1, 7, 7, 2];
        let first = observe_reverse(&input).unwrap();
        let second = observe_reverse(&first.values).unwrap();

        assert_eq!(second.values, input);
    }

    #[test]
    fn is_sorted_trace_links_every_event_to_the_exact_ast_operation() {
        use atlas::ast::{SemanticOperation, is_sorted_ast};

        let trace = trace_is_sorted(&[1, 2, 5, 4, 6]).unwrap();
        assert!(!trace.sorted);
        assert_eq!(trace.first_inversion, Some(3));
        assert_eq!(trace.events.len(), 9);
        assert_eq!(
            trace
                .events
                .iter()
                .map(|event| event.node_id)
                .collect::<Vec<_>>(),
            [
                LEFT_READ_NODE,
                RIGHT_READ_NODE,
                COMPARE_NODE,
                LEFT_READ_NODE,
                RIGHT_READ_NODE,
                COMPARE_NODE,
                LEFT_READ_NODE,
                RIGHT_READ_NODE,
                COMPARE_NODE,
            ]
        );

        let ast = is_sorted_ast();
        for event in &trace.events {
            let expected = match event.operation {
                TraceOperation::Read => SemanticOperation::Read,
                TraceOperation::Compare => SemanticOperation::Compare,
                TraceOperation::Swap => SemanticOperation::Swap,
            };
            assert_eq!(ast.operation_by_id(event.node_id), Some(expected));
        }
        let comparison = trace.events.last().unwrap();
        assert_eq!(comparison.left_index, 2);
        assert_eq!(comparison.right_index, Some(3));
        assert_eq!(comparison.ordering, Some(1));
    }

    #[test]
    fn is_sorted_trace_is_bounded_to_explore_inputs() {
        assert!(trace_is_sorted(&vec![0; MAX_TRACE_INPUT_LENGTH]).is_ok());
        assert_eq!(
            trace_is_sorted(&vec![0; MAX_TRACE_INPUT_LENGTH + 1]),
            Err(MAX_TRACE_INPUT_LENGTH + 1)
        );
    }

    #[test]
    fn is_sorted_trace_distinguishes_complete_scan_from_early_stop() {
        let complete = trace_is_sorted(&[7, 7, 7, 7]).unwrap();
        assert!(complete.sorted);
        assert_eq!(complete.events.len(), 9);

        let stopped = trace_is_sorted(&[5, -1, 5, 3]).unwrap();
        assert!(!stopped.sorted);
        assert_eq!(stopped.first_inversion, Some(1));
        assert_eq!(stopped.events.len(), 3);
    }

    #[test]
    fn is_sorted_stepper_matches_trace_and_native_complete_scan() {
        let input = [1, 1, 3, 5];
        let trace = trace_is_sorted(&input).unwrap();
        let native = observe_is_sorted(&input).unwrap();
        let mut stepper = IsSortedStepper::from_values(&input).unwrap();

        for expected in &trace.events {
            assert!(stepper.step());
            assert_eq!(stepper.operation, Some(*expected));
        }

        assert!(stepper.done());
        assert!(!stepper.step());
        assert_eq!(stepper.sorted, native.sorted);
        assert_eq!(stepper.first_inversion, native.first_inversion);
        assert_eq!(stepper.comparisons, native.comparisons);
        assert_eq!(stepper.steps as usize, trace.events.len());
    }

    #[test]
    fn is_sorted_stepper_preserves_early_stop_and_zero_step_cases() {
        let mut stopped = IsSortedStepper::from_values(&[5, -1, 7]).unwrap();
        while stopped.step() {}
        assert!(!stopped.sorted);
        assert_eq!(stopped.first_inversion, Some(1));
        assert_eq!(stopped.comparisons, 1);
        assert_eq!(stopped.steps, 3);

        for input in [&[][..], &[7][..]] {
            let mut complete = IsSortedStepper::from_values(input).unwrap();
            assert!(complete.done());
            assert!(!complete.step());
            assert!(complete.sorted);
            assert_eq!(complete.steps, 0);
        }
    }

    #[test]
    fn insertion_trace_matches_native_stable_result_and_exact_ast_operations() {
        use atlas::ast::{SemanticOperation, insertion_sort_ast};

        let input = [2, 1, 2, 1];
        let trace = trace_insertion_sort(&input).unwrap();
        let native = observe_insertion_sort(&input).unwrap();
        assert_eq!(trace.values, native.values);
        assert_eq!(trace.original_indices, native.original_indices);
        assert_eq!(trace.comparisons, native.comparisons);
        assert_eq!(trace.swaps, native.swaps);
        assert_eq!(trace.events.len(), 18);

        let ast = insertion_sort_ast();
        for event in &trace.events {
            let expected = match event.operation {
                TraceOperation::Read => SemanticOperation::Read,
                TraceOperation::Compare => SemanticOperation::Compare,
                TraceOperation::Swap => SemanticOperation::Swap,
            };
            assert_eq!(ast.operation_by_id(event.node_id), Some(expected));
        }
        assert_eq!(trace.events[0].node_id, INSERTION_CURRENT_READ_NODE);
        assert_eq!(trace.events[1].node_id, INSERTION_PREVIOUS_READ_NODE);
        assert_eq!(trace.events[2].node_id, INSERTION_COMPARE_NODE);
        assert_eq!(trace.events[3].node_id, INSERTION_SWAP_NODE);
    }

    #[test]
    fn insertion_trace_snapshots_can_be_replayed_from_swaps() {
        let input = [3, 1, 2, 1];
        let trace = trace_insertion_sort(&input).unwrap();
        let mut values = input.to_vec();
        let mut original_indices: Vec<_> = (0..input.len() as u32).collect();

        for event in &trace.events {
            if event.operation == TraceOperation::Swap {
                let right = event.right_index.unwrap() as usize;
                values.swap(event.left_index as usize, right);
                original_indices.swap(event.left_index as usize, right);
            }
        }

        assert_eq!(values, trace.values);
        assert_eq!(original_indices, trace.original_indices);
    }

    #[test]
    fn insertion_stepper_executes_in_wasm_without_materializing_a_trace() {
        let input = [3, 1, 2, 1];
        let trace = trace_insertion_sort(&input).unwrap();
        let native = observe_insertion_sort(&input).unwrap();
        let mut stepper = InsertionSortStepper::from_values(&input).unwrap();

        for expected in &trace.events {
            assert!(stepper.step());
            assert_eq!(stepper.operation, Some(*expected));
        }

        assert!(stepper.done());
        assert!(!stepper.step());
        assert_eq!(stepper.steps as usize, trace.events.len());
        assert_eq!(stepper.values().as_ref(), native.values);
        assert_eq!(stepper.original_indices().as_ref(), native.original_indices);
        assert_eq!(stepper.comparisons, native.comparisons);
        assert_eq!(stepper.swaps, native.swaps);
    }

    #[test]
    fn insertion_stepper_reset_restarts_from_an_unexecuted_state() {
        let mut stepper = InsertionSortStepper::from_values(&[2, 1]).unwrap();
        assert!(stepper.step());
        stepper = InsertionSortStepper::from_values(&[4, 3, 2]).unwrap();

        assert_eq!(stepper.steps, 0);
        assert_eq!(stepper.values().as_ref(), [4, 3, 2]);
        while stepper.step() {}
        assert_eq!(stepper.values().as_ref(), [2, 3, 4]);
        assert!(stepper.done());
    }

    #[test]
    fn insertion_stepper_exposes_nested_loop_context() {
        let mut stepper = InsertionSortStepper::from_values(&[1, 2, 3]).unwrap();
        assert_eq!((stepper.outer_index, stepper.current_index), (1, 1));

        assert!(stepper.step());
        assert!(stepper.step());
        assert!(stepper.step());
        assert_eq!((stepper.outer_index, stepper.current_index), (2, 2));

        while stepper.step() {}
        assert_eq!(stepper.outer_index, 3);
        assert!(stepper.done());
    }

    #[test]
    fn insertion_trace_is_bounded_separately_from_scale_execution() {
        assert!(trace_insertion_sort(&[0; MAX_INSERTION_TRACE_INPUT_LENGTH]).is_ok());
        assert_eq!(
            trace_insertion_sort(&[0; MAX_INSERTION_TRACE_INPUT_LENGTH + 1]),
            Err(MAX_INSERTION_TRACE_INPUT_LENGTH + 1)
        );
        assert!(observe_insertion_sort(&vec![0; MAX_INPUT_LENGTH]).is_ok());
    }

    #[test]
    fn insertion_stepper_accepts_every_explore_size_without_expanding_trace_limit() {
        assert!(
            InsertionSortStepper::from_values(&[0; MAX_INSERTION_STEPPER_INPUT_LENGTH]).is_ok()
        );
        assert_eq!(
            InsertionSortStepper::from_values(&[0; MAX_INSERTION_STEPPER_INPUT_LENGTH + 1]),
            Err(MAX_INSERTION_STEPPER_INPUT_LENGTH + 1)
        );
        assert_eq!(MAX_INSERTION_TRACE_INPUT_LENGTH, 32);
    }
}

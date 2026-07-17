use serde::Serialize;
use wasm_bindgen::prelude::*;

pub const MAX_RLE_INPUT_BYTES: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Run {
    symbol: u8,
    count: u32,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
struct RleSnapshot<'a> {
    input: &'a [u8],
    cursor: usize,
    current_symbol: Option<u8>,
    current_count: u32,
    output: &'a [Run],
    phase: &'static str,
    steps: u32,
    reads: u32,
    comparisons: u32,
    emitted_runs: u32,
    last_operation: Option<&'static str>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct RleMachine {
    input: Vec<u8>,
    cursor: usize,
    current_symbol: Option<u8>,
    current_count: u32,
    output: Vec<Run>,
    complete: bool,
    steps: u32,
    reads: u32,
    comparisons: u32,
    last_operation: Option<&'static str>,
}

#[wasm_bindgen]
impl RleMachine {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Result<RleMachine, JsError> {
        Self::new_checked(input).map_err(|error| JsError::new(&error))
    }

    pub fn reset(&mut self, input: &str) -> Result<(), JsError> {
        *self = Self::new_checked(input).map_err(|error| JsError::new(&error))?;
        Ok(())
    }

    pub fn step(&mut self) -> bool {
        self.step_once()
    }

    pub fn snapshot_json(&self) -> Result<String, JsError> {
        serde_json::to_string(&self.snapshot())
            .map_err(|error| JsError::new(&format!("cannot serialize RLE snapshot: {error}")))
    }

    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.complete
    }
}

impl RleMachine {
    fn new_checked(input: &str) -> Result<Self, String> {
        if !input.is_ascii() {
            return Err("RLE browser input must contain ASCII characters only".to_owned());
        }
        if input.len() > MAX_RLE_INPUT_BYTES {
            return Err(format!(
                "RLE input length {} exceeds the Atlas browser limit of {MAX_RLE_INPUT_BYTES}",
                input.len()
            ));
        }
        Ok(Self {
            input: input.as_bytes().to_vec(),
            cursor: 0,
            current_symbol: None,
            current_count: 0,
            output: Vec::new(),
            complete: input.is_empty(),
            steps: 0,
            reads: 0,
            comparisons: 0,
            last_operation: None,
        })
    }

    fn step_once(&mut self) -> bool {
        if self.complete {
            return false;
        }
        if self.current_symbol.is_none() {
            self.current_symbol = Some(self.input[0]);
            self.current_count = 1;
            self.cursor = 1;
            self.reads += 1;
            self.last_operation = Some("start_run");
        } else if self.cursor < self.input.len() {
            let symbol = self.input[self.cursor];
            self.cursor += 1;
            self.reads += 1;
            self.comparisons += 1;
            if self.current_symbol == Some(symbol) {
                self.current_count += 1;
                self.last_operation = Some("extend_run");
            } else {
                self.emit_current();
                self.current_symbol = Some(symbol);
                self.current_count = 1;
                self.last_operation = Some("emit_and_start");
            }
        } else {
            self.emit_current();
            self.complete = true;
            self.last_operation = Some("emit_final");
        }
        self.steps += 1;
        true
    }

    fn emit_current(&mut self) {
        self.output.push(Run {
            symbol: self.current_symbol.take().expect("a current run exists"),
            count: self.current_count,
        });
        self.current_count = 0;
    }

    fn snapshot(&self) -> RleSnapshot<'_> {
        RleSnapshot {
            input: &self.input,
            cursor: self.cursor,
            current_symbol: self.current_symbol,
            current_count: self.current_count,
            output: &self.output,
            phase: if self.complete { "complete" } else { "running" },
            steps: self.steps,
            reads: self.reads,
            comparisons: self.comparisons,
            emitted_runs: self.output.len() as u32,
            last_operation: self.last_operation,
        }
    }
}

pub fn encode_ascii(input: &[u8]) -> Vec<Run> {
    let mut output: Vec<Run> = Vec::new();
    for &symbol in input {
        if let Some(run) = output.last_mut()
            && run.symbol == symbol
        {
            run.count += 1;
            continue;
        }
        output.push(Run { symbol, count: 1 });
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finish(machine: &mut RleMachine) {
        while machine.step_once() {}
    }

    #[test]
    fn incremental_machine_matches_direct_encoding() {
        let mut machine = RleMachine::new_checked("AAABCCDDDD").unwrap();
        finish(&mut machine);
        assert_eq!(machine.output, encode_ascii(b"AAABCCDDDD"));
        assert_eq!(machine.reads, 10);
        assert_eq!(machine.comparisons, 9);
        assert_eq!(machine.output.len(), 4);
    }

    #[test]
    fn run_transition_is_visible_before_final_emission() {
        let mut machine = RleMachine::new_checked("AAB").unwrap();
        assert!(machine.step_once());
        assert_eq!(machine.current_symbol, Some(b'A'));
        assert!(machine.output.is_empty());
        assert!(machine.step_once());
        assert_eq!(machine.current_count, 2);
        assert!(machine.step_once());
        assert_eq!(
            machine.output,
            [Run {
                symbol: b'A',
                count: 2
            }]
        );
        assert_eq!(machine.current_symbol, Some(b'B'));
        assert!(machine.step_once());
        assert!(machine.complete);
        assert_eq!(machine.output, encode_ascii(b"AAB"));
    }

    #[test]
    fn empty_single_and_alternating_inputs_are_exact() {
        for input in ["", "Z", "ABABAB"] {
            let mut machine = RleMachine::new_checked(input).unwrap();
            finish(&mut machine);
            assert_eq!(machine.output, encode_ascii(input.as_bytes()));
        }
    }

    #[test]
    fn rejects_non_ascii_and_oversized_input() {
        assert!(RleMachine::new_checked("é").is_err());
        assert!(RleMachine::new_checked(&"A".repeat(MAX_RLE_INPUT_BYTES + 1)).is_err());
    }
}

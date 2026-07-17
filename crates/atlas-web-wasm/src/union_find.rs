use petgraph::unionfind::UnionFind;
use serde::Serialize;
use wasm_bindgen::prelude::*;

pub const MAX_UNION_FIND_ELEMENTS: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Ready,
    Inspect,
    Merge,
    Complete,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
struct UnionFindSnapshot {
    element_count: usize,
    components: Vec<u32>,
    phase: &'static str,
    left: Option<u32>,
    right: Option<u32>,
    left_representative: Option<u32>,
    right_representative: Option<u32>,
    merged: Option<bool>,
    steps: u32,
    union_attempts: u32,
    successful_unions: u32,
    last_operation: Option<&'static str>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct UnionFindMachine {
    partition: UnionFind<u32>,
    element_count: usize,
    phase: Phase,
    left: Option<u32>,
    right: Option<u32>,
    left_representative: Option<u32>,
    right_representative: Option<u32>,
    merged: Option<bool>,
    steps: u32,
    union_attempts: u32,
    successful_unions: u32,
    last_operation: Option<&'static str>,
}

#[wasm_bindgen]
impl UnionFindMachine {
    #[wasm_bindgen(constructor)]
    pub fn new(element_count: usize) -> Result<UnionFindMachine, JsError> {
        Self::new_checked(element_count).map_err(|error| JsError::new(&error))
    }

    pub fn reset(&mut self, element_count: usize) -> Result<(), JsError> {
        self.reset_checked(element_count)
            .map_err(|error| JsError::new(&error))
    }

    pub fn begin_union(&mut self, left: u32, right: u32) -> Result<(), JsError> {
        self.begin_union_checked(left, right)
            .map_err(|error| JsError::new(&error))
    }

    pub fn step(&mut self) -> bool {
        self.step_once()
    }

    pub fn snapshot_json(&self) -> Result<String, JsError> {
        serde_json::to_string(&self.snapshot()).map_err(|error| {
            JsError::new(&format!("cannot serialize union-find snapshot: {error}"))
        })
    }

    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        matches!(self.phase, Phase::Ready | Phase::Complete)
    }
}

impl UnionFindMachine {
    fn new_checked(element_count: usize) -> Result<Self, String> {
        validate_element_count(element_count)?;
        Ok(Self {
            partition: UnionFind::new(element_count),
            element_count,
            phase: Phase::Ready,
            left: None,
            right: None,
            left_representative: None,
            right_representative: None,
            merged: None,
            steps: 0,
            union_attempts: 0,
            successful_unions: 0,
            last_operation: None,
        })
    }

    fn reset_checked(&mut self, element_count: usize) -> Result<(), String> {
        validate_element_count(element_count)?;
        self.partition = UnionFind::new(element_count);
        self.element_count = element_count;
        self.phase = Phase::Ready;
        self.left = None;
        self.right = None;
        self.left_representative = None;
        self.right_representative = None;
        self.merged = None;
        self.steps = 0;
        self.union_attempts = 0;
        self.successful_unions = 0;
        self.last_operation = None;
        Ok(())
    }

    fn begin_union_checked(&mut self, left: u32, right: u32) -> Result<(), String> {
        let left_index = left as usize;
        let right_index = right as usize;
        if left_index >= self.element_count {
            return Err(format!(
                "left element {left} is outside union-find size {}",
                self.element_count
            ));
        }
        if right_index >= self.element_count {
            return Err(format!(
                "right element {right} is outside union-find size {}",
                self.element_count
            ));
        }
        if !self.done() {
            return Err("finish the pending union before starting another".to_owned());
        }
        self.phase = Phase::Inspect;
        self.left = Some(left);
        self.right = Some(right);
        self.left_representative = None;
        self.right_representative = None;
        self.merged = None;
        self.last_operation = None;
        Ok(())
    }

    fn step_once(&mut self) -> bool {
        match self.phase {
            Phase::Inspect => {
                let left = self.left.expect("inspect phase has a left element");
                let right = self.right.expect("inspect phase has a right element");
                self.left_representative = Some(self.partition.find(left));
                self.right_representative = Some(self.partition.find(right));
                self.phase = Phase::Merge;
                self.last_operation = Some("inspect_representatives");
            }
            Phase::Merge => {
                let left = self.left.expect("merge phase has a left element");
                let right = self.right.expect("merge phase has a right element");
                let merged = self.partition.union(left, right);
                self.merged = Some(merged);
                self.union_attempts += 1;
                if merged {
                    self.successful_unions += 1;
                }
                self.phase = Phase::Complete;
                self.last_operation = Some("union_components");
            }
            Phase::Ready | Phase::Complete => return false,
        }
        self.steps += 1;
        true
    }

    fn snapshot(&self) -> UnionFindSnapshot {
        UnionFindSnapshot {
            element_count: self.element_count,
            components: self.canonical_components(),
            phase: match self.phase {
                Phase::Ready => "ready",
                Phase::Inspect => "inspect",
                Phase::Merge => "merge",
                Phase::Complete => "complete",
            },
            left: self.left,
            right: self.right,
            left_representative: self.left_representative,
            right_representative: self.right_representative,
            merged: self.merged,
            steps: self.steps,
            union_attempts: self.union_attempts,
            successful_unions: self.successful_unions,
            last_operation: self.last_operation,
        }
    }

    fn canonical_components(&self) -> Vec<u32> {
        let labels = self.partition.clone().into_labeling();
        let mut minimum_members = vec![u32::MAX; self.element_count];
        for (element, representative) in labels.iter().copied().enumerate() {
            let minimum = &mut minimum_members[representative as usize];
            *minimum = (*minimum).min(element as u32);
        }
        labels
            .into_iter()
            .map(|representative| minimum_members[representative as usize])
            .collect()
    }
}

fn validate_element_count(element_count: usize) -> Result<(), String> {
    if element_count > MAX_UNION_FIND_ELEMENTS {
        return Err(format!(
            "union-find size {element_count} exceeds the Atlas browser limit of {MAX_UNION_FIND_ELEMENTS}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_union(machine: &mut UnionFindMachine, left: u32, right: u32) {
        machine.begin_union_checked(left, right).unwrap();
        assert!(machine.step_once());
        assert!(machine.step_once());
        assert!(!machine.step_once());
    }

    #[test]
    fn union_steps_expose_representatives_before_mutation() {
        let mut machine = UnionFindMachine::new_checked(5).unwrap();
        machine.begin_union_checked(1, 3).unwrap();

        assert_eq!(machine.snapshot().phase, "inspect");
        assert!(machine.step_once());
        let inspected = machine.snapshot();
        assert_eq!(inspected.phase, "merge");
        assert_eq!(inspected.left_representative, Some(1));
        assert_eq!(inspected.right_representative, Some(3));
        assert_eq!(inspected.components, [0, 1, 2, 3, 4]);

        assert!(machine.step_once());
        let merged = machine.snapshot();
        assert_eq!(merged.phase, "complete");
        assert_eq!(merged.merged, Some(true));
        assert_eq!(merged.components, [0, 1, 2, 1, 4]);
    }

    #[test]
    fn interleaved_unions_preserve_exact_component_membership() {
        let mut machine = UnionFindMachine::new_checked(6).unwrap();
        complete_union(&mut machine, 0, 1);
        complete_union(&mut machine, 1, 2);
        complete_union(&mut machine, 3, 4);

        let snapshot = machine.snapshot();
        assert_eq!(snapshot.components, [0, 0, 0, 3, 3, 5]);
        assert_eq!(snapshot.union_attempts, 3);
        assert_eq!(snapshot.successful_unions, 3);

        complete_union(&mut machine, 0, 2);
        let redundant = machine.snapshot();
        assert_eq!(redundant.components, [0, 0, 0, 3, 3, 5]);
        assert_eq!(redundant.merged, Some(false));
        assert_eq!(redundant.union_attempts, 4);
        assert_eq!(redundant.successful_unions, 3);
    }

    #[test]
    fn reset_and_json_snapshot_are_deterministic() {
        let mut machine = UnionFindMachine::new_checked(4).unwrap();
        complete_union(&mut machine, 2, 3);
        machine.reset_checked(4).unwrap();

        let first = serde_json::to_string(&machine.snapshot()).unwrap();
        let second = serde_json::to_string(&machine.snapshot()).unwrap();
        assert_eq!(first, second);
        assert_eq!(machine.snapshot().components, [0, 1, 2, 3]);
    }

    #[test]
    fn invalid_sizes_indices_and_overlapping_operations_are_rejected() {
        assert!(UnionFindMachine::new_checked(MAX_UNION_FIND_ELEMENTS + 1).is_err());
        let mut machine = UnionFindMachine::new_checked(3).unwrap();
        assert!(machine.begin_union_checked(3, 0).is_err());
        assert!(machine.begin_union_checked(0, 3).is_err());
        machine.begin_union_checked(0, 1).unwrap();
        assert!(machine.begin_union_checked(1, 2).is_err());
    }
}

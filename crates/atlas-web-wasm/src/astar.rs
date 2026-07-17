use std::cmp::Reverse;
use std::collections::BinaryHeap;

use petgraph::Graph;
use petgraph::Undirected;
use petgraph::algo::astar;
use serde::Serialize;
use wasm_bindgen::prelude::*;

const MAX_GRID_CELLS: usize = 160;
const UNREACHED: u32 = u32::MAX;
const NO_PREDECESSOR: usize = usize::MAX;

#[derive(Debug, Serialize)]
struct AstarSnapshot<'a> {
    width: usize,
    height: usize,
    blocked: &'a [bool],
    source: usize,
    goal: usize,
    current: Option<usize>,
    frontier: Vec<bool>,
    closed: &'a [bool],
    path: &'a [usize],
    phase: &'static str,
    found: Option<bool>,
    expansions: u32,
    relaxations: u32,
    last_operation: Option<&'static str>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct AstarMachine {
    width: usize,
    height: usize,
    blocked: Vec<bool>,
    source: usize,
    goal: usize,
    distance: Vec<u32>,
    predecessor: Vec<usize>,
    open: BinaryHeap<Reverse<(u32, u32, usize)>>,
    closed: Vec<bool>,
    current: Option<usize>,
    path: Vec<usize>,
    complete: bool,
    found: Option<bool>,
    expansions: u32,
    relaxations: u32,
    last_operation: Option<&'static str>,
}

#[wasm_bindgen]
impl AstarMachine {
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: usize,
        height: usize,
        blocked: &[u8],
        source: usize,
        goal: usize,
    ) -> Result<AstarMachine, JsError> {
        Self::new_checked(width, height, blocked, source, goal)
            .map_err(|error| JsError::new(&error))
    }

    pub fn step(&mut self) -> bool {
        self.step_once()
    }

    pub fn snapshot_json(&self) -> Result<String, JsError> {
        serde_json::to_string(&self.snapshot())
            .map_err(|error| JsError::new(&format!("cannot serialize A* snapshot: {error}")))
    }

    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.complete
    }
}

impl AstarMachine {
    fn new_checked(
        width: usize,
        height: usize,
        blocked: &[u8],
        source: usize,
        goal: usize,
    ) -> Result<Self, String> {
        validate_grid(width, height, blocked, source, goal)?;
        let cell_count = width * height;
        let mut distance = vec![UNREACHED; cell_count];
        distance[source] = 0;
        let mut open = BinaryHeap::new();
        open.push(Reverse((heuristic(width, source, goal), 0, source)));
        Ok(Self {
            width,
            height,
            blocked: blocked.iter().map(|value| *value != 0).collect(),
            source,
            goal,
            distance,
            predecessor: vec![NO_PREDECESSOR; cell_count],
            open,
            closed: vec![false; cell_count],
            current: None,
            path: Vec::new(),
            complete: false,
            found: None,
            expansions: 0,
            relaxations: 0,
            last_operation: None,
        })
    }

    fn step_once(&mut self) -> bool {
        if self.complete {
            return false;
        }
        let Some((distance, cell)) = self.pop_best() else {
            self.complete = true;
            self.found = Some(false);
            self.current = None;
            self.last_operation = Some("frontier_exhausted");
            return true;
        };
        self.current = Some(cell);
        self.closed[cell] = true;
        self.expansions += 1;
        if cell == self.goal {
            self.path = reconstruct_path(&self.predecessor, self.source, self.goal);
            self.complete = true;
            self.found = Some(true);
            self.last_operation = Some("reconstruct_path");
            return true;
        }
        for neighbor in neighbors(self.width, self.height, cell) {
            if self.blocked[neighbor] || self.closed[neighbor] {
                continue;
            }
            let candidate = distance + 1;
            if candidate < self.distance[neighbor] {
                self.distance[neighbor] = candidate;
                self.predecessor[neighbor] = cell;
                let estimate = candidate + heuristic(self.width, neighbor, self.goal);
                self.open.push(Reverse((estimate, candidate, neighbor)));
                self.relaxations += 1;
            }
        }
        self.last_operation = Some("expand_frontier");
        true
    }

    fn pop_best(&mut self) -> Option<(u32, usize)> {
        while let Some(Reverse((_, distance, cell))) = self.open.pop() {
            if !self.closed[cell] && self.distance[cell] == distance {
                return Some((distance, cell));
            }
        }
        None
    }

    fn snapshot(&self) -> AstarSnapshot<'_> {
        let mut frontier = vec![false; self.width * self.height];
        for Reverse((_, distance, cell)) in &self.open {
            if !self.closed[*cell] && self.distance[*cell] == *distance {
                frontier[*cell] = true;
            }
        }
        AstarSnapshot {
            width: self.width,
            height: self.height,
            blocked: &self.blocked,
            source: self.source,
            goal: self.goal,
            current: self.current,
            frontier,
            closed: &self.closed,
            path: &self.path,
            phase: if self.complete {
                "complete"
            } else {
                "searching"
            },
            found: self.found,
            expansions: self.expansions,
            relaxations: self.relaxations,
            last_operation: self.last_operation,
        }
    }
}

pub fn astar_grid(
    width: usize,
    height: usize,
    blocked: &[u8],
    source: usize,
    goal: usize,
) -> Option<(u32, Vec<usize>)> {
    validate_grid(width, height, blocked, source, goal).ok()?;
    let mut graph = Graph::<(), u32, Undirected>::new_undirected();
    let nodes = (0..width * height)
        .map(|_| graph.add_node(()))
        .collect::<Vec<_>>();
    for cell in 0..width * height {
        if blocked[cell] != 0 {
            continue;
        }
        for neighbor in neighbors(width, height, cell) {
            if blocked[neighbor] == 0 && cell < neighbor {
                graph.add_edge(nodes[cell], nodes[neighbor], 1);
            }
        }
    }
    astar(
        &graph,
        nodes[source],
        |node| node == nodes[goal],
        |edge| *edge.weight(),
        |node| heuristic(width, node.index(), goal),
    )
    .map(|(cost, path)| (cost, path.into_iter().map(|node| node.index()).collect()))
}

fn validate_grid(
    width: usize,
    height: usize,
    blocked: &[u8],
    source: usize,
    goal: usize,
) -> Result<(), String> {
    let Some(cell_count) = width.checked_mul(height) else {
        return Err("grid dimensions overflow".to_owned());
    };
    if width < 2 || height < 2 || cell_count > MAX_GRID_CELLS {
        return Err(format!(
            "grid must be at least 2 by 2 and contain at most {MAX_GRID_CELLS} cells"
        ));
    }
    if blocked.len() != cell_count {
        return Err(format!(
            "blocked map has {} cells but grid requires {cell_count}",
            blocked.len()
        ));
    }
    if source >= cell_count || goal >= cell_count {
        return Err("source and goal must identify grid cells".to_owned());
    }
    if blocked[source] != 0 || blocked[goal] != 0 {
        return Err("source and goal cells cannot be blocked".to_owned());
    }
    Ok(())
}

fn heuristic(width: usize, left: usize, right: usize) -> u32 {
    let (lx, ly) = (left % width, left / width);
    let (rx, ry) = (right % width, right / width);
    (lx.abs_diff(rx) + ly.abs_diff(ry)) as u32
}

fn neighbors(width: usize, height: usize, cell: usize) -> impl Iterator<Item = usize> {
    let x = cell % width;
    let y = cell / width;
    [
        (x > 0).then(|| cell - 1),
        (x + 1 < width).then(|| cell + 1),
        (y > 0).then(|| cell - width),
        (y + 1 < height).then(|| cell + width),
    ]
    .into_iter()
    .flatten()
}

fn reconstruct_path(predecessor: &[usize], source: usize, goal: usize) -> Vec<usize> {
    let mut path = vec![goal];
    let mut current = goal;
    while current != source {
        current = predecessor[current];
        path.push(current);
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finish(machine: &mut AstarMachine) {
        while machine.step_once() {}
    }

    #[test]
    fn incremental_search_matches_petgraph_oracle() {
        let blocked = [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0];
        let mut machine = AstarMachine::new_checked(4, 3, &blocked, 0, 11).unwrap();
        finish(&mut machine);
        let oracle = astar_grid(4, 3, &blocked, 0, 11).unwrap();
        assert_eq!(machine.path.len() as u32 - 1, oracle.0);
        assert_eq!(machine.path.first(), Some(&0));
        assert_eq!(machine.path.last(), Some(&11));
        assert!(machine.found.unwrap());
    }

    #[test]
    fn frontier_and_closed_sets_evolve_without_a_trace() {
        let blocked = [0; 12];
        let mut machine = AstarMachine::new_checked(4, 3, &blocked, 0, 11).unwrap();
        assert!(machine.step_once());
        let snapshot = machine.snapshot();
        assert_eq!(snapshot.current, Some(0));
        assert!(snapshot.closed[0]);
        assert!(snapshot.frontier[1]);
        assert!(snapshot.frontier[4]);
        assert_eq!(snapshot.expansions, 1);
    }

    #[test]
    fn unreachable_goal_is_reported_exactly() {
        let blocked = [0, 1, 0, 1, 0, 1, 0, 1, 0];
        let mut machine = AstarMachine::new_checked(3, 3, &blocked, 0, 2).unwrap();
        finish(&mut machine);
        assert_eq!(machine.found, Some(false));
        assert!(machine.path.is_empty());
        assert_eq!(astar_grid(3, 3, &blocked, 0, 2), None);
    }

    #[test]
    fn rejects_invalid_maps_and_blocked_endpoints() {
        assert!(AstarMachine::new_checked(1, 2, &[0, 0], 0, 1).is_err());
        assert!(AstarMachine::new_checked(2, 2, &[0, 0], 0, 1).is_err());
        assert!(AstarMachine::new_checked(2, 2, &[1, 0, 0, 0], 0, 3).is_err());
    }
}

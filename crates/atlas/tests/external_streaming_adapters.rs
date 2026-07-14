use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn bounded_top_k(values: &[i32], k: usize) -> Vec<i32> {
    let mut retained = BinaryHeap::<Reverse<i32>>::with_capacity(k);
    for &value in values {
        if retained.len() < k {
            retained.push(Reverse(value));
        } else if retained.peek().is_some_and(|minimum| value > minimum.0) {
            retained.pop();
            retained.push(Reverse(value));
        }
    }
    let mut result = retained
        .into_iter()
        .map(|Reverse(value)| value)
        .collect::<Vec<_>>();
    result.sort_unstable_by(|left, right| right.cmp(left));
    result
}

#[derive(Clone, Copy, Debug, Default)]
struct OnlineMoments {
    count: u64,
    mean: f64,
    m2: f64,
}

impl OnlineMoments {
    fn update(&mut self, value: f64) {
        self.count += 1;
        let delta = value - self.mean;
        self.mean += delta / self.count as f64;
        let delta_after = value - self.mean;
        self.m2 += delta * delta_after;
    }

    fn population_variance(self) -> Option<f64> {
        (self.count != 0).then(|| self.m2 / self.count as f64)
    }
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn uniform_below(state: &mut u64, upper: usize) -> usize {
    let upper = upper as u64;
    let acceptance_zone = u64::MAX - (u64::MAX % upper);
    loop {
        let candidate = splitmix64(state);
        if candidate < acceptance_zone {
            return (candidate % upper) as usize;
        }
    }
}

fn reservoir_sample(values: &[i32], capacity: usize, seed: u64) -> Vec<i32> {
    let mut reservoir = values.iter().take(capacity).copied().collect::<Vec<_>>();
    let mut state = seed;
    for (index, &value) in values.iter().enumerate().skip(capacity) {
        let replacement = uniform_below(&mut state, index + 1);
        if replacement < capacity {
            reservoir[replacement] = value;
        }
    }
    reservoir
}

#[derive(Debug)]
struct BloomFilter {
    bits: Vec<bool>,
    hash_count: u32,
}

impl BloomFilter {
    fn new(bit_count: usize, hash_count: u32) -> Self {
        assert!(bit_count > 0);
        assert!(hash_count > 0);
        Self {
            bits: vec![false; bit_count],
            hash_count,
        }
    }

    fn position(bit_count: usize, value: u64, index: u32) -> usize {
        let mut state = value ^ (u64::from(index) << 32);
        (splitmix64(&mut state) % bit_count as u64) as usize
    }

    fn insert(&mut self, value: u64) {
        for index in 0..self.hash_count {
            let position = Self::position(self.bits.len(), value, index);
            self.bits[position] = true;
        }
    }

    fn might_contain(&self, value: u64) -> bool {
        (0..self.hash_count)
            .map(|index| Self::position(self.bits.len(), value, index))
            .all(|position| self.bits[position])
    }
}

#[test]
fn bounded_top_k_is_exact_and_never_exceeds_its_budget() {
    assert_eq!(bounded_top_k(&[4, 1, 8, 3, 8, 2, 7], 3), [8, 8, 7]);
    assert_eq!(bounded_top_k(&[2, 1], 5), [2, 1]);
    assert!(bounded_top_k(&[3, 2, 1], 0).is_empty());
}

#[test]
fn online_moments_match_two_pass_population_statistics() {
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let mut moments = OnlineMoments::default();
    for value in values {
        moments.update(value);
    }

    assert_eq!(moments.count, 8);
    assert_eq!(moments.mean, 5.0);
    assert_eq!(moments.population_variance(), Some(4.0));
}

#[test]
fn online_moments_remain_accurate_for_large_offset_small_spread() {
    let values = [
        1_000_000_000_002.0,
        1_000_000_000_004.0,
        1_000_000_000_004.0,
        1_000_000_000_004.0,
        1_000_000_000_005.0,
        1_000_000_000_005.0,
        1_000_000_000_007.0,
        1_000_000_000_009.0,
    ];
    let reference_mean = values.iter().sum::<f64>() / values.len() as f64;
    let reference_variance = values
        .iter()
        .map(|value| (value - reference_mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    let naive_variance = values.iter().map(|value| value * value).sum::<f64>()
        / values.len() as f64
        - reference_mean * reference_mean;
    let mut moments = OnlineMoments::default();
    for value in values {
        moments.update(value);
    }

    assert_eq!(moments.mean, reference_mean);
    let online_error = (moments.population_variance().unwrap() - reference_variance).abs();
    let naive_error = (naive_variance - reference_variance).abs();
    assert!(online_error < 0.000_1);
    assert!(online_error < naive_error);
}

#[test]
fn reservoir_sample_is_seed_reproducible_and_bounded() {
    let values = (0..100).collect::<Vec<_>>();
    let first = reservoir_sample(&values, 8, 42);
    let repeated = reservoir_sample(&values, 8, 42);
    let other_seed = reservoir_sample(&values, 8, 43);

    assert_eq!(first, repeated);
    assert_ne!(first, other_seed);
    assert_eq!(first.len(), 8);
    assert!(first.iter().all(|value| values.contains(value)));
    assert_eq!(reservoir_sample(&values[..4], 8, 42), values[..4]);
}

#[test]
fn bloom_filter_has_no_false_negatives_and_can_return_false_positives() {
    let inserted = (0..20).collect::<Vec<_>>();
    let mut filter = BloomFilter::new(32, 3);
    for &value in &inserted {
        filter.insert(value);
    }

    assert_eq!(filter.bits.len(), 32);
    assert!(inserted.iter().all(|&value| filter.might_contain(value)));
    assert!((100..10_000).any(|value| filter.might_contain(value)));
}

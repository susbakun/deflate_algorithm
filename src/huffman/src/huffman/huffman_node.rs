use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HuffmanNode {
    pub symbol: Option<u16>,
    pub count: u32,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.count.cmp(&self.count))
    }
}

type MinHeap = BinaryHeap<HuffmanNode>;

impl HuffmanNode {
    pub fn new(counts: &HashMap<u16, u32>) -> Self {
        let mut min_heap = Self::build_min_heap(counts.clone());
        Self::build_tree(&mut min_heap)
    }

    pub fn build_tree(min_heap: &mut MinHeap) -> Self {
        // Handle empty input
        if min_heap.is_empty() {
            return HuffmanNode {
                symbol: None,
                count: 0,
                left: None,
                right: None,
            };
        }

        while min_heap.len() > 1 {
            let left = min_heap.pop().unwrap();
            let right = min_heap.pop().unwrap();

            let parent = HuffmanNode {
                symbol: None,
                count: left.count + right.count,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };

            min_heap.push(parent);
        }

        min_heap.pop().unwrap()
    }

    pub fn build_min_heap(counts: HashMap<u16, u32>) -> MinHeap {
        let mut min_heap = BinaryHeap::new();

        // Sort by symbol value to ensure deterministic insertion order
        let mut sorted_counts: Vec<(u16, u32)> = counts.into_iter().collect();
        sorted_counts.sort_by_key(|(symbol, _)| *symbol);

        for (symbol, count) in sorted_counts {
            let node = HuffmanNode {
                symbol: Some(symbol),
                count,
                left: None,
                right: None,
            };

            min_heap.push(node);
        }

        min_heap
    }
}

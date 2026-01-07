use std::collections::{BinaryHeap, HashMap};


#[derive(PartialEq, Eq, Debug)]
pub struct HuffmanNode {
    pub byte: Option<u8>,
    pub count: u16,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
}


impl Ord for HuffmanNode{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for HuffmanNode{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.count.cmp(&self.count))
    }
}

type MinHeap = BinaryHeap<HuffmanNode>;

impl HuffmanNode {
    pub fn new(counts: &HashMap<u8, u16>) -> Self{
        let mut min_heap = Self::build_min_heap(counts.clone());
        Self::build_tree(&mut min_heap)
    }

    pub fn build_tree(min_heap: &mut MinHeap) -> Self{
        // Handle empty input
        if min_heap.is_empty() {
            return HuffmanNode {
                byte: None,
                count: 0,
                left: None,
                right: None,
            };
        }

        while min_heap.len() > 1 {
            let left = min_heap.pop().unwrap();
            let right = min_heap.pop().unwrap();

            let parent = HuffmanNode {
                byte: None,
                count: left.count + right.count,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };

            min_heap.push(parent);
        }

        min_heap.pop().unwrap()
    }

    pub fn build_min_heap(counts: HashMap<u8, u16>) -> MinHeap {
        let mut min_heap = BinaryHeap::new();

        // Sort by byte value to ensure deterministic insertion order
        let mut sorted_counts: Vec<(u8, u16)> = counts.into_iter().collect();
        sorted_counts.sort_by_key(|(byte, _)| *byte);

        for (byte, count) in sorted_counts {
            let node = HuffmanNode {
                byte: Some(byte),
                count,
                left: None,
                right: None
            };

            min_heap.push(node);
        }

        min_heap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars_basic() {
        let input = b"aaaabbc".to_vec();
        let counts = HuffmanNode::count_chars(&input);
        
        assert_eq!(counts.len(), 3);
        assert_eq!(counts.get(&b'a'), Some(&4));
        assert_eq!(counts.get(&b'b'), Some(&2));
        assert_eq!(counts.get(&b'c'), Some(&1));
    }

    #[test]
    fn test_count_chars_empty() {
        let input: Vec<u8> = vec![];
        let counts = HuffmanNode::count_chars(&input);
        
        assert_eq!(counts.len(), 0);
    }

    #[test]
    fn test_build_tree_root_properties() {
        let mut counts = HashMap::new();
        counts.insert(b'a', 4);
        counts.insert(b'b', 2);
        counts.insert(b'c', 1);
        
        let mut heap = HuffmanNode::build_min_heap(counts);
        let root = HuffmanNode::build_tree(&mut heap);
        
        // Root should have total count
        assert_eq!(root.count, 7);
        // Root should be internal node
        assert_eq!(root.byte, None);
        // Root should have children
        assert!(root.left.is_some());
        assert!(root.right.is_some());
    }

    #[test]
    fn test_build_tree_single_char() {
        let mut counts = HashMap::new();
        counts.insert(b'a', 5);
        let mut heap = HuffmanNode::build_min_heap(counts);
        let root = HuffmanNode::build_tree(&mut heap);
        
        // Single character tree
        assert_eq!(root.count, 5);
        assert_eq!(root.byte, Some(b'a'));
    }
}
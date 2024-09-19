use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// A wrapper to store ListNode references in a min-heap, since BinaryHeap is a max-heap by default.
#[derive(Clone, Eq, PartialEq)]
struct HeapNode {
    node: Box<ListNode>, // Store Boxed ListNode for heap
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order to make it a min-heap
        other.node.val.cmp(&self.node.val)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap: BinaryHeap<HeapNode> = BinaryHeap::new();  // Explicit type annotation

    // Push the head of each list into the heap
    for list in lists {
        if let Some(node) = list {
            heap.push(HeapNode { node });
        }
    }

    let mut dummy: Box<ListNode> = Box::new(ListNode::new(0)); // Explicit type annotation
    let mut current: &mut Box<ListNode> = &mut dummy; // Explicit type annotation

    // Process nodes until the heap is empty
    while let Some(HeapNode { mut node }) = heap.pop() {
        // Append the smallest node to the result list
        current.next = Some(node.clone());
        current = current.next.as_mut().unwrap();

        // If the node has a next, push it into the heap
        if let Some(next) = node.next.take() {
            heap.push(HeapNode { node: next });
        }
    }

    dummy.next
}

// Helper function to convert a vector of integers to a linked list
fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;  // Explicit type annotation
    let mut current: &mut Option<Box<ListNode>> = &mut head;  // Explicit type annotation

    for &val in &v {
        let new_node = Box::new(ListNode::new(val));
        if let Some(ref mut current_node) = current {
            current_node.next = Some(new_node);
            current = &mut current_node.next;
        } else {
            *current = Some(new_node);
        }
    }

    head
}

// Function to read input lists from the user
fn read_input_lists() -> Vec<Option<Box<ListNode>>> {
    let mut lists: Vec<Option<Box<ListNode>>> = Vec::new();  // Explicit type annotation

    println!("Enter the number of linked lists:");
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("Failed to read line");
    let num_lists: usize = line.trim().parse().expect("Please enter a valid number");

    for i in 0..num_lists {
        println!("Enter sorted integers for list {} separated by spaces:", i + 1);
        line.clear();
        stdin.lock().read_line(&mut line).expect("Failed to read line");
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Please enter valid integers"))
            .collect();
        let list = vec_to_list(nums);
        lists.push(list);
    }

    lists
}

// Function to print the merged linked list
fn print_list(list: Option<Box<ListNode>>) {
    let mut current = list;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = node.next;
    }
    println!("None");
}

fn main() {
    // Read input lists from the user
    let lists = read_input_lists();
    // Merge the lists
    let merged = merge_k_lists(lists);

    // Output the merged list
    println!("Merged sorted linked list:");
    print_list(merged);
}

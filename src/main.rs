extern crate min_heap;
use min_heap::MinHeap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::binary_heap::BinaryHeap;


fn main() {
    let file = File::open("/home/chris/Workspace/rust/min_heap/median.txt").unwrap();
    let reader = BufReader::new(file);

    let mut min_heap: MinHeap<i32> = MinHeap::new();
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut sum_of_medians = 0;

    for line in reader.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();

        if min_heap.is_empty() && max_heap.is_empty() {
            min_heap.push(num); // arbitrarily add new number to min_heap
        } else if max_heap.is_empty() {
            if num < *min_heap.peek().unwrap() {
                max_heap.push(num); 
            } else {
                min_heap.push(num);
            }
        } else { // if min_heap is empty or neither are empty
            if num > *max_heap.peek().unwrap() {
                min_heap.push(num);
            } else {
                max_heap.push(num);
            }
        } 

        println!("Size of min heap: {}, size of max heap: {}", max_heap.len(), min_heap.len());
        if max_heap.len() > min_heap.len() {
            if max_heap.len() - min_heap.len() > 1 {
                let rebalance_item = max_heap.pop().unwrap();
                min_heap.push(rebalance_item);
            }
        }
        if min_heap.len() > max_heap.len() {
            if min_heap.len() - max_heap.len() > 1 {
                let rebalance_item = min_heap.pop().unwrap();
                max_heap.push(rebalance_item);
            }
        }
        let mut median: i32;
        if min_heap.len() > max_heap.len() {
            median = *min_heap.peek().unwrap();
        } else {
            median = *max_heap.peek().unwrap();
        }
        sum_of_medians += median;
    }
    println!("Sum of medians mod 10000 is {}", sum_of_medians % 10000);

}

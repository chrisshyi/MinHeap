use std::vec::Vec;
use std::marker::Copy;

///
/// Implements a minimum heap 
/// Supports:
///     push in O(log n)
///     peek in O(1)
///     pop in O(log n)
/// 
pub struct MinHeap<T: Ord + Copy> {
    data: Vec<T>,
}

impl <T: Ord + Copy> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap {
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
        
    fn get_parent_index(&self, index: usize) -> usize {
        if index % 2 == 0 {
            return (index - 2) / 2;
        } 
        (index - 1) / 2
    }

    fn get_children_index(&self, index: usize) -> (usize, usize) {
        (2 * index + 1, 2 * index + 2)
    }

    fn sink_down(&mut self, index: usize) {
        let (child1_index, child2_index) = self.get_children_index(index);
        // node has no children
        if child1_index >= self.len() {
            return;
        }

        let node_to_sink = self.data[index];
        let child1 = self.data[child1_index];

        if child2_index < self.len() {
            let child2 = self.data[child2_index]; 
            if node_to_sink > child1 && node_to_sink > child2 {
                if child1 < child2 {
                    self.data.swap(index, child1_index);
                    self.sink_down(child1_index);
                } else {
                    self.data.swap(index, child2_index);
                    self.sink_down(child2_index);
                }
            } else if node_to_sink > child1 {
                self.data.swap(index, child1_index);
                self.sink_down(child1_index);
            } else if node_to_sink > child2 {
                self.data.swap(index, child2_index);
                self.sink_down(child2_index);
            } 
        } else {
            if node_to_sink > child1 {
                self.data.swap(index, child1_index);
                self.sink_down(child1_index);
            }
        }
    }

    fn bubble_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let parent_index = self.get_parent_index(index);
        let parent = self.data[parent_index];

        let node_to_bubble = self.data[index];
        if node_to_bubble < parent {
            self.data.swap(index, parent_index);
            self.bubble_up(parent_index);
        } 
    }


    pub fn push(&mut self, item: T) {
        // add new element to the end of the vector
        self.data.push(item);
        let end_index = self.data.len() - 1;
        // bubble the new item up
        self.bubble_up(end_index);
    }

    pub fn heapify(&mut self, data: Vec<T>) {
        for datum in data.iter() {
            let to_push = *datum;
            self.push(to_push);
        }
    }

    // Optional operation
    // pub fn delete(&mut self, item: T) {

    // }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let end_index = self.data.len() - 1;
        self.data.swap(0, end_index);
        let return_val = self.data.pop();
        self.sink_down(0);
        return_val
    }

    pub fn clear(&mut self) {
        self.data = Vec::new();
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_heap_insertion() {
        let mut min_heap: MinHeap<i32> = MinHeap::new();
        let data_vec = vec![19238, 477, 827, 19, 25, 3, 1];
        for item in data_vec.iter() {
            let to_push = *item; // copy the item
            min_heap.push(to_push);

            if to_push == 477 {
                assert_eq!(*min_heap.peek().unwrap(), 477);
            } else if to_push == 25 {
                assert_eq!(*min_heap.peek().unwrap(), 19);
            } else if to_push == 3 {
                assert_eq!(*min_heap.peek().unwrap(), 3);
            } else if to_push == 1 {
                assert_eq!(*min_heap.peek().unwrap(), 1);
            }
        }
    }

    #[test]
    fn test_heapify() {
        let mut min_heap: MinHeap<i32> = MinHeap::new();
        let data_vec = vec![19238, 477, 827, 19, 25, 3, 1];
        min_heap.heapify(data_vec);

        assert_eq!(min_heap.len(), 7);
        assert_eq!(*min_heap.peek().unwrap(), 1);
    }

    #[test]
    fn test_extract_min() {
        let mut min_heap: MinHeap<i32> = MinHeap::new();
        let data_vec = vec![19238, 477, 827, 19, 25, 3, 1];
        min_heap.heapify(data_vec);

        let mut popped = min_heap.pop().unwrap();
        assert_eq!(popped, 1);
        
        min_heap.pop();
        popped = min_heap.pop().unwrap();
        assert_eq!(popped, 19);

        min_heap.pop();
        popped = min_heap.pop().unwrap();
        assert_eq!(popped, 477);
        assert_eq!(min_heap.len(), 2);
    }
}
use std::vec::Vec;

///
/// Implements a minimum heap 
/// Supports:
///     push in O(log n)
///     peek in O(1)
///     pop in O(log n)
///     delete in O(n)
/// 
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl <T: Ord> MinHeap<T> {
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
        let child2_op = self.data.get(child2_index);

        if child2_op.is_some() {
            let child2 = *child2_op.unwrap();
            if node_to_sink < child1 && node_to_sink < child2 {
                if child1 < child2 {
                    self.data.swap(index, child1_index);
                    self.sink_down(child1_index);
                } else {
                    self.data.swap(index, child2_index);
                    self.sink_down(child2_index);
                }
            } else if node_to_sink < child1 {
                self.data.swap(index, child1_index);
                self.sink_down(child1_index);
            } else if node_to_sink < child2 {
                self.data.swap(index, child2_index);
                self.sink_down(child2_index);
            } 
        } else {
            if node_to_sink < child1 {
                self.data.swap(index, child1_index);
                self.sink_down(child1_index);
            }
        }
    }

    fn bubble_up(&mut self, index: usize) {
        let parent_index = self.get_parent_index(index);
        let parent = self.data[parent_index];

        let node_to_bubble = self.data[index];
        if node_to_bubble < parent {
            self.data.swap(index, parent_index);
            self.bubble_up(parent_index);
        } 
    }


    pub fn push(&mut self, item: T) {

    }

    pub fn delete(&mut self, item: T) {

    }

    pub fn peek(&self) -> Option<T> {

    }

    pub fn pop(&mut self) -> Option<T> {

    }

    pub fn clear(&mut self) {
        self.data = Vec::new();
    }
}
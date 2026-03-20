pub mod demo;

use crate::common::{GarbageCollector, Heap};
use std::collections::VecDeque;

pub struct MarkSweep {
    pub heap: Heap,
}

impl MarkSweep {
    pub fn new() -> Self {
        MarkSweep { heap: Heap::new() }
    }

    fn mark(&mut self, roots: &[usize]) {
        // We use a queue for BFS traversal (like your drawing — follow arrows)
        let mut queue: VecDeque<usize> = VecDeque::new();

        // Seed the queue with all roots (these are your 'a' and 'b' from the drawing)
        for &root_id in roots {
            if let Some(obj) = self.heap.objects.get(&root_id) {
                if !obj.marked {
                    queue.push_back(root_id);
                }
            }
        }

        // Walk the object graph — keep following children until nothing left
        while let Some(id) = queue.pop_front() {
            if let Some(obj) = self.heap.objects.get_mut(&id) {
                if obj.marked {
                    // already visited, skip
                    continue;
                }
                // Mark this object as reachable
                obj.marked = true;

                // Collect children ids first (to avoid borrow conflict)
                let children: Vec<usize> = obj.children.clone();

                // Push all children into the queue to visit next
                for child_id in children {
                    if let Some(child) = self.heap.objects.get(&child_id) {
                        if !child.marked {
                            queue.push_back(child_id);
                        }
                    }
                }
            }
        }
    }

    // PHASE 2 — SWEEP
    // Walk every object in the heap — if not marked, it's garbage, delete it
    fn sweep(&mut self) -> usize {
        let garbage: Vec<usize> = self
            .heap
            .objects
            .values()
            .filter(|obj| !obj.marked)
            .map(|obj| obj.id)
            .collect();

        let removed_count = garbage.len();
        for id in &garbage {
            self.heap.objects.remove(id);
        }

        println!("  [sweep] removed {} unreachable objects", removed_count);

        // Reset marks on surviving objects for the next cycle
        for obj in self.heap.objects.values_mut() {
            obj.marked = false;
        }
        removed_count
    }
}

impl GarbageCollector for MarkSweep {
    fn allocate(&mut self, size: usize, children: Vec<usize>) -> usize {
        self.heap.allocate(size, children)
    }

    fn collect(&mut self, roots: &[usize]) -> usize {
        println!("  [mark]  traversing from {} root(s)...", roots.len());
        self.mark(roots);

        let marked_count = self.heap.objects.values().filter(|o| o.marked).count();
        println!("  [mark]  found {} reachable objects", marked_count);

        self.sweep()
    }

    fn live_objects(&self) -> usize {
        self.heap.objects.len()
    }
}

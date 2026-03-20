// why am i adding this ?
// Rust is already a language which manages the memory so it wont let me use the real heap pointers
// so i am stimulating a heap in safe rust

use std::collections::HashMap;

// this is a raw object node stimulated heap
pub struct Object {
    pub id: usize, // You can't use real memory addresses, so IDs are your pointers. When object A references object B, it stores B's id. This is how you simulate a pointer graph without raw pointers.
    pub size: usize, // Simulates that objects take up memory. Lets you track "how much memory is in use" and report meaningful stats.

    // The children feild is the very intresting part , its the object graph cuz the gc algorithm need to traverse from object to object . In real time it would be the raw pointer feild inside the struct . Here it is the list of IDs
    pub children: Vec<usize>, // pointer to other objects by ID

    // When you do DFS/BFS during collection, you follow these.
    pub marked: bool,           // for mark sweep tracing
    pub reference_count: usize, // for reference counting

    pub generation: u8,                // for generation GC
    pub forwarding_ptr: Option<usize>, // for cheney's algorithm
}

// This is the heap where all the objects are stored
pub struct Heap {
    // In real life a heap is a contiguous block of memory. Here i am simulating  it with a map because:
    // 1. I need O(1) lookup by ID (following a "pointer" = objects.get(id))
    // 2. I need easy deletion (sweeping dead objects = objects.remove(id))
    // 3. Rust isnt letting me to have a Vec of objects that point into themselves with raw indices safely when i am also mutating the Vec
    pub objects: HashMap<usize, Object>,

    // next_id is my bump allocator — every new object gets the next integer.
    pub next_id: usize,
}

pub trait GarbageCollector {
    fn allocate(&mut self, size: usize, children: Vec<usize>) -> usize;
    fn collect(&mut self, roots: &[usize]) -> usize;
    fn live_objects(&self) -> usize;
}

// Implementing the Objects and the Heap

impl Object {
    pub fn new(id: usize, size: usize, children: Vec<usize>) -> Self {
        Self {
            id,
            size,
            children,
            marked: false,
            reference_count: 0,
            generation: 0,
            forwarding_ptr: None,
        }
    }
}

impl Heap {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn allocate(&mut self, size: usize, children: Vec<usize>) -> usize {
        let id = self.next_id;
        self.objects.insert(id, Object::new(id, size, children));
        self.next_id += 1;
        id
    }

    pub fn get_object(&self, id: usize) -> Option<&Object> {
        self.objects.get(&id)
    }
}

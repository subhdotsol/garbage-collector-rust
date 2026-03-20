use crate::common::GarbageCollector;
use crate::mark_sweep::MarkSweep;

pub fn run() {
    demo_basic();
    demo_chain();
}

// Demo 1: your exact drawing from the whiteboard
// a → neha, b → tiya, brianna is floating garbage
fn demo_basic() {
    println!("-- demo 1: Basic Mark and Sweep --\n");

    let mut gc = MarkSweep::new();

    let tiya = gc.allocate(16, vec![]); // id = 0
    let neha = gc.allocate(32, vec![tiya]); // id = 1, neha points to tiya
    let brianna = gc.allocate(8, vec![]); // id = 2, nobody points here

    println!("allocated:");
    println!("  tiya   id={}", tiya);
    println!("  neha   id={} → points to tiya({})", neha, tiya);
    println!("  brianna id={} → nobody points here", brianna);
    println!("  heap size before GC: {}\n", gc.live_objects());

    // a = neha, b = tiya (your stack variables / roots)
    let roots = vec![neha, tiya];
    println!("roots (stack variables): {:?}\n", roots);

    gc.collect(&roots);

    println!("\nheap size after GC: {}", gc.live_objects());
    println!(
        "brianna(id={}) collected: {}",
        brianna,
        !gc.heap.objects.contains_key(&brianna)
    );
}

// Demo 2: chain — a → b → c → d, and e floats
// shows that reachability travels through the whole chain
fn demo_chain() {
    println!("\n-- demo 2: chain a → b → c → d, e is garbage --\n");

    let mut gc = MarkSweep::new();

    let d = gc.allocate(10, vec![]);
    let c = gc.allocate(10, vec![d]);
    let b = gc.allocate(10, vec![c]);
    let a = gc.allocate(10, vec![b]);
    let e = gc.allocate(10, vec![]); // garbage — no root points here

    println!("allocated: a={} b={} c={} d={} e={}", a, b, c, d, e);
    println!("heap size before GC: {}\n", gc.live_objects());

    // only 'a' is a root — but a→b→c→d all survive through the chain
    println!("roots: [a={}] only\n", a);
    gc.collect(&[a]);

    println!("\nheap size after GC: {}", gc.live_objects());
    println!(
        "e(id={}) collected: {}",
        e,
        !gc.heap.objects.contains_key(&e)
    );
    println!(
        "d(id={}) survived:  {}",
        d,
        gc.heap.objects.contains_key(&d)
    );
}

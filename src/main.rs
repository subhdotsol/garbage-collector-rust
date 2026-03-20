mod common;
mod mark_sweep;
// mod ref_count;
// mod generational;
// mod cheney;
// mod tracing;

fn main() {
    println!("Approach 1: Mark and Sweep \n");
    mark_sweep::demo::run();
    println!("\n──────────────────────────────────\n");

    // will be implementing more soon
}

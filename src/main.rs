use aoc_runner_derive::aoc_main;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

aoc_main! { lib = aoc_2021 }

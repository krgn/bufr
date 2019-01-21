#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate criterion;
extern crate bufr;
extern crate rand;

use criterion::{Criterion, Fun};
use rand::{Rng, SeedableRng, XorShiftRng};

// macro_rules! insert_lookup {
//     ($fn:ident, $s:expr) => {
//         fn $fn(c: &mut Criterion) {
//             let naive = Fun::new("naive", |b, i| b.iter(|| insert_and_lookup_naive(*i)));
//             let standard = Fun::new("standard", |b, i| b.iter(|| insert_and_lookup_standard(*i)));

//             let functions = vec![naive, standard];

//             c.bench_functions(&format!("HashMap/{}", $s), functions, &$s);
//         }
//     };
// }

// insert_lookup!(insert_lookup_100000, 100_000);
// insert_lookup!(insert_lookup_10000, 10_000);
// insert_lookup!(insert_lookup_1000, 1_000);
// insert_lookup!(insert_lookup_100, 100);
// insert_lookup!(insert_lookup_10, 10);
// insert_lookup!(insert_lookup_1, 1);

// criterion_group! {
//     name = benches;
//     config = Criterion::default();
//     targets = insert_lookup_100000, insert_lookup_10000, insert_lookup_1000, insert_lookup_100, insert_lookup_10, insert_lookup_1
// }

// criterion_main!(benches);

fn main() {
    println!("mspc benches")
}

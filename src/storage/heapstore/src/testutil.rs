use crate::heap_page::HeapPage;
use crate::page::Page;
use common::prelude::*;
use common::testutil::*;
use rand::rngs::SmallRng;
use rand::Rng;
use std::hint::black_box;

pub fn bench_page_insert(vals: &[Vec<u8>]) {
    let mut p = Page::new(0);
    for i in vals {
        p.add_value(i).unwrap();
    }
}

pub enum BenchOp {
    Insert(Vec<u8>),
    Delete(SlotId),
    Update(SlotId, Vec<u8>),
    Read(SlotId),
    Scan,
}

pub fn gen_page_bench_workload(
    rng: &mut SmallRng,
    num_ops: usize,
    min_size: usize,
    max_size: usize,
) -> Vec<BenchOp> {
    let mut res = Vec::new();
    let mut random_bytes = get_random_vec_of_byte_vec(rng, num_ops, min_size, max_size);
    let mut expected_max_slot = 0;
    let seed_insert = 5;
    // Seed the first SEED_INSERT ops to be inserts
    for _ in 0..seed_insert {
        expected_max_slot += 1;
        res.push(BenchOp::Insert(random_bytes.pop().unwrap()));
    }
    for _ in seed_insert..num_ops {
        let op = match rng.random_range(0..100) {
            0..20 => {
                expected_max_slot += 1;
                BenchOp::Insert(random_bytes.pop().unwrap())
            }
            20..30 => BenchOp::Delete(rng.random_range(0..expected_max_slot)),
            30..50 => BenchOp::Update(
                rng.random_range(0..expected_max_slot),
                random_bytes.pop().unwrap(),
            ),
            50..60 => BenchOp::Scan,
            _ => BenchOp::Read(rng.random_range(0..expected_max_slot)),
        };
        res.push(op);
    }
    res
}

pub fn bench_page_mixed(workload: &Vec<BenchOp>) {
    let mut p = Page::new(23500);
    p.init_heap_page();
    for op in workload {
        match op {
            BenchOp::Insert(v) => {
                let res = p.add_value(v);
                black_box(res);
            }
            BenchOp::Delete(sid) => {
                let res = p.delete_value(*sid);
                black_box(res);
            }
            BenchOp::Update(sid, v) => {
                let res = p.update_value(*sid, v);
                black_box(res);
            }
            BenchOp::Read(sid) => {
                let res = p.get_value(*sid);
                black_box(res);
            }
            BenchOp::Scan => {
                for (i, slot) in p.iter() {
                    black_box(i);
                    black_box(slot);
                }
            }
        }
    }
}

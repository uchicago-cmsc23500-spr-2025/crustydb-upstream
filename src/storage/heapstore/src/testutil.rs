use crate::heap_page::HeapPage;
use crate::page::Page;

pub fn bench_page_insert(vals: &[Vec<u8>]) {
    let mut p = Page::new(0);
    for i in vals {
        p.add_value(i).unwrap();
    }
}

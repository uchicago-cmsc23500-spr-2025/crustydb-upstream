use crate::{CrustyError, TableSchema, Tuple};

pub trait TupleConverterTrait {
    fn new(schema: TableSchema) -> Self;
    fn write_tuple(&self, tuple: &Tuple, buf: &mut [u8], offset: usize) -> Option<usize>;
    fn read_tuple(&self, buf: &[u8], offset: usize, len: usize) -> Result<Tuple, CrustyError>;
}

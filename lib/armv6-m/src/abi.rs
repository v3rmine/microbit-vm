use std::rc::Rc;

pub trait MemoryMutation {
    fn apply(on: &mut [u8]);
    fn rollback(on: &mut [u8]);
}

pub trait Runtime
where
    Self: Sized,
{
    type Error: std::error::Error;
    type Mutation: MemoryMutation;

    fn init() -> Self;

    fn load_file(&mut self, path: &str) -> Result<&mut Self, Self::Error>;
    fn load_hex(&mut self, hex: &str) -> Result<&mut Self, Self::Error>;
    fn load_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, Self::Error>;

    fn get_memory(&self) -> Rc<[u8]>;
    fn get_mutations_history(&self) -> Rc<[Self::Mutation]>;

    fn get_memory_at_mutation(&self, idx: usize) -> Vec<u8>;

    fn step(&mut self) -> Self;
    fn run(&mut self) -> Self;
}

pub trait RuntimeExtras
where
    Self: Runtime,
{
    fn apply_mutation(
        &mut self,
        mutation: <Self as Runtime>::Mutation,
    ) -> Result<&mut Self, <Self as Runtime>::Error>;
    fn rollback_last_mutation(&mut self) -> Result<&mut Self, <Self as Runtime>::Error>;
}

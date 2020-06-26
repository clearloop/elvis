/// data format transfer
pub trait Serde<S, T, E> {
    fn de(h: T) -> Result<S, E>;
    fn ser(&self) -> T;
}

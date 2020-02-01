use crate::Text;
use elvis::Text as ElvisText;
use std::convert::Into;

pub trait ProtoType<P> {
    fn prototype(self) -> P;
}

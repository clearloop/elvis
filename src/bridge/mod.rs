mod layout;
mod macros;
mod widgets;

use crate::{layout::*, widgets::*, Error, Tree};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// data format transfer
pub trait Serde<S, T> {
    fn de(h: T) -> Result<S, Error>;
    fn ser(&self) -> T;
}

sw! {
    Align,
    Container,
    Flex,
    SizedBox,
}

mcw! {
    Col,
    List,
    Row,
}

mcws! {
    Grid,
    MultiColumn,
}

it! {
    Center,
    Col,
    Row,
    Image,
    Text,
    List,
}

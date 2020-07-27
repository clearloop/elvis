mod layouts;
mod macros;
mod widgets;

use crate::widgets::{layouts::*, *};
use elvis_core::{Class, Node};
use std::{cell::RefCell, rc::Rc};

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

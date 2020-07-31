mod layouts;
mod macros;
mod widgets;

use crate::widgets::{layouts::*, *};
use elvis_core::{Class, Node};
use std::{cell::RefCell, rc::Rc};

it! {
    Center,
    Image,
    Text,
}

into_node! {[
    Align,
    Container,
    SizedBox,
],[
    Flex,
],[
    Col,
    List,
    Row,
],[
    Grid,
    MultiColumn,
]}

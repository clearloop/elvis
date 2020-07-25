mod layouts;
mod macros;
mod widgets;

use crate::{
    widgets::{layouts::*, *},
    Node,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

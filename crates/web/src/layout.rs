use crate::{
    AlignStyle, ContainerStyle, FlexStyle, GridStyle, MultiColumnStyle, SizedBoxStyle, Widget,
};
use elvis::widgets::layouts::{
    Align, Center, Col as ElvisCol, Container, Flex, Grid as ElvisGrid, List as ElvisList,
    MultiColumn as ElvisMultiColumn, Row as ElvisRow, SizedBox,
};
use wasm_bindgen::prelude::*;

/// `List` is a set of poor orphan children, dzn`t have any style, just blowing in the wind.
#[wasm_bindgen]
pub struct List(ElvisList);

#[wasm_bindgen]
impl List {
    #[wasm_bindgen(constructor)]
    pub fn new() -> List {
        List(ElvisList { children: vec![] })
    }

    pub fn push(&mut self, widget: Widget) {
        self.0.children.push(widget.into())
    }

    pub fn widget(self) -> Widget {
        Widget::new(self.0)
    }
}

/// `Col` is the typical flow in html, with flexible enhance.
#[wasm_bindgen]
pub struct Col(ElvisCol);

#[wasm_bindgen]
impl Col {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Col {
        Col(ElvisCol { children: vec![] })
    }

    pub fn push(&mut self, widget: Widget) {
        self.0.children.push(widget.into())
    }

    pub fn widget(self) -> Widget {
        Widget::new(self.0)
    }
}

/// Both `Col` and `Row` are using flex-start, if you want to reverse the children of them, better to work on the list order.
#[wasm_bindgen]
pub struct Row(ElvisRow);

#[wasm_bindgen]
impl Row {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Row {
        Row(ElvisRow { children: vec![] })
    }

    pub fn push(&mut self, widget: Widget) {
        self.0.children.push(widget.into())
    }

    pub fn widget(self) -> Widget {
        Widget::new(self.0)
    }
}

/// `Grid` is quite complex in some way, usually, we just `Grid` our contains.
#[wasm_bindgen]
pub struct Grid(ElvisGrid);

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(style: Option<GridStyle>) -> Grid {
        Grid(ElvisGrid {
            children: vec![],
            style: style.unwrap_or_default().into(),
        })
    }

    pub fn push(&mut self, widget: Widget) {
        self.0.children.push(widget.into())
    }

    pub fn widget(self) -> Widget {
        Widget::new(self.0)
    }
}

/// **Homework**: code a New York Times.
#[wasm_bindgen]
pub struct MultiColumn(ElvisMultiColumn);

#[wasm_bindgen]
impl MultiColumn {
    #[wasm_bindgen(constructor)]
    pub fn new(style: Option<MultiColumnStyle>) -> MultiColumn {
        MultiColumn(ElvisMultiColumn {
            children: vec![],
            style: style.unwrap_or_default().into(),
        })
    }

    pub fn push(&mut self, widget: Widget) {
        self.0.children.push(widget.into())
    }

    pub fn widget(self) -> Widget {
        Widget::new(self.0)
    }
}

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
#[wasm_bindgen(js_name = "Container")]
pub fn container(child: Widget, style: Option<ContainerStyle>) -> Widget {
    Widget::new(Container {
        child: child.into(),
        style: style.unwrap_or_default().into(),
    })
}

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
#[wasm_bindgen(js_name = "SizedBox")]
pub fn sized_box(child: Widget, style: Option<SizedBoxStyle>) -> Widget {
    Widget::new(SizedBox {
        child: child.into(),
        style: style.unwrap_or_default().into(),
    })
}

/// `Align` inherits the core usage of Alignments, it's quite simple, just one property.
#[wasm_bindgen(js_name = "Align")]
pub fn align(child: Widget, style: Option<AlignStyle>) -> Widget {
    Widget::new(Align {
        child: child.into(),
        style: style.unwrap_or_default().into(),
    })
}

/// `Center` is a very nice widget, if your website only have a line of chars, use it!
#[wasm_bindgen(js_name = "Center")]
pub fn center(child: Widget) -> Widget {
    Widget::new(Center {
        child: child.into(),
    })
}

/// This is the Lunatic Widget to Ground Control, 'I`m stepping throw the Window.'
#[wasm_bindgen(js_name = "Flex")]
pub fn flex(child: Widget, style: Option<FlexStyle>) -> Widget {
    Widget::new(Flex {
        child: child.into(),
        style: style.unwrap_or_default().into(),
    })
}

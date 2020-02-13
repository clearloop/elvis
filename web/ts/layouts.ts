import {
  Align as ElvisAlign,
  AlignStyle,
  Col as ElvisCol,
  Container as ElvisContainer,
  ContainerStyle,
  Flex as ElvisFlex,
  FlexStyle,
  Grid as ElvisGrid,
  GridStyle,
  IAlignStyle,
  IContainerStyle,
  IFlexStyle,
  IGridStyle,
  ISizedBoxStyle,
  List as ElvisList,
  Row as ElvisRow,
  SizedBox as ElvisSizedBox,
  SizedBoxStyle,
  Widget,
} from "../pkg";

export function Align(widget: Widget, style: IAlignStyle): Widget {
  return ElvisAlign(widget, new AlignStyle(style.align));
}

export function Col(widgets: Widget[]): Widget {
  const col = new ElvisCol();
  for (const i in widgets) {
    if (widgets[i] !== undefined) {
      col.push(widgets[i]);
    }
  }

  return col.widget();
}

export function Container(widget: Widget, style: IContainerStyle): Widget {
  return ElvisContainer(widget, new ContainerStyle(
    style.align,
    style.color,
    style.height,
    style.margin,
    style.padding,
    style.width,
  ));
}

export function Flex(widget: Widget, style: IFlexStyle): Widget {
  return ElvisFlex(widget, new FlexStyle(
    style.align,
    style.basis,
    style.direction,
    style.grow,
    style.order,
    style.wrap,
  ));
}

export function Grid(widgets: Widget[], style: IGridStyle): Widget {
  const grid = new ElvisGrid(new GridStyle(
    style.auto_rows,
    style.col,
    style.gap,
    style.row,
    style.template_col,
    style.template_row,
  ));

  for (const i in widgets) {
    if (widgets[i] !== undefined) {
      grid.push(widgets[i]);
    }
  }

  return grid.widget();
}

export function List(widgets: Widget[]): Widget {
  const list = new ElvisList();
  for (const i in widgets) {
    if (widgets[i] !== undefined) {
      list.push(widgets[i]);
    }
  }

  return list.widget();
}

export function Row(widgets: Widget[]): Widget {
  const row = new ElvisRow();
  for (const i in widgets) {
    if (widgets[i] !== undefined) {
      row.push(widgets[i]);
    }
  }

  return row.widget();
}

export function SizedBox(widget: Widget, style: ISizedBoxStyle): Widget {
  return ElvisSizedBox(widget, new SizedBoxStyle(style.height, style.width));
}

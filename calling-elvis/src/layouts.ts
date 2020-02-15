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
  IMultiColumnStyle,
  ISizedBoxStyle,
  List as ElvisList,
  MultiColumn as ElvisMultiColumn,
  MultiColumnStyle,
  Row as ElvisRow,
  SizedBox as ElvisSizedBox,
  SizedBoxStyle,
  Widget,
} from "elvis-web";

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

export function Container(widget: Widget, style?: IContainerStyle): Widget {
  if (style === undefined) {
    style = {};
  }

  return ElvisContainer(widget, new ContainerStyle(
    style.align,
    style.color,
    style.height,
    style.margin,
    style.padding,
    style.width,
  ));
}

export function Flex(widget: Widget, style?: IFlexStyle): Widget {
  if (style === undefined) {
    style = {};
  }

  return ElvisFlex(widget, new FlexStyle(
    style.align,
    style.basis,
    style.direction,
    style.grow,
    style.order,
    style.wrap,
  ));
}

export function Grid(widgets: Widget[], style?: IGridStyle): Widget {
  if (style === undefined) {
    style = {};
  }

  const grid = new ElvisGrid(new GridStyle(
    style.col,
    style.col_gap,
    style.flow,
    style.row,
    style.row_gap,
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

export function MultiColumn(widgets: Widget[], style?: IMultiColumnStyle): Widget {
  if (style === undefined) {
    style = {};
  }

  const mc = new ElvisMultiColumn(new MultiColumnStyle(
    style.color,
    style.count,
    style.gap,
    style.style,
  ));
  for (const i in widgets) {
    if (widgets[i] !== undefined) {
      mc.push(widgets[i]);
    }
  }

  return mc.widget();
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

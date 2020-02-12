import {
  Align as ElvisAlign,
  AlignStyle,
  Container as ElvisContainer,
  ContainerStyle,
  IAlignStyle,
  IContainerStyle,
  ISizedBoxStyle,
  List as ElvisList,
  SizedBox as ElvisSizedBox,
  SizedBoxStyle,
  Widget,
} from "../pkg";

export function Align(widget: Widget, style: IAlignStyle): Widget {
  return ElvisAlign(widget, new AlignStyle(style.align));
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

export function List(widgets: Widget[]): Widget {
  const list = new ElvisList();
  for (const i in widgets) {
    if (widgets[i] !== undefined) {
      list.push(widgets[i]);
    }
  }

  return list.widget();
}

export function SizedBox(widget: Widget, style: ISizedBoxStyle): Widget {
  return ElvisSizedBox(widget, new SizedBoxStyle(style.height, style.width));
}

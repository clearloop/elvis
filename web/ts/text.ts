import {
  Colors,
  Text as ElvisText,
  TextStyle,
  Widget,
} from "../pkg";

// TextStyle Interface
export interface ITextStyle {
  bold?: boolean;
  color?: Colors;
  italic?: boolean;
  size?: number;
  weight?: number;
  height?: number;
  stretch?: number;
}

// Text Wrapper
export function Text(text: string, style: ITextStyle): Widget {
  return ElvisText(text, new TextStyle(
    style.bold,
    style.color,
    style.italic,
    style.size,
    style.weight,
    style.height,
    style.stretch,
  ));
}

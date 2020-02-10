import {
  ITextStyle,
  Text as ElvisText,
  TextStyle,
  Widget,
} from "../pkg";

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

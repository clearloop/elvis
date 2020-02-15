import {
  IImage,
  Image as ElvisImage,
  ITextStyle,
  Text as ElvisText,
  TextStyle,
  Widget,
} from "elvis-web";

export function Image(cfg: IImage): Widget {
  return ElvisImage(cfg.src, cfg.child);
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

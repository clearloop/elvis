import E from "./ElvisElement";

interface ITextStyle {
  bold: boolean;
  italic: boolean;
  size: number;
}

// Inner prototype class
class TextElement extends E {
  constructor(tag: string, text: string, style: ITextStyle) {
    super({ tag });
    this.text(text);

    const es = this.el.style;
    if (style) {
      if (style.bold) es.fontWeight = "700";
      if (style.italic) es.fontStyle = "italic";
      if (style.size) es.fontSize = `${style.size}rem`;
    }
  }
}

// exports
export { TextElement, ITextStyle };

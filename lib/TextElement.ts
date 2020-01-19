import { ElvisElement as E } from "./ElvisElement";

interface TextStyle {
  bold: boolean,
  italic: boolean,
  size: number,
}

// Inner prototype class
class TextElement extends E {
  constructor(tag: string, text: string, style: TextStyle) {
    super({tag: tag});
    this.text(text);
    
    let es = this.el.style;
    console.log(es);
    if (style) {
      if (style.bold) es.fontWeight = "700";
      if (style.italic) es.fontStyle = "italic";
      if (style.size) es.fontSize = `${style.size}rem`;
    }
  }
}

// exports
export { TextElement, TextStyle };

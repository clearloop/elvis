import { ElvisElement, Page, TextElement, TextStyle } from "@";

let t = new TextElement("p", "WTF", new TextStyle(true, "red", true, 9));
let p = new Page(t);
p.render();

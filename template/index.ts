import { Page, TextElement, TextStyle } from "../pkg";

let t = new TextElement("p", "WTF", new TextStyle(true, "red", true, 2.0));
let p = new Page(t.el());
p.render();

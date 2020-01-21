import { ElvisElement, Page, EvlisText } from "@";

let h1 = new ElvisText("h1", "hello, world");
let p = new Page(h1);
p.render();

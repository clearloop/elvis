import { ElvisElement, Page } from "@";

let h1 = new ElvisElement("h1").text("hello, world!");
let p = new Page(h1);
p.render();

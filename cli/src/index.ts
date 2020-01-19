import { Text, Page, Headline } from "@/index";

// page hello
let hello = new Page(
  new Text("headline", {
    bold: true,
    italic: true,
    size: 3
  })
);

hello.render();

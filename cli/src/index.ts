import { Headline, Page, Text } from "@/index";

// page hello
const hello = new Page(
  new Text("headline", {
    bold: true,
    italic: true,
    size: 2,
  }),
);

hello.render();

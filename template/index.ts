import { Page, TextStyle, Element } from "../pkg/elvis";
const { Title } = Element;

let page = new Page(
  Title(
    "WTF",
    new TextStyle(
      true, "red", true, 0.0
    )
  )
);

page.render();

use crate::Error;
// use crate::CSS;
use std::collections::HashMap;

/// Virtual DOM Tree
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tree {
    pub attrs: HashMap<&'static str, &'static str>,
    pub tag: &'static str,
    pub children: Vec<Box<Tree>>,
}

impl Tree {
    /// Deserialize Tree from html string
    ///
    /// `attrs` field follows MDN doc [HTML attribute refference][1],
    /// all values are `String` in "".
    ///
    /// [1]: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#Boolean_Attributes
    pub fn de(h: &'static str) -> Result<Self, Error> {
        if h.is_empty() {
            return Ok(Tree::default());
        } else if h.find("</").is_none() {
            let mut attrs = HashMap::<&'static str, &'static str>::new();
            attrs.insert("text", h);
            return Ok(Tree {
                tag: "plain",
                attrs: attrs,
                children: vec![],
            });
        }

        // parse tag
        let tag = &h[(h.find("<")? + 1)..h.find(|c: char| c.is_whitespace() || c == '>')?];

        // parse attrs
        let fe = h.find(">")?;
        let fw = h.find(|c: char| c.is_whitespace());
        let mut attrs = HashMap::<&'static str, &'static str>::new();
        if fw.is_some() && fe > fw.unwrap() {
            let ac = &h[fw.unwrap()..fe]
                .split("\"")
                .collect::<Vec<&'static str>>();

            if ac.len() % 2 == 0 {
                return Err(Error::DeserializeHtmlError(
                    "invalid html attrs".to_string(),
                ));
            }

            ac.iter().enumerate().for_each(|(p, q)| {
                if q.contains("=") {
                    let s = q.split("=").collect::<Vec<&'static str>>();
                    attrs.insert(s[0].trim(), ac[p + 1].trim());
                }
            });
        }

        // parse f*cking children
        let mut children: Vec<Box<Tree>> = vec![];
        let cht = &h[(h.find(">")? + 1)..(h.len() - tag.len() - 3)];
        if cht.len() > 0 && cht.find("</").is_none() {
            children.push(Box::new(Tree::de(&cht)?));
        } else if cht.len() > 0 {
            let mut ptr = 0_usize;
            while ptr < cht.len() - 1 {
                let i_tag = &cht[(cht[ptr..].find("<")? + 1)..cht[ptr..].find(">")?];
                let nxt = cht[(ptr + 1)..].find(&format!("<{}>", &i_tag));
                let end = cht[ptr..].find(&format!("</{}>", &i_tag));
                if i_tag.contains("<") || i_tag.contains(">") {
                    return Err(Error::DeserializeHtmlError(format!(
                        "html tag parse failed, {}, {}, {}, {}",
                        format!("tag: {}", &i_tag),
                        format!("ptr: {}", &ptr),
                        format!("cht: {}", &cht),
                        format!("ptr: {}", &cht[ptr..]),
                    )));
                }

                match (nxt, end) {
                    (Some(mut n), Some(mut e)) => {
                        // fill the prefix space
                        n = n + ptr + 1;
                        e = e + ptr;

                        // count pairs
                        let (mut ll, mut lr) = (1, 0);
                        while ll != lr {
                            if let Some(inxt) = cht[(n + 1)..].find(&format!("<{}>", &i_tag)) {
                                ll += 1;
                                n = inxt + n + 1;
                            }

                            if let Some(iend) = cht[(e + 1)..].find(&format!("</{}>", &i_tag)) {
                                lr += 1;
                                e = iend + e + 1;
                            }
                        }

                        children.push(Box::new(Tree::de(&cht[ptr..(e + i_tag.len() + 3)])?));
                        ptr = e + i_tag.len() + 3;
                    }
                    (None, Some(e)) => {
                        children.push(Box::new(Tree::de(&cht[ptr..(e + i_tag.len() + 3)])?));
                        ptr = e + i_tag.len() + 3;
                    }
                    _ => {
                        return Err(Error::DeserializeHtmlError(format!(
                            "html tag not pair, {}, {}, {}, {}, {}, {}",
                            format!("tag: {}", &i_tag),
                            format!("nxt: {:?}", &nxt),
                            format!("end: {:?}", &end),
                            format!("ptr: {}", &ptr),
                            format!("cht: {}", &cht[ptr..]),
                            format!("children: {:?}", &children),
                        )));
                    }
                }
            }
        }

        Ok(Tree {
            tag,
            attrs,
            children,
        })
    }

    // /// Serlize Tree to html string
    // #[allow(unconditional_recursion)]
    // pub fn ser(&self) -> String {
    //     format!(
    //         "<{} style=\"{}\">{}<{}>",
    //         &self.tag,
    //         &self.css.as_ref(),
    //         "",
    //         // self.children
    //         //     .as_ref()
    //         //     .unwrap_or(&Box::new(Tree::default()))
    //         //     .ser(),
    //         &self.tag
    //     )
    // }
}

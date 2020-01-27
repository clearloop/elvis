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
    fn de_children(
        tag: &'static str,
        cht: &'static str,
        children: &mut Vec<Box<Tree>>,
    ) -> Result<(), Error> {
        let mut ptr = 0_usize;
        while ptr < cht.len() - 1 {
            let nxt = cht[ptr..].find(&format!("<{}>", &tag));
            let end = cht[ptr..].find(&format!("</{}>", &tag));
            match (nxt, end) {
                (Some(mut n), Some(mut e)) => {
                    // fill the prefix space
                    n = n + ptr;
                    e = e + ptr;

                    // count pairs
                    let (mut ll, mut lr) = (1, 0);
                    while ll != lr {
                        if let Some(inxt) = cht[(n + 1)..].find(&format!("<{}>", &tag)) {
                            ll += 1;
                            n = inxt + n + 1;
                        }

                        if let Some(iend) = cht[(e + 1)..].find(&format!("</{}>", &tag)) {
                            lr += 1;
                            e = iend + e + 1;
                        }
                    }

                    children.push(Box::new(Tree::de(&cht[ptr..e])?));
                    ptr = e + tag.len() + 3;
                }
                (None, Some(e)) => {
                    if e != 0 {
                        children.push(Box::new(Tree::de(&cht[ptr..e])?));
                    }
                    ptr = e + tag.len() + 3;
                }
                _ => {
                    return Err(Error::DeserializeHtmlError(format!(
                        "html tag not pair, {}, {}, {}, {}, {}, {}",
                        format!("tag: {}", &tag),
                        format!("nxt: {:?}", &nxt),
                        format!("end: {:?}", &end),
                        format!("ptr: {}", &ptr),
                        format!("cht: {}", &cht[ptr..]),
                        format!("children: {:?}", &children),
                    )));
                }
            }
        }
        Ok(())
    }

    /// Deserialize Tree from html string
    ///
    /// `attrs` field follows MDN doc [HTML attribute refference][1],
    /// all values are `String` in "".
    ///
    /// [1]: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#Boolean_Attributes
    pub fn de(h: &'static str) -> Result<Self, Error> {
        let mut pos = 0_usize;
        if h.is_empty() {
            return Ok(Tree::default());
        } else if h.find("</").is_none() {
            return parser::plain(h);
        }

        let (tag, _) = parser::tag(&h[pos..], &mut pos)?;

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
        Self::de_children(tag, &h[(fe + 1)..], &mut children)?;

        Ok(Tree {
            tag,
            attrs,
            children,
        })
    }
}

mod parser {
    use crate::{Error, Tree};
    use std::collections::HashMap;

    /// process of parsing tag
    pub enum TagProcess {
        Attrs,
        Quote,
        None,
        Tag,
    }

    /// generate palin text
    pub fn plain(h: &'static str) -> Result<Tree, Error> {
        let mut attrs = HashMap::<&'static str, &'static str>::new();
        attrs.insert("text", h);

        Ok(Tree {
            tag: "plain",
            attrs: attrs,
            children: vec![],
        })
    }

    /// parse html tag
    pub fn tag(
        h: &'static str,
        pos: &mut usize,
    ) -> Result<(&'static str, HashMap<&'static str, &'static str>), Error> {
        let b = h.find("<")? + 1;
        let e = &h[b..].find(|c: char| c.is_whitespace() || c == '>')? + b;
        let tag = &h[b..e];
        if tag.contains("<") || tag.contains(">") || tag.contains("/") {
            return Err(Error::DeserializeHtmlError(format!(
                "html tag parse failed: {}, html: {}",
                &tag, &h
            )));
        }

        let (mut t, mut k, mut v) = ((0, 0), (0, 0), (0, 0));
        let mut attrs = HashMap::<&'static str, &'static str>::new();
        let mut process = TagProcess::None;
        for (p, q) in h.chars().enumerate() {
            match q {
                '<' => process = TagProcess::Tag,
                '>' => {
                    *pos = *pos + p + 1;
                    return Ok((tag, HashMap::new()));
                }
                '"' => match process {
                    TagProcess::Quote => process = TagProcess::Attrs,
                    _ => process = TagProcess::Quote,
                },
                x if !x.is_whitespace() => match process {
                    TagProcess::Tag => {
                        t.1 += 1;
                    }
                    TagProcess::Quote => {}
                    TagProcess::Attrs => {
                        if k.1 - k.0 != 0 {
                            k.1 += 1;
                        } else {
                            v.1 += 1;
                        }
                    }
                    _ => {}
                },
                x if x.is_whitespace() => match process {
                    TagProcess::Tag => {
                        if t.1 - t.0 != 0 {
                            t.0 = p;
                            process = TagProcess::Attrs;
                        }
                    }
                    TagProcess::Quote => {}
                    TagProcess::Attrs => {
                        if (k.1 - k.0 != 0) && (v.1 - v.0 != 0) {
                            attrs.insert(&h[k.0..k.1], &h[v.0..v.1]);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Err(Error::DeserializeHtmlError(format!(
            "html tag parse failed: {}, html: {}",
            &tag, &h
        )))
    }
}

//! parser in #[cfg(feature = "web")]
use crate::err::Error;
use crate::{Serde, Node};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// Extra html stream
#[derive(Debug)]
struct Extra<'e> {
    pub end: bool,
    pub pos: usize,
    pub tag: &'e str,
}

/// process of parsing children
#[derive(Eq, PartialEq)]
enum ChildrenProcess {
    BeginTag,
    CloseTag,
    None,
    Plain,
}

/// process of parsing tag
enum TagProcess {
    Attrs,
    Quote,
    None,
    Tag,
}

/// Deserialize Node from html string
///
/// `attrs` field follows MDN doc [HTML attribute refference][1],
/// all values are `String` in "".
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#Boolean_Attributes
fn rde<'t>(
    h: &'t str,
    pre: Option<Weak<RefCell<Node>>>,
) -> Result<(Rc<RefCell<Node>>, Option<Extra<'t>>), Error> {
    let mut pos = 0_usize;
    if h.is_empty() {
        return Ok((Rc::new(RefCell::new(Node::default())), None));
    } else if h.find("</").is_none() {
        return Ok((Rc::new(RefCell::new(self::plain(h, pre.clone()))), None));
    }

    // the return-will tree
    let tree = Rc::new(RefCell::new(Node::default()));
    let tw = Rc::downgrade(&tree);
    let (tag, attrs) = self::tag(&h[pos..], &mut pos)?;

    // parse f*cking children
    let mut children: Vec<Rc<RefCell<Node>>> = vec![];
    let mut cext = self::ch(&h[pos..], Some(tw.clone()), tag, &mut children)?;

    // parse parallel children
    pos += cext.pos;
    while !cext.end {
        cext = self::ch(&h[pos..], Some(tw.clone()), tag, &mut children)?;
        pos += cext.pos;
    }

    // communite with child parser
    let mut ext = None;
    if (pos + 1) != h.len() {
        ext = Some(Extra {
            end: false,
            pos: pos,
            tag: cext.tag,
        });
    }

    let mut bt = tree.borrow_mut();
    bt.pre = pre;
    bt.tag = tag.to_string();
    bt.attrs = attrs;
    bt.children = children;
    drop(bt);

    Ok((tree, ext))
}

/// push child from html stream
fn ch<'t>(
    cht: &'t str,
    pre: Option<Weak<RefCell<Node>>>,
    tag: &'t str,
    children: &mut Vec<Rc<RefCell<Node>>>,
) -> Result<Extra<'t>, Error> {
    let mut itag = tag;
    let mut process = ChildrenProcess::None;
    let (mut t, mut c) = ((0, 0), (0, 0));
    for (p, q) in cht.chars().enumerate() {
        match q {
            '<' => {
                if process == ChildrenProcess::Plain {
                    c.1 = p;
                }

                process = ChildrenProcess::BeginTag;
                t.0 = p;
                t.1 = p;
            }
            '/' => {
                if &cht[t.0..(t.0 + 1)] == "<" {
                    process = ChildrenProcess::CloseTag;
                } else if process != ChildrenProcess::Plain {
                    return Err(Error::DeserializeHtmlError(format!(
                        "children parse failed {}, cht: {}, process: {}",
                        &tag,
                        &cht,
                        &cht[t.0..(t.0 + 1)],
                    )));
                }
            }
            '>' => {
                t.1 = p;
                match process {
                    ChildrenProcess::BeginTag => {
                        let (tree, ext) = self::rde(&cht[t.0..], pre.clone())?;
                        children.push(tree);

                        // communite with father node
                        if let Some(cext) = ext {
                            return Ok(Extra {
                                end: false,
                                tag: cext.tag,
                                pos: cext.pos + t.0,
                            });
                        }
                    }
                    ChildrenProcess::CloseTag => {
                        // verify tag, trim:  "/ tag"
                        itag = &cht[(t.0 + 1)..t.1].trim()[1..].trim()[..];
                        if itag != tag {
                            return Err(Error::DeserializeHtmlError(format!(
                                "children parse failed {}, cht: {}, close_tag: {}",
                                &tag, &cht, &itag
                            )));
                        } else if !cht[c.0..c.1].is_empty() {
                            children.push(Rc::new(RefCell::new(self::plain(
                                &cht[c.0..c.1],
                                pre.clone(),
                            ))));
                        }

                        return Ok(Extra {
                            end: true,
                            pos: p,
                            tag: itag,
                        });
                    }
                    _ => {
                        // None and Plain
                    }
                }
            }
            x if !x.is_whitespace() => {
                match process {
                    ChildrenProcess::None => {
                        process = ChildrenProcess::Plain;
                        c.0 = p;
                        c.1 = p;
                    }
                    ChildrenProcess::Plain => {
                        c.1 = p;
                    }
                    _ => {
                        // tag conditions
                    }
                }
            }
            _ => {
                // invalid chars
            }
        }
    }
    Ok(Extra {
        end: true,
        pos: cht.len(),
        tag: itag,
    })
}

/// generate palin text
fn plain<'t>(h: &'t str, pre: Option<Weak<RefCell<Node>>>) -> Node {
    let mut attrs = HashMap::<String, String>::new();
    attrs.insert("text".into(), h.into());

    Node {
        pre: pre.clone(),
        tag: "plain".into(),
        attrs: attrs,
        children: vec![],
    }
}

/// parse html tag
fn tag<'t>(h: &'t str, pos: &mut usize) -> Result<(&'t str, HashMap<String, String>), Error> {
    let (mut t, mut k, mut v) = ((0, 0), (0, 0), (0, 0));
    let mut attrs = HashMap::<String, String>::new();
    let mut process = TagProcess::None;
    for (p, q) in h.chars().enumerate() {
        match q {
            '<' => {
                process = TagProcess::Tag;
                t.0 = p + 1;
                t.1 = p + 1;
            }
            '>' => {
                match process {
                    TagProcess::Tag => t.1 = p,
                    TagProcess::Attrs => {
                        if !&h[k.0..k.1].trim().is_empty() {
                            attrs.insert(h[k.0..k.1].trim().to_string(), h[v.0..v.1].trim().into());
                        }
                    }
                    _ => {}
                }

                *pos = *pos + p + 1;
                return Ok((&h[t.0..t.1].trim(), attrs));
            }
            '"' => match process {
                TagProcess::Quote => {
                    process = TagProcess::Attrs;
                    v.1 = p;
                }
                _ => {
                    v.0 = p + 1;
                    v.1 = p + 1;
                    process = TagProcess::Quote;
                }
            },
            '=' => match process {
                TagProcess::Attrs => k.1 = p,
                _ => {
                    return Err(Error::DeserializeHtmlError(format!(
                        "html tag parse failed: {}, html: {}",
                        &h[t.0..t.1],
                        &h
                    )))
                }
            },
            x if x.is_whitespace() => match process {
                TagProcess::Tag => {
                    if h[t.0..t.1].trim().is_empty() {
                        t.1 = p;
                    } else {
                        process = TagProcess::Attrs;
                        k.0 = p + 1;
                        k.1 = p + 1;
                    }
                }
                TagProcess::Quote => {
                    v.1 = p;
                }
                TagProcess::Attrs => {
                    if (k.1 - k.0 != 0) && (v.1 - v.0 != 0) {
                        attrs.insert(h[k.0..k.1].trim().to_string(), h[v.0..v.1].trim().into());
                        k.0 = p;
                        k.1 = p;
                    }
                }
                _ => {}
            },
            x if !x.is_whitespace() => match process {
                TagProcess::Tag => {
                    t.1 = p + 1;
                }
                TagProcess::Quote => {
                    v.1 = p;
                }
                TagProcess::Attrs => {
                    if v.0 == 0 {
                        k.1 = p;
                    } else {
                        v.1 = p;
                    }
                }
                _ => {}
            },
            _ => {
                return Err(Error::DeserializeHtmlError(format!(
                    "html tag parse failed: {}, html: {}, char: {}",
                    &h[t.0..t.1],
                    &h,
                    &q
                )))
            }
        }
    }

    Err(Error::DeserializeHtmlError(format!(
        "html tag parse failed: {}, html: {}",
        &h[t.0..t.1],
        &h
    )))
}

impl<'t> Serde<Node, String> for Node {
    fn de(h: String) -> Result<Node, Error> {
        Ok(self::rde(Box::leak(Box::new(h)), None)?
            .0
            .borrow()
            .to_owned())
    }

    fn ser(&self) -> String {
        let mut html = "".to_string();
        let mut attrs = " ".to_string();
        let mut children = "".to_string();

        // plain text
        if self.tag == "plain" {
            html.push_str(&self.attrs.get("text").unwrap_or(&"".into()));
        } else {
            for (k, v) in self.attrs.iter() {
                attrs.push_str(&format!("{}=\"{}\" ", k, v));
            }

            for i in &self.children {
                children.push_str(&i.borrow().to_owned().ser());
            }

            if attrs.trim().is_empty() {
                attrs.drain(..);
            }

            html.push_str(&format!(
                "<{}{}>{}</{}>",
                &self.tag,
                attrs.trim_end(),
                children,
                &self.tag,
            ));
        }

        html
    }
}

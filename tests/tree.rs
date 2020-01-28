use elvis::Tree;
use std::collections::HashMap;

#[test]
fn de_tree_empty_string() {
    let plain = "";
    assert_eq!(Tree::de(plain).unwrap(), Tree::default());
}

#[test]
fn de_tree_pure_tag() {
    assert_eq!(
        Tree::de("<div></div>").unwrap(),
        Tree {
            tag: "div",
            attrs: HashMap::new(),
            children: vec![],
        }
    );
}

#[test]
fn de_tree_has_plain_content() {
    let mut attrs = HashMap::<&'static str, &'static str>::new();
    attrs.insert("text", "elvis");

    let predicate = Tree {
        tag: "div",
        attrs: HashMap::new(),
        children: vec![Box::new(Tree {
            tag: "plain",
            attrs: attrs,
            children: vec![],
        })],
    };

    assert_eq!(Tree::de("<div>elvis</div>").unwrap(), predicate);
    assert_eq!(Tree::de("<div>elvis</ div>").unwrap(), predicate);
    assert_eq!(Tree::de("<div>elvis</ div >").unwrap(), predicate);
    assert_eq!(Tree::de("< div>elvis</div>").unwrap(), predicate);
    assert_eq!(Tree::de("< div >elvis</div>").unwrap(), predicate);
}

#[test]
fn de_tree_tag_and_style() {
    let mut attrs = HashMap::<&'static str, &'static str>::new();
    attrs.insert("style", "height: 20;");
    assert_eq!(
        Tree::de("<div style=\"height: 20;\"></div>").unwrap(),
        Tree {
            tag: "div",
            attrs: attrs,
            children: vec![],
        }
    );
}

#[test]
fn de_tree_tag_and_multi_attr() {
    let mut attrs = HashMap::<&'static str, &'static str>::new();
    attrs.insert("style", "height: 20;");
    attrs.insert("name", "bowie");
    attrs.insert("bool", "false");
    assert_eq!(
        Tree::de("<div style=\"height: 20;\" name=\"bowie\" bool=\"false\"></div>").unwrap(),
        Tree {
            tag: "div",
            attrs: attrs,
            children: vec![],
        }
    );
}

#[test]
fn de_tree_has_single_tag_child() {
    assert_eq!(
        Tree::de("<div><div></div></div>").unwrap(),
        Tree {
            tag: "div",
            attrs: HashMap::new(),
            children: vec![Box::new(Tree {
                tag: "div",
                attrs: HashMap::new(),
                children: vec![],
            })],
        }
    );
}

#[test]
fn de_tree_has_deep_single_tag_child() {
    assert_eq!(
        Tree::de("<div><div><div></div></div></div>").unwrap(),
        Tree {
            tag: "div",
            attrs: HashMap::new(),
            children: vec![Box::new(Tree {
                tag: "div",
                attrs: HashMap::new(),
                children: vec![Box::new(Tree {
                    tag: "div",
                    attrs: HashMap::new(),
                    children: vec![],
                })],
            })],
        }
    );
}

#[test]
fn de_tree_has_deep_multi_tag_child() {
    assert_eq!(
        Tree::de("<div><a><b><p></p></b></a></div>").unwrap(),
        Tree {
            tag: "div",
            attrs: HashMap::new(),
            children: vec![Box::new(Tree {
                tag: "a",
                attrs: HashMap::new(),
                children: vec![Box::new(Tree {
                    tag: "b",
                    attrs: HashMap::new(),
                    children: vec![Box::new(Tree {
                        tag: "p",
                        attrs: HashMap::new(),
                        children: vec![],
                    })],
                })],
            })],
        }
    );
}

#[test]
fn de_tree_has_parallel_tag_children() {
    assert_eq!(
        Tree::de("<div><div></div><div></div></div>").unwrap(),
        Tree {
            tag: "div",
            attrs: HashMap::new(),
            children: vec![
                Box::new(Tree {
                    tag: "div",
                    attrs: HashMap::new(),
                    children: vec![]
                }),
                Box::new(Tree {
                    tag: "div",
                    attrs: HashMap::new(),
                    children: vec![]
                })
            ],
        }
    );
}

#[test]
fn de_tree_has_parallel_multi_tag_children() {
    assert_eq!(
        Tree::de("<div><a></a><b></b><p></p></div>").unwrap(),
        Tree {
            tag: "div",
            attrs: HashMap::new(),
            children: vec![
                Box::new(Tree {
                    tag: "a",
                    attrs: HashMap::new(),
                    children: vec![]
                }),
                Box::new(Tree {
                    tag: "b",
                    attrs: HashMap::new(),
                    children: vec![]
                }),
                Box::new(Tree {
                    tag: "p",
                    attrs: HashMap::new(),
                    children: vec![]
                })
            ],
        }
    );
}

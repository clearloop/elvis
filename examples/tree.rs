use elvis::Tree;

fn main() {
    let r = Tree::de(
        "<div id=\"1\"><div id=\"2\"></div><div id=\"3\"></div><div id=\"4\"></div></div>",
    );
    // let r = Tree::de("<div>elvis</div>").unwrap();
    // let a = Tree::de("<div><div></div><div></div></div>").unwrap();
    println!("{:#?}", r);
}

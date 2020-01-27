use elvis::Tree;

fn main() {
    let r = Tree::de("<div style=\"height: 20;\"></div>").unwrap();
    // let r = Tree::de("<div>elvis</div>").unwrap();
    // let a = Tree::de("<div><div></div><div></div></div>").unwrap();
    println!("{:#?}", r);
}

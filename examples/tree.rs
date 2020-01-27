use elvis::Tree;

fn main() {
    let a = Tree::de("<div><div></div><div></div></div>").unwrap();
    println!("{:#?}", a);
}

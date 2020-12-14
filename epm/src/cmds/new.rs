use etc::{Etc, Tree, Write};
use std::{env, path::PathBuf};

/// Run new command
pub fn run(path: PathBuf, node: &'static str) {
    let mut proj: PathBuf;
    if path.is_relative() {
        proj = env::current_dir().unwrap();
        proj.push(path.file_name().unwrap());
    } else {
        proj = path;
    }

    let mut tree: Tree = toml::from_str(node).unwrap();
    if tree.content.is_some() {
        tree.redir(proj).unwrap();
    } else if let Some(ts) = &mut tree.children {
        ts.iter_mut().for_each(|f| {
            f.redir(proj.clone()).unwrap();
        });
    }

    tree.map(|f| {
        if f.content.is_some() {
            Etc::from(f.path.clone())
                .write(f.content.as_ref().unwrap())
                .unwrap()
        }
    });
}

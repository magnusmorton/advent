use std::{ rc::Rc, cell::RefCell, collections::HashMap};

use regex::Regex;

struct FileNode {
    name: String,
    size: usize,
    children: Option<HashMap<String,Rc<RefCell<FileNode>>>>,
    parent: Option<Rc<RefCell<FileNode>>>
}

fn main() {
    let str = std::fs::read_to_string("input.txt").unwrap();
    let root = Rc::new(RefCell::new(FileNode{name:"/".to_string(),size: 0, children: Some(HashMap::new()), parent: None}));
    let mut pwd = root.clone();
    let mut tot: usize = 0;
    let cd_reg = Regex::new(r"\$ cd (.+)").unwrap();
    //let ls_reg = Regex::new(r"\$ ls").unwrap();
    let dir_reg =Regex::new(r"dir (.+)").unwrap();
    let file_reg = Regex::new(r"(\d+) (.+)").unwrap();
    for line in str.lines() {
        if let Some(cap) = cd_reg.captures(line) {
            
            let dir = cap.get(1).unwrap().as_str();
            println!("cd {}", dir);
            if dir == "/" {
                println!("ROOT");
                pwd = root.clone()
            } else if dir == ".." {
                println!("UP");
                let clone = pwd.clone();
                let mut brw = clone.borrow_mut();
                let children = brw.children.as_ref().unwrap();
                let mut size = 0;
                for child in children.values() {
                    size += child.borrow().size;
                }
                brw.size = size;
                if size <= 100000 {
                    tot += size
                }
                let tmp = brw.parent.as_ref().unwrap().clone();
                pwd = tmp;
            } else {
                let clone = pwd.clone();
                let brw = clone.borrow();
                let children = brw.children.as_ref().unwrap();

                let child = children.get(&dir.to_string());
                pwd = child.unwrap().clone();
            }
            
        } else if let Some(cap) = dir_reg.captures(line) {
            let name = cap.get(1).unwrap().as_str().to_string();
            let namec = name.clone();
            let clone = pwd.clone();
            let mut brw = clone.borrow_mut();
            let children = brw.children.as_mut().unwrap();
            if !children.contains_key(&name) {
                children.insert(name, Rc::new(RefCell::new(FileNode { name: namec, size: 0, children: Some(HashMap::new()), parent: Some(pwd.clone()) })));
            }
        } else if let Some(cap) = file_reg.captures(line) {
            let name = cap.get(2).unwrap().as_str().to_string();
            let namec = name.clone();
            let clone = pwd.clone();
            let mut brw = clone.borrow_mut();
            println!("{:?} {}", brw.name, cap.get(2).unwrap().as_str());
            let children = brw.children.as_mut().unwrap();
            if !children.contains_key(&name) {
                children.insert(name, Rc::new(RefCell::new(FileNode { name:namec, size: str::parse(cap.get(1).unwrap().as_str()).unwrap(), children: None, parent: Some(pwd.clone()) })));
            }
        }
    }

    println!("{}", tot)
}

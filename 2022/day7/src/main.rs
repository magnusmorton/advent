use std::{ rc::Rc, cell::RefCell, collections::{HashMap, VecDeque}};

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
    let dir_reg =Regex::new(r"dir (.+)").unwrap();
    let file_reg = Regex::new(r"(\d+) (.+)").unwrap();
    for line in str.lines() {
        let clone = pwd.clone();
        let mut brw = clone.borrow_mut();   
        let children = brw.children.as_mut().unwrap();
        if let Some(cap) = cd_reg.captures(line) {
            let dir = cap.get(1).unwrap().as_str();
            if dir == "/" {
                pwd = root.clone()
            } else if dir == ".." {
                pwd = brw.parent.as_ref().unwrap().clone();
            } else {
                let child = children.get(&dir.to_string());
                pwd = child.unwrap().clone();
            }
            
        } else if let Some(cap) = dir_reg.captures(line) {
            let name = cap.get(1).unwrap().as_str().to_string();
            let namec = name.clone();
            if !children.contains_key(&name) {
                children.insert(name, Rc::new(RefCell::new(FileNode { name: namec, size: 0, children: Some(HashMap::new()), parent: Some(pwd.clone()) })));
            }
        } else if let Some(cap) = file_reg.captures(line) {
            let name = cap.get(2).unwrap().as_str().to_string();
            let namec = name.clone();
            if !children.contains_key(&name) {
                children.insert(name, Rc::new(RefCell::new(FileNode { name:namec, size: str::parse(cap.get(1).unwrap().as_str()).unwrap(), children: None, parent: Some(pwd.clone()) })));
            }
        }
    }

    let mut stack = Vec::new();
    let mut other_stack = VecDeque::new();
    stack.push(root.clone());
    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        other_stack.push_front(current.clone());

        for child in current.borrow().children.as_ref().unwrap().values() {
            if child.borrow().children.is_some() {
                stack.push(child.clone())
            }
        }
    }

    for item in &other_stack {
        let mut size = 0;
        for child in item.borrow().children.as_ref().unwrap().values() {
            size += child.borrow().size;
        }
        if size <= 100000 {
            tot += size;
        }
        item.borrow_mut().size = size;
    }
    println!("{}", tot);

    

    let unsused = 70000000 - root.borrow().size;
    let target = 30000000 - unsused;
    
    let mut smallest: usize = usize::MAX;
    for item in &other_stack {
        let sz = item.borrow().size;
        if sz >= target && sz < smallest {
            smallest = sz
        }
    }
    println!("smallest: {}", smallest)
}

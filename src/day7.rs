use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap};
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

enum FSNode {
    DIR {
        parent: Option<Rc<RefCell<FSNode>>>,
        children: HashMap<String, Rc<RefCell<FSNode>>>,
    },
    FILE {
        parent: Rc<RefCell<FSNode>>,
        size: u32,
    },
    NIL,
}

impl Default for FSNode {
    fn default() -> Self {
        FSNode::NIL
    }
}

impl FSNode {
    fn new_nil() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(FSNode::NIL))
    }

    fn new_dir(parent: Option<Rc<RefCell<Self>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(FSNode::DIR {
            parent,
            children: HashMap::new(),
        }))
    }

    fn new_file(parent: Rc<RefCell<FSNode>>, size: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(FSNode::FILE { parent, size }))
    }

    fn is_dir(&self) -> bool {
        match self {
            FSNode::DIR { .. } => true,
            _ => false,
        }
    }

    fn add_child(&mut self, name: String, fs_node: Rc<RefCell<Self>>) {
        match self {
            FSNode::DIR { children, .. } => {
                children.insert(name, fs_node);
            }
            _ => panic!("Cannot add child to a file!"),
        };
    }

    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Self>>> {
        match self {
            FSNode::DIR { children, .. } => {
                if let Some(child) = children.get(name) {
                    Some(Rc::clone(child))
                } else {
                    None
                }
            }
            _ => panic!("Cannot add child to a file!"),
        }
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Self>>> {
        match self {
            FSNode::DIR {
                parent: Some(p), ..
            }
            | FSNode::FILE { parent: p, .. } => Some(Rc::clone(p)),
            _ => None,
        }
    }

    fn size(&self) -> u32 {
        match self {
            FSNode::FILE { size, .. } => *size,
            FSNode::DIR { children, .. } => {
                let mut sum = 0u32;
                for child in children.values() {
                    sum += RefCell::borrow(child).size();
                }
                sum
            }
            FSNode::NIL => {
                panic!("I shouldn't be here!");
            }
        }
    }

    fn visit_dirs<F: FnMut(&FSNode)>(&self, visitor: &mut F) {
        match self {
            FSNode::DIR { children, .. } => {
                for child in children.values() {
                    Self::visit_dirs(RefCell::borrow(child).deref(), visitor);
                }
                visitor(self);
            }
            FSNode::FILE { size, .. } => {}
            FSNode::NIL => {
                panic!("I shouldn't be here!");
            }
        }
    }
}

enum LineType {
    CMD_CD_ROOT,
    CMD_CD_UP,
    CMD_CD_DOWN { name: String },
    CMD_LS,
    RESULT_DIR { name: String },
    RESULT_FILE { name: String, size: u32 },
}

fn parse_line(line: &str) -> LineType {
    if line.strip_prefix("$ cd /").is_some() {
        return LineType::CMD_CD_ROOT;
    } else if line.strip_prefix("$ cd ..").is_some() {
        return LineType::CMD_CD_UP;
    } else if line.strip_prefix("$ ls").is_some() {
        return LineType::CMD_LS;
    } else if let Some(dir_name) = line.strip_prefix("$ cd ") {
        return LineType::CMD_CD_DOWN {
            name: dir_name.trim().to_owned(),
        };
    } else if let Some(dir_name) = line.strip_prefix("dir ") {
        return LineType::RESULT_DIR {
            name: dir_name.trim().to_owned(),
        };
    } else {
        let size_name = line.split(" ").collect::<Vec<&str>>();
        assert_eq!(2, size_name.len());
        return LineType::RESULT_FILE {
            name: size_name[1].to_owned(),
            size: u32::from_str(size_name[0]).unwrap(),
        };
    }
    todo!()
}

fn build_tree(input: &str) -> Rc<RefCell<FSNode>> {
    let root = FSNode::new_dir(None);

    let mut current_dir = Rc::clone(&root);
    for line in input.lines() {
        {
            let my_dir = RefCell::borrow(&current_dir);
            if !my_dir.is_dir() {
                panic!("Current dir is not a dir! {}", line);
            }
        }

        let line_type = parse_line(line);
        match line_type {
            LineType::CMD_CD_ROOT => {
                current_dir = Rc::clone(&root);
            }
            LineType::CMD_CD_UP => {
                let mut parent = FSNode::new_nil();
                {
                    let my_dir = RefCell::borrow(&current_dir);
                    parent = my_dir.get_parent().unwrap();
                }
                current_dir = Rc::clone(&parent);
            }
            LineType::CMD_LS => {
                // NO OP
            }
            LineType::CMD_CD_DOWN { name } => {
                let mut next_current = FSNode::new_nil();
                {
                    let mut my_dir = RefCell::borrow_mut(&current_dir);
                    if let Some(node) = my_dir.get_child(&name) {
                        next_current = Rc::clone(&node);
                    } else {
                        let new_child = FSNode::new_dir(Some(current_dir.clone()));
                        my_dir.add_child(name.clone(), new_child.clone());
                        next_current = Rc::clone(&new_child);
                    }
                }
                current_dir = next_current;
            }
            LineType::RESULT_DIR { name } => {
                let mut my_dir = RefCell::borrow_mut(&current_dir);
                my_dir.add_child(name, FSNode::new_dir(Some(Rc::clone(&current_dir))));
            }
            LineType::RESULT_FILE { name, size } => {
                let mut my_dir = RefCell::borrow_mut(&current_dir);
                my_dir.add_child(name, FSNode::new_file(Rc::clone(&current_dir), size));
            }
        }
    }
    root
}

pub fn solve(input: &str) -> u32 {
    let root = build_tree(input);

    let mut sum = 0;
    let mut print_size = |dir: &FSNode| {
        let size = dir.size();
        println!("Dir Size: {}", size);
        if size < 100000 {
            sum += size;
        }
    };
    RefCell::borrow(&root).visit_dirs(&mut print_size);

    sum
}

pub fn solve2(input: &str) -> u32 {
    let root = build_tree(input);

    let used_space = RefCell::borrow(&root).size();
    let unused_space = 70_000_000 - used_space;
    let required_space = 30_000_000;
    let need_to_free = required_space - unused_space;

    let mut heap = BinaryHeap::new();
    use std::cmp::Reverse;

    let mut print_size = |dir: &FSNode| {
        let size = dir.size();
        if size > need_to_free {
            println!("Big Enough: {}", size);
            heap.push(Reverse(size));
        }
    };
    RefCell::borrow(&root).visit_dirs(&mut print_size);

    heap.pop().unwrap().0
}

#[test]
fn test_solve() {
    let input = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#
    .strip_prefix("\n")
    .unwrap()
    .strip_suffix("\n")
    .unwrap();
    assert_eq!(95437, solve(input));
    assert_eq!(24933642, solve2(input))
}

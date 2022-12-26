use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
enum Node {
    Dir(Dir),
    File(File),
}

trait Child {
    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>>;
}

impl Child for Node {
    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>> {
        match self {
            Self::File(f) => f.get_parent(),
            Self::Dir(d) => d.get_parent(),
        }
    }
}

impl Child for Dir {
    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>> {
        self.parent.upgrade()
    }
}
impl Child for File {
    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>> {
        self.parent.upgrade()
    }
}

type Bytes = usize;
impl Node {
    pub fn get_byte_size(&self) -> Bytes {
        match self {
            Self::File(f) => f.bytes,
            Self::Dir(d) => d.children.iter().map(|c| c.borrow().get_byte_size()).sum(),
        }
    }

    pub fn get_tree(self) -> Dir {
        let mut top_dir: Dir = match self {
            Self::File(f) => f.get_parent().unwrap().take(),
            Self::Dir(d) => d,
        };

        while let Some(p) = top_dir.get_parent() {
            top_dir = p.take();
        }
        top_dir
    }
}
impl Into<Node> for Dir {
    fn into(self) -> Node {
        Node::Dir(self)
    }
}
impl Into<Node> for File {
    fn into(self) -> Node {
        Node::File(self)
    }
}

impl TryInto<File> for Node {
    type Error = String;
    fn try_into(self) -> Result<File, Self::Error> {
        if let Self::File(f) = self {
            Ok(f)
        } else {
            Err("Not a File".into())
        }
    }
}
impl TryInto<Dir> for Node {
    type Error = String;
    fn try_into(self) -> Result<Dir, Self::Error> {
        if let Self::Dir(d) = self {
            Ok(d)
        } else {
            Err("Not a Dir".into())
        }
    }
}
#[derive(Debug, Default, Clone)]
struct Dir {
    name: String,
    parent: Weak<RefCell<Dir>>,
    children: Vec<Rc<RefCell<Node>>>,
}
impl Dir {
    fn add_parent(self_: Rc<RefCell<Self>>, parent: Rc<RefCell<Dir>>) {
        (*self_).replace_with(|old| Dir {
            parent: Rc::downgrade(&parent.clone()),
            ..old.clone()
        });

        let self_ = self_.take();
        /*
                parent
                    .borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(Node::Dir(self_))));

        */
        Dir::add_child(parent, Rc::new(RefCell::new(Node::Dir(self_))));
    }
    fn add_child(self_: Rc<RefCell<Self>>, child: Rc<RefCell<Node>>) {
        (*child).replace_with(|old| match old {
            Node::Dir(d) => Node::Dir(Dir {
                parent: Rc::downgrade(&self_),
                ..d.clone()
            }),
            Node::File(f) => Node::File(File {
                parent: Rc::downgrade(&self_),
                ..f.clone()
            }),
        });
        let mut x = (*self_).borrow_mut();
        x.children.push(child);
    }
    pub fn get_byte_size(&self) -> Bytes {
        self.children
            .iter()
            .map(|c| c.borrow().get_byte_size())
            .sum()
    }
}

#[derive(Debug, Default, Clone)]
struct File {
    name: String,
    bytes: Bytes,
    parent: Weak<RefCell<Dir>>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_nodes() {
        let dir1 = Dir {
            name: "dir1".into(),
            ..Dir::default()
        };
        let dir2 = Dir {
            name: "dir2".into(),
            ..Dir::default()
        };
        let file1 = File {
            name: "file1".into(),
            ..File::default()
        };

        let dir2_node: Node = dir2.into();
        let file1_node: Node = file1.into();

        let ref_dir1 = Rc::new(RefCell::new(dir1));

        let ref_dir2_node = Rc::new(RefCell::new(dir2_node));
        let ref_file1_node = Rc::new(RefCell::new(file1_node));

        Dir::add_child(Rc::clone(&ref_dir1), ref_dir2_node);
        Dir::add_child(Rc::clone(&ref_dir1), ref_file1_node);
    }

    #[test]
    fn test_add_parent() {
        let dir1 = Dir {
            name: "dir1".into(),
            ..Dir::default()
        };
        // strong ref_dir1
        let ref_dir1 = Rc::new(RefCell::new(dir1));

        {
            let dir = &ref_dir1.borrow().name;
            assert_eq!("dir1", dir);
        }

        let root = Dir {
            name: "/".into(),
            ..Default::default()
        };

        let ref_root = Rc::new(RefCell::new(root));

        dbg!(Dir::add_parent(Rc::clone(&ref_dir1), Rc::clone(&ref_root)));

        {
            let child = ref_root
                .borrow()
                .children
                .get(0)
                .unwrap()
                .clone()
                .borrow()
                .clone();
            let child: Dir = TryInto::try_into(child).unwrap();
            let parent = child.parent.upgrade().unwrap();
            let parent = &parent.borrow().name;
            assert_eq!("/", parent);
        }
    }

    #[test]
    fn test_get_byte_size() {
        let dir1 = Dir {
            name: "dir1".into(),
            ..Dir::default()
        };
        let dir2 = Dir {
            name: "dir2".into(),
            ..Dir::default()
        };
        let file1 = File {
            name: "file1".into(),
            bytes: 500,
            ..File::default()
        };
        let file2 = File {
            name: "file1".into(),
            bytes: 456,
            ..File::default()
        };

        let ref_dir1 = Rc::new(RefCell::new(dir1));
        let ref_dir2_node = Rc::new(RefCell::new(Node::Dir(dir2)));
        let ref_file1_node = Rc::new(RefCell::new(Node::File(file1)));
        let ref_file2_node = Rc::new(RefCell::new(Node::File(file2)));

        Dir::add_child(ref_dir1.clone(), ref_dir2_node);
        Dir::add_child(ref_dir1.clone(), ref_file1_node);
        Dir::add_child(ref_dir1.clone(), ref_file2_node);

        let bytes = ref_dir1.borrow().get_byte_size();
        assert_eq!(956, bytes);
    }

    #[test]
    fn test_get_tree() {
        let dir1 = Dir {
            name: "dir1".into(),
            ..Dir::default()
        };
        let dir2 = Dir {
            name: "dir2".into(),
            ..Dir::default()
        };
        let file1 = File {
            name: "file1".into(),
            ..File::default()
        };

        let dir2_node: Node = dir2.into();
        let file1_node: Node = file1.into();

        let ref_dir1 = Rc::new(RefCell::new(dir1));

        let ref_dir2_node = Rc::new(RefCell::new(dir2_node));
        let ref_file1_node = Rc::new(RefCell::new(file1_node));

        Dir::add_child(Rc::clone(&ref_dir1), ref_dir2_node.clone());

        Dir::add_child(Rc::clone(&ref_dir1), ref_file1_node.clone());

        let file1 = Rc::clone(&ref_file1_node);
        let f = file1.borrow().to_owned();
        if let Node::File(f) = f {
            dbg!(f.parent.upgrade());
        }

        let dir2 = Rc::clone(&ref_dir2_node);
        dbg!(dir2.borrow().to_owned().get_tree());
    }
}

use crate::output::OutputLine;

pub struct FileSystem {
    root: FileSystemNode,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            root: FileSystemNode::new_dir("/"),
        }
    }

    fn insert_node(&mut self, path: &Vec<String>, node: FileSystemNode) {
        let mut curr_dir = &mut self.root;
        for component in path {
            let Some(dir) = curr_dir.get_child(component) else {
                return;
            };
            curr_dir = dir;
        }
        curr_dir.add_child(node);
    }

    pub fn size(&self) -> usize {
        self.root.size
    }

    pub fn fold_dirs<A, F>(&self, init: A, mut f: F) -> A
    where
        F: FnMut(A, &FileSystemNode) -> A,
    {
        self.root.fold_dirs(init, &mut f)
    }
}

impl From<Vec<OutputLine>> for FileSystem {
    fn from(output: Vec<OutputLine>) -> Self {
        let mut fs = FileSystem::new();

        // Construct the file system (in an inefficient but simple way)
        let mut working_dir = vec![];
        for line in output {
            match line {
                OutputLine::LsEntry(node) => fs.insert_node(&working_dir, node),
                OutputLine::CdUp => working_dir.pop().map_or((), |_| ()),
                OutputLine::CdDown(dir) => working_dir.push(dir),
            }
        }

        // Then calculate all node sizes
        fs.root.annotate_size();

        fs
    }
}

pub struct FileSystemNode {
    pub name: String,
    pub size: usize,
    node_type: NodeType,
    children: Vec<FileSystemNode>,
}

impl FileSystemNode {
    pub fn new_file(name: &str, size: usize) -> Self {
        Self {
            name: String::from(name),
            size,
            node_type: NodeType::File,
            children: vec![],
        }
    }

    pub fn new_dir(name: &str) -> Self {
        Self {
            name: String::from(name),
            node_type: NodeType::Dir,
            size: 0,
            children: vec![],
        }
    }

    fn annotate_size(&mut self) {
        for child in self.children.iter_mut() {
            child.annotate_size();
            self.size += child.size;
        }
    }

    fn get_child(&mut self, name: &str) -> Option<&mut Self> {
        self.children.iter_mut().find(|child| child.name == name)
    }

    fn add_child(&mut self, child: FileSystemNode) {
        if let NodeType::Dir = self.node_type {
            self.children.push(child);
        }
    }

    fn fold_dirs<A, F>(&self, init: A, f: &mut F) -> A
    where
        F: FnMut(A, &FileSystemNode) -> A,
    {
        let mut acc = init;
        for child in &self.children {
            if let NodeType::Dir = child.node_type {
                acc = {
                    let acc = child.fold_dirs(acc, f);
                    f(acc, child)
                }
            }
        }
        acc
    }
}

enum NodeType {
    File,
    Dir,
}

use std::{
    cell::RefCell,
    fs,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum FileType {
    Dir,
    File,
}

#[derive(Debug)]
struct Node {
    file_type: FileType,
    size: u32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let input = fs::read_to_string("day_7/input.txt").unwrap();
    let input = input.trim().split_at(6).1.trim();

    let root = Rc::new(Node {
        file_type: FileType::Dir,
        size: 0,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    build_tree(input, &root);

    let (total_size, dirs) = sum_nodes(&root);
    let unused_space = 70_000_000 - total_size;
    let required_space = 30_000_000 - unused_space;

    let dir_size_sum: u32 = dirs.iter().filter(|d| **d <= 100_000).sum();
    let dir_to_clean = dirs
        .iter()
        .filter(|s| **s >= required_space)
        .min()
        .expect("No directory large enough found.");

    println!("Part 1: {}\nPart 2: {}", dir_size_sum, dir_to_clean);
}

fn build_tree(input: &str, root: &Rc<Node>) {
    let mut current_parent = Rc::clone(root);

    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();

        if split[0] == "$" {
            if split[1] == "cd" {
                if split[2] == ".." {
                    let one_above = current_parent
                        .parent
                        .borrow()
                        .upgrade()
                        .expect("Node doesn't have a parent.");
                    current_parent = Rc::clone(&one_above);

                    continue;
                }

                let new_node = Rc::new(Node {
                    file_type: FileType::Dir,
                    size: 0,
                    parent: RefCell::new(Rc::downgrade(&current_parent)),
                    children: RefCell::new(vec![]),
                });

                current_parent
                    .children
                    .borrow_mut()
                    .push(Rc::clone(&new_node));

                current_parent = new_node;
            }
        } else {
            if split[0] == "dir" {
                continue;
            }

            let (file_type, size) = (
                FileType::File,
                split[0]
                    .parse::<u32>()
                    .expect("File size couldn't be parsed to u32."),
            );

            let new_node = Rc::new(Node {
                file_type,
                size,
                parent: RefCell::new(Rc::downgrade(&current_parent)),
                children: RefCell::new(vec![]),
            });

            current_parent.children.borrow_mut().push(new_node);
        }
    }
}

fn sum_nodes(root: &Rc<Node>) -> (u32, Vec<u32>) {
    let mut sum: u32 = 0;
    let mut dir_sizes = vec![];

    for node in root.children.borrow().iter() {
        match node.file_type {
            FileType::File => sum += node.size,
            FileType::Dir => {
                let (current_size, mut current_dir_size) = sum_nodes(node);

                sum += current_size;
                dir_sizes.append(&mut current_dir_size);
                dir_sizes.push(current_size)
            }
        };
    }

    (sum, dir_sizes)
}

use std::{collections::HashMap, path::PathBuf};

fn main() {
    let mut location = PathBuf::new();
    let mut sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut sub_dirs: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();

    let mut lines = std::io::stdin().lines();
    assert_eq!(lines.next().unwrap().unwrap(), "$ cd /".to_string());
    location.push("/");

    for line in lines {
        let line = line.unwrap();
        if line == "$ cd .." {
            location.pop();
        } else if line.starts_with("$ cd ") {
            location.push(&line[5..]);
        } else if line == "$ ls" {
            // do nothing
        } else if line.starts_with("dir ") {
            let mut subdir_path = location.clone();
            subdir_path.push(&line[4..]);
            sub_dirs
                .entry(location.clone())
                .or_default()
                .push(subdir_path);
        } else {
            let (size, _) = line.split_once(' ').unwrap();
            *sizes.entry(location.clone()).or_default() += size.parse::<usize>().unwrap();
        }
    }

    let mut dir_tree = sub_dirs
        .into_iter()
        .collect::<Vec<(PathBuf, Vec<PathBuf>)>>();
    dir_tree.sort_by_key(|(parent, _)| usize::MAX - parent.as_os_str().len());

    for (parent, children) in dir_tree {
        *sizes.entry(parent.to_path_buf()).or_default() +=
            children.into_iter().map(|p| sizes[&p]).sum::<usize>();
    }

    let part_1: usize = sizes
        .iter()
        .filter_map(|(_, size)| (*size <= 100_000).then_some(*size))
        .sum();

    println!("Part 1: {part_1}");

    const TOTAL_SPACE: usize = 70_000_000;
    const REQUIRED_SPACE: usize = 30_000_000;

    let actually_used_space = sizes[AsRef::<std::path::Path>::as_ref("/")];
    let remaining_space = TOTAL_SPACE - actually_used_space;
    let additional_space_needed = REQUIRED_SPACE - remaining_space;

    let part_2 = sizes
        .iter()
        .filter(|(_, size)| **size >= additional_space_needed)
        .min_by_key(|(_, size)| **size);

    println!("Part 2: {:?}", part_2.unwrap().1);
}

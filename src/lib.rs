pub mod file_tree;
pub mod error;
mod utils;

use std::io::Write;
use std::path::PathBuf;
use std::fs;

use self::utils::directories;
use self::error::{Error, Result, ErrorKind};
use self::file_tree::node::{Node, Catalog, File};
use self::file_tree::FileTree;

pub fn create_catalog(path: &str, catalog: Catalog) -> Result<()> {
    let mut file_tree = get_file_tree()?;

    let node = Node::from(catalog);
    let parent_id = match file_tree.find_id_by_path(path) {
        Some(id) => id,
        None => return Err(Error::new(ErrorKind::PathNotExists(path.into()))),
    };

    file_tree.add(node, parent_id)?;
    save_file_tree(&file_tree)?;

    Ok(())
}

pub fn create_file(path: &str, file: File) -> Result<()> {
    let mut file_tree = get_file_tree()?;

    let node = Node::from(file);
    let parent_id = match file_tree.find_id_by_path(path) {
        Some(id) => id,
        None => return Err(Error::new(ErrorKind::PathNotExists(path.into()))),
    };

    file_tree.add(node, parent_id)?;
    save_file_tree(&file_tree)?;

    Ok(())
}

pub fn remove_entry(path: &str) -> Result<()> {
    let mut file_tree = get_file_tree()?;

    let node_id = match file_tree.find_id_by_path(path) {
        Some(id) => id,
        None => return Err(Error::new(ErrorKind::PathNotExists(path.into()))),
    };

    file_tree.remove(node_id)?;
    save_file_tree(&file_tree)?;

    Ok(())
}

pub fn rename_entry(path: &str, new_name: &str) -> Result<()> {
    let mut file_tree = get_file_tree()?;

    let node_id = match file_tree.find_id_by_path(path) {
        Some(id) => id,
        None => return Err(Error::new(ErrorKind::PathNotExists(path.into()))),
    };
    let node = file_tree.get_mut(node_id).unwrap();

    node.name = String::from(new_name);
    save_file_tree(&file_tree)?;

    Ok(())
}

 fn get_file_tree() -> Result<FileTree> {
    let file_tree_path = get_path_to_file_tree()?;
    let serialized_file_tree = fs::read_to_string(file_tree_path)?;
    let file_tree: FileTree = serde_json::from_str(&serialized_file_tree)?;

    Ok(file_tree)
}

fn save_file_tree(file_tree: &FileTree) -> Result<()> {
    let serialized_file_tree = serde_json::to_string_pretty(file_tree)?;
    let file_tree_file_path = get_path_to_file_tree()?;
    fs::write(file_tree_file_path, serialized_file_tree)?;

    Ok(())
}

fn get_path_to_file_tree() -> Result<PathBuf> {
    let mut file_path = directories::get_app_data_dir_and_create_if_not_exist()?;

    file_path.push("file_tree.json");

    if let Err(_) = fs::metadata(&file_path) {
        let mut file = fs::File::create(&file_path)?;
        let serialized_file_tree = serde_json::to_string_pretty(&FileTree::new())?;
        file.write(&serialized_file_tree.as_bytes())?;
    }

    Ok(file_path)
}

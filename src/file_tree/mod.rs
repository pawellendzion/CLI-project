pub mod node;
mod region;

use serde::{Serialize, Deserialize};
use region::*;
use node::*;

use super::error::{Result, Error, ErrorKind};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileTree<> {
    root_id: usize,
    region: Region<Node>,
}

impl FileTree {
    pub fn new() -> Self {
        let mut region = Region::new();
        let root = Node::from(Catalog::new(String::from("root")));
        let index = region.alloc_data(root).unwrap();
        Self { region, root_id: index }
    }

    pub fn with_nodes(mut nodes: Vec<Node>) -> Result<Self> {
        let mut region = Region::with_capacity(nodes.len() + 1);
        let mut root_id: Option<usize> = None;

        loop {
            if let Some(node) = nodes.pop() {
                if node.parent.is_none() {
                    root_id = Some(node.get_id().clone())
                }
                region.alloc_data(node).unwrap();
            } else {
                break;
            }
        }

        if root_id.is_none() {
            panic!("Root node doesn't exist");
        }

        Ok(Self { region, root_id: root_id.unwrap() })

    }

    pub fn add(&mut self, node: Node, parent_id: usize) -> Result<()> {
        if  self.region.get(parent_id).is_none() {
            return Err(Error::new(ErrorKind::NodeNotExists(parent_id)));
        }

        let index = self.region.alloc_data(node)?;

        let node = self.region.get_mut(index).unwrap();
        node.parent = Some(parent_id);

        let parent = self.region.get_mut(parent_id).unwrap();
        parent.children.push(index);

        Ok(())
    }

    pub fn remove(&mut self, node_id: usize) -> Result<()> {
        let node = match self.region.get(node_id) {
            Some(n) => n,
            None => return Err(Error::new(ErrorKind::NodeNotExists(node_id))),
        };

        if let Some(parent_id) = node.parent {
            let parent = self.region.get_mut(parent_id).unwrap();
            let node_index = parent.children.iter()
                .position(|e| *e == node_id).unwrap();

            parent.children.remove(node_index);
        }

        self.remove_recursive(node_id)?;

        Ok(())
    }

    pub fn get(&self, node_id: usize) -> Option<&Node> {
        self.region.get(node_id)
    }

    pub fn get_mut(&mut self, node_id: usize) -> Option<&mut Node> {
        self.region.get_mut(node_id)
    }

    pub fn find_id_by_path(&self, path: &str) -> Option<usize> {
        let path_parts = path.split('/');
        let mut node = self.region.get(self.root_id).unwrap();

        for part in path_parts {
            if part.is_empty() {
                continue;
            }
    
            let mut found = false;
            for child_id in node.children.clone() {
                let child = self.region.get(child_id).unwrap();
                if child.name == part {
                    node = child;
                    found = true;
                    break;
                }
            }

            if !found {
                return None;
            }
        }
        
        Some(node.get_id())
    }

    fn remove_recursive(&mut self, node_id: usize) -> Result<()> {
        let children = &mut self.region.get_mut(node_id).unwrap().children;

        if children.len() == 0 {
            self.region.free_slot(node_id)?;
            return Ok(());
        }

        for child_id in children.clone() {
            self.remove_recursive(child_id)?;
        }

        self.region.free_slot(node_id)?;

        Ok(())
    }
}


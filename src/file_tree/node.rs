use serde::{Serialize, Deserialize};

use super::region::Identifiable;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NodeType {
    Catalog,
    File,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Catalog {
    id: Option<usize>,
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Catalog {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            parent: None,
            children: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    id: Option<usize>,
    name: String,
    parent: Option<usize>,
    real_path: String,
    purposes: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub(crate) id: Option<usize>,
    pub(crate) name: String,
    pub(crate) parent: Option<usize>,
    pub(crate) children: Vec<usize>,

    node_type: NodeType,
    real_path: Option<String>,
    puropses: Option<Vec<String>>,
}

impl Identifiable for Node {
    fn get_id(&self) -> usize {
        self.id.unwrap()
    }

    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id.as_ref().unwrap() == other.id.as_ref().unwrap()
    }
}

impl TryInto<Catalog> for Node {
    type Error = ();

    fn try_into(self) -> Result<Catalog, Self::Error> {
        if let NodeType::Catalog = self.node_type {
            Ok(Catalog {
                id: self.id,
                name: self.name,
                parent: self.parent,
                children: self.children,
            })
        } else {
            Err(())
        }
    }
}

impl TryInto<File> for Node {
    type Error = ();

    fn try_into(self) -> Result<File, Self::Error> {
        if let NodeType::File = self.node_type {
            Ok(File {
                id: self.id,
                name: self.name,
                parent: self.parent,
                purposes: self.puropses.unwrap(),
                real_path: self.real_path.unwrap(),
            })
        } else {
            Err(())
        }
    }
}

impl From<Catalog> for Node {
    fn from(value: Catalog) -> Self {
        Node {
            id: value.id,
            name: value.name,
            parent: value.parent,
            children: value.children,
            node_type: NodeType::Catalog,
            puropses: None,
            real_path: None,
        }
    }
}

impl From<File> for Node {
    fn from(value: File) -> Self {
        Node {
            id: value.id,
            name: value.name,
            parent: value.parent,
            children: Vec::new(),
            node_type: NodeType::File,
            puropses: Some(value.purposes),
            real_path: Some(value.real_path),
        }
    }
}

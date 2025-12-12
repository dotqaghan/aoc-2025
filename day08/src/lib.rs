use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub enum Node {
    None,
    Parent(usize),
    Root,
}

#[derive(Clone)]
pub struct Forest(Vec<Node>);

impl Forest {
    pub fn with_size(size: usize) -> Forest {
        Forest(vec![Node::None; size])
    }
}

impl Index<usize> for Forest {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
impl IndexMut<usize> for Forest {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Forest {
    pub fn get_parent(&self, i: usize) -> Option<usize> {
        match self[i] {
            Node::Parent(i) => self.get_parent(i),
            Node::None => None,
            Node::Root => Some(i),
        }
    }
}

pub struct IVec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub fn distance_between(a: &IVec3, b: &IVec3) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;

    ((dx.pow(2) + dy.pow(2) + dz.pow(2)) as f64).sqrt()
}

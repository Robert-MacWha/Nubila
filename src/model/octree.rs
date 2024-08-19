use cgmath::{Point3, Vector3};

use super::{model::Model, voxel::Voxel};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Node {
    parent: u32,
    // data contains either the material of the voxel or the start index of the children
    // depending on the first bit of this field.
    // If the first bit is set, this is a leaf node and the data contains the material.
    // If the first bit is not set, this is an internal node and the data contains the start index
    data: u32,
}
implement_uniform_block!(Node, parent, data);

pub struct Octree {
    pos: Point3<u32>,
    size: u32,

    // If a internal node, this will contain the children
    children: Vec<Octree>,
    // If a leaf node, this may contain the voxel
    voxel: Option<Voxel>,
}

impl Node {
    pub fn new(parent: u32, data: u32) -> Self {
        Node { parent, data }
    }
}

impl Octree {
    pub fn new(model: &Model) -> Self {
        let model_size = model.size();
        let size = model_size.x.max(model_size.y).max(model_size.z);

        //? Octree sizes must be a power of 2
        let size = size.next_power_of_two();

        let mut root = Octree {
            pos: Point3::new(0, 0, 0),
            size,
            voxel: None,
            children: Vec::new(),
        };

        for voxel in model.voxels() {
            root.insert(voxel.clone());
        }

        return root;
    }

    // Serialize the octree into a list of nodes. Ensure that siblings are stored
    // sequentially.
    pub fn serialize(&self) -> Vec<Node> {
        let mut nodes = Vec::new();
        nodes.push(Node::new(0, 0)); // Root node
                                     // nodes[0].size = self.size;
                                     // nodes[0].pos = self.pos;

        // serialize all children breadth-first
        self.serialize_recursive(&mut nodes, 0);

        return nodes;
    }

    // serialize_recursive serializes the octree into a list of nodes recursively
    // in a breadth-first manner. The method ensures that siblings are stored
    // beside each other in a predictable order.
    //
    // Works by first appending all direct children to the node list, then
    // serializing the grandchildren and having them update their parents
    // with the start indices.
    fn serialize_recursive(&self, nodes: &mut Vec<Node>, parent: usize) {
        // iterate over all children and append them to the nodes list
        let child_start = nodes.len();
        for child in self.children.iter() {
            match &child.voxel {
                Some(voxel) => {
                    let node = Node::new(parent as u32, voxel.material());
                    nodes.push(node);
                }
                None => {
                    let node = Node::new(parent as u32, 0);
                    nodes.push(node);
                }
            }
        }

        // update the parent node with the start index of the children
        nodes[parent].data = child_start as u32;

        // serialize all children recursively
        for (i, child) in self.children.iter().enumerate() {
            let is_leaf = child.children.is_empty();
            if is_leaf {
                continue;
            }

            let child_index = child_start + i;
            child.serialize_recursive(nodes, child_index);
        }

        return;
    }

    fn insert(&mut self, voxel: Voxel) {
        //? If the voxel is outside the octree, ignore it
        if !self.contains(voxel.position()) {
            return;
        }

        //? If the octree is a leaf node, insert the voxel
        if self.size == 1 {
            self.voxel = Some(voxel);
            return;
        }

        //* Octree is an internal node
        //? If the octree is empty, split
        self.split();

        //? Insert the voxel into the children
        for child in self.children.iter_mut() {
            child.insert(voxel.clone());
        }
    }

    fn contains(&self, pos: Point3<u32>) -> bool {
        let min = self.pos;
        let max = self.pos + Vector3::new(self.size, self.size, self.size);

        return pos.x >= min.x
            && pos.x < max.x
            && pos.y >= min.y
            && pos.y < max.y
            && pos.z >= min.z
            && pos.z < max.z;
    }

    fn split(&mut self) {
        if !self.children.is_empty() {
            return;
        }

        let child_size = self.size / 2;
        for i in 0..8 {
            let child_pos = self.pos
                + Vector3::new(
                    (i & 1) * child_size,
                    ((i >> 1) & 1) * child_size,
                    ((i >> 2) & 1) * child_size,
                );

            self.children.push(Octree {
                pos: child_pos,
                size: child_size,
                voxel: None,
                children: Vec::new(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_insert() {
        let mut octree = Octree {
            pos: Point3::new(0, 0, 0),
            size: 1,
            voxel: None,
            children: Vec::new(),
        };

        let voxel = Voxel::new(Point3::new(0, 0, 0), 0, 0, 0);
        octree.insert(voxel);

        assert_eq!(octree.voxel.is_some(), true);
        assert_eq!(octree.children.len(), 0);
    }

    #[test]
    fn test_octree_insert_split() {
        let mut octree = Octree {
            pos: Point3::new(0, 0, 0),
            size: 2,
            voxel: None,
            children: Vec::new(),
        };

        let voxel = Voxel::new(Point3::new(0, 0, 0), 0, 0, 0);
        octree.insert(voxel);

        assert_eq!(octree.voxel.is_none(), true);
        assert_eq!(octree.children.len(), 8);
    }

    #[test]
    fn test_octree_contains() {
        let octree = Octree {
            pos: Point3::new(0, 0, 0),
            size: 8,
            voxel: None,
            children: Vec::new(),
        };

        assert_eq!(octree.contains(Point3::new(0, 0, 0)), true);
        assert_eq!(octree.contains(Point3::new(7, 7, 7)), true);
        assert_eq!(octree.contains(Point3::new(8, 8, 8)), false);
    }

    #[test]
    fn test_octree_split() {
        let mut octree = Octree {
            pos: Point3::new(0, 0, 0),
            size: 2,
            voxel: None,
            children: Vec::new(),
        };

        octree.split();

        assert_eq!(octree.children.len(), 8);
        assert_eq!(octree.children[0].pos, Point3::new(0, 0, 0));
        assert_eq!(octree.children[1].pos, Point3::new(1, 0, 0));
        assert_eq!(octree.children[2].pos, Point3::new(0, 1, 0));
        assert_eq!(octree.children[3].pos, Point3::new(1, 1, 0));
        assert_eq!(octree.children[4].pos, Point3::new(0, 0, 1));
        assert_eq!(octree.children[5].pos, Point3::new(1, 0, 1));
        assert_eq!(octree.children[6].pos, Point3::new(0, 1, 1));
        assert_eq!(octree.children[7].pos, Point3::new(1, 1, 1));
    }
}

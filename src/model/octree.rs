use cgmath::{Point3, Vector3};

use super::{model::Model, voxel::Voxel};

pub struct Node {
    child_start: u32,
    material: u32,
}

pub struct Octree {
    pos: Point3<u32>,
    size: u32,

    // If a internal node, this will contain the children
    children: Vec<Octree>,
    // If a leaf node, this may contain the voxel
    voxel: Option<Voxel>,
}

impl Node {
    pub fn new(child_start: u32, material: u32) -> Self {
        Node {
            child_start,
            material,
        }
    }
}

impl Octree {
    pub fn new(model: Model) -> Self {
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
                    let node = Node::new(0, voxel.material());
                    nodes.push(node);
                }
                None => {
                    let node = Node::new(0, 0);
                    nodes.push(node);
                }
            }
        }

        // update the parent node with the start index of the children
        nodes[parent].child_start = child_start as u32;

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

        for i in 0..8 {
            let child_pos = self.pos
                + Vector3::new(
                    (i & 1) * self.size / 2,
                    ((i >> 1) & 1) * self.size / 2,
                    ((i >> 2) & 1) * self.size / 2,
                );

            self.children.push(Octree {
                pos: child_pos,
                size: self.size / 2,
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
    fn test_serialize() {
        let mut octree = Octree {
            pos: Point3::new(0, 0, 0),
            size: 2,
            voxel: None,
            children: Vec::new(),
        };

        octree.insert(Voxel::new(Point3::new(0, 1, 0), 1));

        let nodes = octree.serialize();
        assert_eq!(nodes.len(), 9);
        assert_eq!(nodes[0].child_start, 1);
        assert_eq!(nodes[1].child_start, 0);
        assert_eq!(nodes[2].child_start, 0);
        assert_eq!(nodes[3].child_start, 0);
        assert_eq!(nodes[4].child_start, 0);
        assert_eq!(nodes[5].child_start, 0);
        assert_eq!(nodes[6].child_start, 0);
        assert_eq!(nodes[7].child_start, 0);
        assert_eq!(nodes[8].child_start, 0);

        assert_eq!(nodes[3].material, 1);
    }

    #[test]
    fn test_octree_insert() {
        let mut octree = Octree {
            pos: Point3::new(0, 0, 0),
            size: 1,
            voxel: None,
            children: Vec::new(),
        };

        let voxel = Voxel::new(Point3::new(0, 0, 0), 1);
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

        let voxel = Voxel::new(Point3::new(0, 0, 0), 1);
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
            size: 8,
            voxel: None,
            children: Vec::new(),
        };

        octree.split();

        assert_eq!(octree.children.len(), 8);
        assert_eq!(octree.children[0].pos, Point3::new(0, 0, 0));
        assert_eq!(octree.children[1].pos, Point3::new(4, 0, 0));
        assert_eq!(octree.children[2].pos, Point3::new(0, 4, 0));
        assert_eq!(octree.children[3].pos, Point3::new(4, 4, 0));
        assert_eq!(octree.children[4].pos, Point3::new(0, 0, 4));
        assert_eq!(octree.children[5].pos, Point3::new(4, 0, 4));
        assert_eq!(octree.children[6].pos, Point3::new(0, 4, 4));
        assert_eq!(octree.children[7].pos, Point3::new(4, 4, 4));
    }
}

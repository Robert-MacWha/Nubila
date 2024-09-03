use anyhow::anyhow;
use cgmath::{Point3, Vector3};

use super::{model::Model, voxel::Voxel};

// https://research.nvidia.com/sites/default/files/pubs/2010-02_Efficient-Sparse-Voxel/laine2010i3d_paper.pdf
// 0-23: child pointer (leaf node if zero)
// 24-31: child mask
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    value: u32,
    is_leaf: bool,
}

#[derive(Copy, Clone)]
pub struct Attribute {
    rgb: u32,
}
implement_uniform_block!(Attribute, rgb);

pub struct Octree {
    pos: Point3<u32>,
    size: u32,

    // If a internal node, this will contain the children
    children: Vec<Octree>,
    // If a leaf node, this may contain the voxel
    voxel: Option<Voxel>,
}

impl Node {
    pub fn new(ptr: u32, child_mask: u8) -> Self {
        let mut node = Node {
            value: 0,
            is_leaf: false,
        };
        node.set_pointer(ptr);
        node.set_child_mask(child_mask);
        return node;
    }

    pub fn new_leaf() -> Self {
        return Node {
            value: 0, // TODO: Is there a use for the bottom 8 bits?
            is_leaf: true,
        };
    }

    // Set the child pointer or far pointer (bits 0-23)
    pub fn set_pointer(&mut self, pointer: u32) {
        self.value = (self.value & 0xFF000000) | (pointer & 0x00FFFFFF);
    }

    // Set the child mask (bits 24-31)
    pub fn set_child_mask(&mut self, mask: u8) {
        self.value = (self.value & 0x00FFFFFF) | ((mask as u32) << 24);
    }

    pub fn pointer(&self) -> u32 {
        return self.value & 0x00FFFFFF;
    }

    pub fn child_mask(&self) -> u8 {
        return (self.value >> 24) as u8;
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_leaf {
            return write!(f, "Leaf {{ voxel: {} }}", self.value,);
        }
        write!(
            f,
            "Node {{ pointer: {}, child: {:08b} }}",
            self.pointer(),
            self.child_mask(),
        )
    }
}

impl Attribute {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        return Attribute {
            rgb: (r as u32) << 16 | (g as u32) << 8 | b as u32,
        };
    }
}

impl Octree {
    pub fn new(pos: Point3<u32>, size: u32) -> Self {
        let size = size.next_power_of_two();

        return Octree {
            pos,
            size,
            voxel: None,
            children: Vec::new(),
        };
    }

    pub fn from_model(model: &Model) -> Self {
        let pos = Point3::new(0, 0, 0);
        let model_size = model.size();
        let size = model_size.x.max(model_size.y).max(model_size.z) + 1;
        let size = size.next_power_of_two();

        let mut root = Octree {
            pos,
            size,
            voxel: None,
            children: Vec::new(),
        };

        for voxel in model.voxels() {
            root.insert(voxel);
        }

        return root;
    }

    // serializes the octree into a list of sections.
    pub fn serialize(&self) -> Result<(Vec<u32>, Vec<Attribute>), anyhow::Error> {
        let (nodes, attributes) = self.serialize_nodes()?;
        let nodes = nodes.iter().map(|node| node.value).collect();
        return Ok((nodes, attributes));
    }

    fn serialize_nodes(&self) -> Result<(Vec<Node>, Vec<Attribute>), anyhow::Error> {
        let mut nodes = Vec::new();
        nodes.push(Node::new(1, 0)); // root node

        let mut attributes = Vec::new();
        attributes.push(Attribute::new(255, 255, 255)); // root node
        self.serialize_recursive(&mut nodes, &mut attributes, 0)?;

        return Ok((nodes, attributes));
    }

    // serializes the octree
    // the serialization groups nodes into sections, where each section is a list of relatively referenced nodes.
    fn serialize_recursive(
        &self,
        nodes: &mut Vec<Node>,
        attributes: &mut Vec<Attribute>,
        parent_idx: u64,
    ) -> Result<u64, anyhow::Error> {
        let child_start = nodes.len() as u64 - parent_idx;

        // fetch the child indices, child mask, and leaf mask
        let mut child_count = 0;
        let mut child_indices: [u32; 8] = [0; 8];
        let mut child_mask: u32 = 0;
        let mut has_leaf = false;
        //? The compute shader processes the children in reverse order
        for (i, child) in self.children.iter().enumerate() {
            if let Some(_) = child.voxel {
                child_indices[child_count] = i as u32;
                child_mask |= 1 << i;
                child_count += 1;
                has_leaf = true;
            } else if !child.children.is_empty() {
                child_indices[child_count] = i as u32;
                child_mask |= 1 << i;
                child_count += 1;
            }
        }

        if has_leaf {
            // TODO: Figure out what to store in the leaf nodes instead of just
            // having them take up space for the attribute table.
            for i in 0..child_count {
                nodes.push(Node::new_leaf());

                let idx = child_indices[i] as usize;
                let color = self.children[idx].voxel.unwrap().color();
                attributes.push(Attribute::new(color.x, color.y, color.z));
            }
        } else {
            // pre-push all children
            for _ in 0..child_count {
                nodes.push(Node::new(0, 0));
                attributes.push(Attribute::new(255, 255, 255));
            }

            // serialize all children and store where the grandchildren start within the allocator
            let mut grandchild_starts: [u64; 8] = [0; 8];
            for i in 0..child_count {
                let idx = child_indices[i] as usize;
                grandchild_starts[i as usize] = self.children[idx].serialize_recursive(
                    nodes,
                    attributes,
                    parent_idx + child_start + i as u64,
                )?;

                //? Check if the pointer can be represented with 23 bits
                if grandchild_starts[i] > u64::pow(2, 23) {
                    return Err(anyhow!(
                        "far pointers not supported: {} > {}",
                        grandchild_starts[i],
                        i32::pow(2, 23)
                    ));
                }
            }

            // update the child nodes with the grandchild data
            for i in 0..child_count {
                let child_index = child_start + parent_idx + i as u64;
                let offset = grandchild_starts[i];

                nodes[child_index as usize].set_pointer(offset as u32);
            }
        }

        // update the parent node with the child data
        nodes[parent_idx as usize].set_child_mask(child_mask as u8);

        return Ok(child_start);
    }

    fn insert(&mut self, voxel: &Voxel) {
        // If the voxel is outside the octree, ignore it
        if !self.contains(voxel.position()) {
            return;
        }

        // If the octree is a leaf node, insert the voxel
        if self.size == 1 {
            self.voxel = Some(*voxel);
            return;
        }

        //* Octree is an internal node
        // If the octree is empty, split
        self.split();

        // Insert the voxel into the children
        for child in self.children.iter_mut() {
            child.insert(voxel);
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

        let child_size = self.size >> 1;
        for i in 0..8 {
            let child_pos = self.pos
                + Vector3::new(
                    (i & 1) * child_size,
                    ((i >> 2) & 1) * child_size,
                    ((i >> 1) & 1) * child_size,
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

impl std::fmt::Debug for Octree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{\"pos\": \"{:?}\", \"size\": \"{}\", \"voxel\": \"{:?}\", \"children\": {:?} }}",
            self.pos, self.size, self.voxel, self.children
        )
    }
}

#[cfg(test)]
mod tests {
    use cgmath::point3;

    use super::*;

    #[test]
    fn test_serialize_empty_octree() {
        let octree = Octree::new(Point3::new(0, 0, 0), 1);
        let (serialized, _) = octree.serialize().expect("Serialization failed");
        assert_eq!(serialized, vec![1]); // Expecting just the root node descriptor
    }

    #[test]
    fn test_serialize_single_voxel_octree() {
        let mut octree = Octree::new(Point3::new(0, 0, 0), 4);
        octree.insert(&Voxel::new(Point3::new(1, 1, 1), point3(255, 0, 0)));

        let (serialized, _) = octree.serialize_nodes().expect("Serialization failed");

        assert_eq!(serialized.len(), 3); // One for root, one for child
        assert_eq!(serialized[0], Node::new(1, 0b00000001));
        assert_eq!(serialized[1], Node::new(1, 0b10000000));
        assert_eq!(serialized[2], Node::new_leaf());
    }

    #[test]
    fn test_serialize_balanced_octree() {
        let mut octree = Octree::new(Point3::new(0, 0, 0), 8);
        octree.insert(&Voxel::new(Point3::new(0, 0, 0), Point3::new(255, 0, 0)));
        octree.insert(&Voxel::new(Point3::new(1, 1, 1), Point3::new(0, 255, 0)));
        octree.insert(&Voxel::new(Point3::new(2, 2, 2), Point3::new(0, 0, 255)));
        octree.insert(&Voxel::new(Point3::new(4, 4, 4), Point3::new(255, 255, 0)));

        let (serialized, _) = octree.serialize_nodes().expect("Serialization failed");

        assert_eq!(serialized.len(), 10);
        assert_eq!(serialized[0], Node::new(1, 0b10000001)); //
        assert_eq!(serialized[1], Node::new(2, 0b00000001)); // 0
        assert_eq!(serialized[2], Node::new(3, 0b10000001)); // 1
        assert_eq!(serialized[3], Node::new(1, 0b00000001)); // 0.0
        assert_eq!(serialized[4], Node::new_leaf()); // 0.0.0
        assert_eq!(serialized[5], Node::new(2, 0b00000001)); // 1.0
        assert_eq!(serialized[6], Node::new(2, 0b10000001)); // 1.1
        assert_eq!(serialized[7], Node::new_leaf()); // 1.0.0
        assert_eq!(serialized[8], Node::new_leaf()); // 1.1.0
        assert_eq!(serialized[9], Node::new_leaf()); // 1.1.1
    }

    #[test]
    fn test_octree_insert() {
        let mut octree = Octree::new(Point3::new(0, 0, 0), 1);

        octree.insert(&Voxel::new(Point3::new(0, 0, 0), Point3::new(0, 0, 0)));

        assert_eq!(octree.voxel.is_some(), true);
        assert_eq!(octree.children.len(), 0);
    }

    #[test]
    fn test_octree_insert_split() {
        let mut octree = Octree::new(Point3::new(0, 0, 0), 2);

        octree.insert(&Voxel::new(Point3::new(0, 0, 0), Point3::new(0, 0, 0)));

        assert_eq!(octree.voxel.is_none(), true);
        assert_eq!(octree.children.len(), 8);
    }

    #[test]
    fn test_octree_contains() {
        let octree = Octree::new(Point3::new(0, 0, 0), 8);

        assert_eq!(octree.contains(Point3::new(0, 0, 0)), true);
        assert_eq!(octree.contains(Point3::new(7, 7, 7)), true);
        assert_eq!(octree.contains(Point3::new(8, 8, 8)), false);
    }

    #[test]
    fn test_octree_split() {
        let mut octree = Octree::new(Point3::new(0, 0, 0), 2);

        octree.split();

        assert_eq!(octree.children.len(), 8);
        assert_eq!(octree.children[0].size, 1);
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

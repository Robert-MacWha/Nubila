// Based on the cpp implementation from https://github.com/tunabrain/sparse-voxel-octrees/blob/master/src/ChunkedAllocator.hpp

// Unused because I decided not to bother with chunked allocation for now. If
// I need a more picky solution for uploading data to the GPU I'll revisit it
// later.

pub struct ChunkedAllocator<T> {
    chunk_size: usize,
    size: usize,
    data: Vec<Box<[T]>>, // Vec of boxed arrays (chunks)
    insertions: Vec<InsertionPoint<T>>,
}

struct InsertionPoint<T> {
    idx: usize,
    data: T,
}

impl<T> ChunkedAllocator<T>
where
    T: Default + Clone,
{
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            size: 0,
            data: Vec::new(),
            insertions: Vec::new(),
        }
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insertion_count(&self) -> usize {
        self.insertions.len()
    }

    pub fn get(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.size {
            return None;
        }
        let chunk_idx = index / self.chunk_size;
        let element_idx = index % self.chunk_size;
        Some(&mut self.data[chunk_idx][element_idx])
    }

    pub fn push_back(&mut self, value: T) {
        if self.size % self.chunk_size == 0 {
            // Allocate a new chunk if the current chunk is full
            self.data
                .push(vec![T::default(); self.chunk_size].into_boxed_slice());
        }
        let chunk_idx = self.size / self.chunk_size;
        let element_idx = self.size % self.chunk_size;
        self.data[chunk_idx][element_idx] = value;
        self.size += 1;
    }

    /// Insert a value at the specified index. The value will be inserted at the
    /// specified index and all subsequent elements will be shifted to the right.
    ///
    /// Insertions are independant of each other and processed on calling "finalize".
    /// This leads to unintuitive behavior.
    pub fn insert(&mut self, index: usize, value: T) {
        self.insertions.push(InsertionPoint {
            idx: index,
            data: value,
        });
    }

    pub fn finalize(&mut self) -> Vec<T> {
        self.insertions
            .sort_unstable_by_key(|insertion| insertion.idx);

        let mut result = Vec::with_capacity(self.size + self.insertions.len());

        let mut insertion_count = 0;
        let mut input_count = 0;

        // Iterate over all items in the allocator and perform all insertions
        while input_count < self.size {
            let chunk_idx = input_count / self.chunk_size;
            let element_offset = input_count % self.chunk_size;
            let chunk = &self.data[chunk_idx];

            // Insert any element that is supposed to be inserted at this index
            //? Insertions are independant of each other.  See test_finalize_with_insertions
            //? for an example of unintuitive behavior.
            if insertion_count < self.insertions.len()
                && self.insertions[insertion_count].idx == input_count
            {
                result.push(self.insertions[insertion_count].data.clone());
                insertion_count += 1;
                continue;
            }

            // Copy the current element
            result.push(chunk[element_offset].clone());
            input_count += 1;
        }

        // If there are any insertions left, add them to the end
        while insertion_count < self.insertions.len() {
            result.push(self.insertions[insertion_count].data.clone());
            insertion_count += 1;
        }

        // Clear the allocator state
        self.data.clear();
        self.insertions.clear();
        self.size = 0;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_chunked_allocator() {
        let allocator = ChunkedAllocator::<i32>::new(4);
        assert_eq!(allocator.chunk_size(), 4);
        assert_eq!(allocator.size(), 0);
        assert_eq!(allocator.insertion_count(), 0);
    }

    #[test]
    fn test_push_back() {
        let mut allocator = ChunkedAllocator::<i32>::new(4);
        allocator.push_back(10);
        allocator.push_back(20);
        allocator.push_back(30);

        assert_eq!(allocator.size(), 3);

        assert_eq!(allocator.get(0), Some(&mut 10));
        assert_eq!(allocator.get(1), Some(&mut 20));
        assert_eq!(allocator.get(2), Some(&mut 30));
        assert_eq!(allocator.get(3), None); // Out of bounds
    }

    #[test]
    fn test_insertion() {
        let mut allocator: ChunkedAllocator<i32> = ChunkedAllocator::new(4);
        allocator.push_back(1);
        allocator.push_back(2);
        allocator.push_back(3);
        allocator.insert(1, 99); // Inserting 99 at index 1

        assert_eq!(allocator.size(), 3);
        assert_eq!(allocator.insertion_count(), 1);
    }

    #[test]
    fn test_finalize_without_insertions() {
        let mut allocator: ChunkedAllocator<i32> = ChunkedAllocator::new(4);
        allocator.push_back(5);
        allocator.push_back(15);
        allocator.push_back(25);

        let finalized = allocator.finalize();
        assert_eq!(finalized, vec![5, 15, 25]);

        // Ensure allocator is reset
        assert_eq!(allocator.size(), 0);
        assert_eq!(allocator.insertion_count(), 0);
    }

    #[test]
    fn test_finalize_with_insertions() {
        let mut allocator: ChunkedAllocator<i32> = ChunkedAllocator::new(4);
        allocator.push_back(1);
        allocator.push_back(2);
        allocator.push_back(3);
        allocator.push_back(4);

        allocator.insert(0, 10); // Insert at the beginning
        allocator.insert(1, 20); // Insert at index 1
        allocator.insert(2, 30); // Insert at index 2
        allocator.insert(3, 40); // Insert at index 3
        allocator.insert(6, 50); // Insert after all elements

        let finalized = allocator.finalize();

        //? Each insertion is independant and relative to the original allocator
        //? state prior to any insertions.  This leads to unintuitive behavior
        //? where "40" was inserted into index 3, but the final index is 4.
        //? Since the original allocator state is [1, 2, 3, 4], if we inserted
        //? 40 at index 3 then it would be between 3 and 4.
        //
        //? This also explains why, when inserting at indexes 0, 1, 2, 3; the
        //? inserted values have the original list between them.
        assert_eq!(finalized, vec![10, 1, 20, 2, 30, 3, 40, 4, 50]);

        // Ensure allocator is reset
        assert_eq!(allocator.size(), 0);
        assert_eq!(allocator.insertion_count(), 0);
    }

    #[test]
    fn test_finalize_large_allocator() {
        let mut allocator: ChunkedAllocator<i32> = ChunkedAllocator::new(4);

        for i in 0..10 {
            allocator.push_back(i);
        }

        allocator.insert(5, 50);
        allocator.insert(8, 80);

        let finalized = allocator.finalize();
        assert_eq!(finalized, vec![0, 1, 2, 3, 4, 50, 5, 6, 7, 80, 8, 9]);

        // Ensure allocator is reset
        assert_eq!(allocator.size(), 0);
        assert_eq!(allocator.insertion_count(), 0);
    }

    #[test]
    fn test_get() {
        let mut allocator: ChunkedAllocator<i32> = ChunkedAllocator::new(4);
        allocator.push_back(1);
        allocator.push_back(2);
        assert_eq!(allocator.get(1), Some(&mut 2));
    }

    #[test]
    fn test_get_out_of_bounds() {
        let mut allocator: ChunkedAllocator<i32> = ChunkedAllocator::new(4);
        allocator.push_back(1);
        allocator.push_back(2);
        assert_eq!(allocator.get(2), None); // Out of bounds
    }
}

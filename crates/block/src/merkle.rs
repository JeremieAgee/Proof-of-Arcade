use agee_primitives::Hash;

pub fn compute_merkle_root(_items: &[Vec<u8>]) -> Hash {
    // TODO: Implement merkle tree computation
    Hash::new([0u8; 32])
}

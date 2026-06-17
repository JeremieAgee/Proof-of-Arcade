#[cfg(test)]
mod tests {
    use crate::block::Block;
    use crate::header::BlockHeader;
    use crate::merkle::compute_merkle_root;
    use agee_primitives::Hash;

    fn sample_header() -> BlockHeader {
        BlockHeader {
            height: 1,
            timestamp_ms: 1000,
            previous_block_hash: Hash::new([0u8; 32]),
            tx_root: Hash::new([1u8; 32]),
            state_root: Hash::new([2u8; 32]),
            proposer: [3u8; 32],
        }
    }

    #[test]
    fn block_hash_deterministic() {
        let header = sample_header();
        let hash1 = header.hash();
        let hash2 = header.hash();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn different_headers_produce_different_hashes() {
        let mut header1 = sample_header();
        let mut header2 = sample_header();

        header2.height = 2;

        let hash1 = header1.hash();
        let hash2 = header2.hash();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn tx_root_changes_with_transactions() {
        let tx1 = b"tx_1".to_vec();
        let tx2 = b"tx_2".to_vec();

        let root1 = compute_merkle_root(&[tx1.clone()]);
        let root2 = compute_merkle_root(&[tx1.clone(), tx2.clone()]);

        assert_ne!(root1, root2);
    }

    #[test]
    fn state_root_changes_with_height() {
        let mut header1 = sample_header();
        let mut header2 = sample_header();

        header2.state_root = Hash::new([99u8; 32]);

        let hash1 = header1.hash();
        let hash2 = header2.hash();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn merkle_order_deterministic() {
        let txs = vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()];
        let root1 = compute_merkle_root(&txs);
        let root2 = compute_merkle_root(&txs);
        assert_eq!(root1, root2);
    }

    #[test]
    fn tampered_tx_root_changes_block_hash() {
        let mut header1 = sample_header();
        let mut header2 = sample_header();

        header2.tx_root = Hash::new([255u8; 32]); // Different tx root

        let hash1 = header1.hash();
        let hash2 = header2.hash();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn tamperedstate_root_changes_block_hash() {
        let mut header1 = sample_header();
        let mut header2 = sample_header();

        header2.state_root = Hash::new([255u8; 32]); // Different state root

        let hash1 = header1.hash();
        let hash2 = header2.hash();
        assert_ne!(hash1, hash2);
    }
}

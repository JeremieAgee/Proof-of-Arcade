use agee_primitives::Hash;
use sha2::{Digest, Sha256};

/// Compute merkle root from transaction list.
/// If empty, returns hash of "merkle_empty".
pub fn compute_merkle_root(transactions: &[Vec<u8>]) -> Hash {
    if transactions.is_empty() {
        let mut hasher = Sha256::new();
        hasher.update(b"merkle_empty");
        let result = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        return Hash::new(bytes);
    }

    let mut hashes: Vec<Hash> = transactions
        .iter()
        .map(|tx| {
            let mut hasher = Sha256::new();
            hasher.update(tx);
            let result = hasher.finalize();
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(&result);
            Hash::new(bytes)
        })
        .collect();

    while hashes.len() > 1 {
        let mut next_level = vec![];

        for i in (0..hashes.len()).step_by(2) {
            let left = hashes[i];
            let right = if i + 1 < hashes.len() {
                hashes[i + 1]
            } else {
                hashes[i]
            };

            let mut hasher = Sha256::new();
            hasher.update(&left.0);
            hasher.update(&right.0);
            let result = hasher.finalize();
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(&result);
            next_level.push(Hash::new(bytes));
        }

        hashes = next_level;
    }

    hashes[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_merkle_root_deterministic() {
        let root1 = compute_merkle_root(&[]);
        let root2 = compute_merkle_root(&[]);
        assert_eq!(root1, root2);
    }

    #[test]
    fn single_tx_hash_deterministic() {
        let tx = b"test_transaction".to_vec();
        let root1 = compute_merkle_root(&[tx.clone()]);
        let root2 = compute_merkle_root(&[tx.clone()]);
        assert_eq!(root1, root2);
    }

    #[test]
    fn different_txs_produce_different_roots() {
        let tx1 = b"transaction_1".to_vec();
        let tx2 = b"transaction_2".to_vec();
        let root1 = compute_merkle_root(&[tx1.clone()]);
        let root2 = compute_merkle_root(&[tx2.clone()]);
        assert_ne!(root1, root2);
    }

    #[test]
    fn order_matters_in_merkle() {
        let tx1 = b"tx_a".to_vec();
        let tx2 = b"tx_b".to_vec();
        let root1 = compute_merkle_root(&[tx1.clone(), tx2.clone()]);
        let root2 = compute_merkle_root(&[tx2.clone(), tx1.clone()]);
        assert_ne!(root1, root2);
    }
}

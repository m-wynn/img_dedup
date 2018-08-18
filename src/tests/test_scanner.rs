use crate::scanner::*;
use img_hash::HashType;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref test_data: Vec<(ImageHash, PathBuf)> = vec![
        (
            ImageHash {
                hash_type: HashType::Mean,
                bitv: BitVec::from_bytes(&[
                    0b0000_0001,
                    0b0000_0000,
                    0b0000_1110,
                    0b0001_1111,
                    0b1011_1111,
                    0b1011_1001,
                    0b1011_1011,
                    0b1011_1111,
                ]),
            },
            PathBuf::from("test/rustBsquish500.jpg"),
        ),
        (
            ImageHash {
                hash_type: HashType::Mean,
                bitv: BitVec::from_bytes(&[
                    0b1110_0011,
                    0b1111_0011,
                    0b1110_1111,
                    0b1110_0001,
                    0b0111_0011,
                    0b0111_1010,
                    0b0011_1110,
                    0b0011_1100,
                ]),
            },
            PathBuf::from("test/rustA500.jpg"),
        ),
        (
            ImageHash {
                hash_type: HashType::Mean,
                bitv: BitVec::from_bytes(&[
                    0b1110_0011,
                    0b1111_0011,
                    0b1110_1111,
                    0b1110_0001,
                    0b0111_0011,
                    0b0111_1010,
                    0b0011_1110,
                    0b0011_1100,
                ]),
            },
            PathBuf::from("test/rustA500_copy.jpg"),
        ),
        (
            ImageHash {
                hash_type: HashType::Mean,
                bitv: BitVec::from_bytes(&[
                    0b0100_0001,
                    0b0000_0000,
                    0b0000_1110,
                    0b0001_1111,
                    0b1011_1111,
                    0b1011_1001,
                    0b1011_0011,
                    0b1011_1011,
                ]),
            },
            PathBuf::from("test/rustB250.jpg"),
        ),
    ].into_iter()
    .collect();
}

#[test]
fn test_sort_ham() {
    let expected_result: Vec<(usize, (PathBuf, PathBuf))> = vec![
        (
            0,
            (
                PathBuf::from("test/rustA500.jpg"),
                PathBuf::from("test/rustA500_copy.jpg"),
            ),
        ),
        (
            3,
            (
                PathBuf::from("test/rustBsquish500.jpg"),
                PathBuf::from("test/rustB250.jpg"),
            ),
        ),
        (
            35,
            (
                PathBuf::from("test/rustBsquish500.jpg"),
                PathBuf::from("test/rustA500.jpg"),
            ),
        ),
        (
            35,
            (
                PathBuf::from("test/rustBsquish500.jpg"),
                PathBuf::from("test/rustA500_copy.jpg"),
            ),
        ),
        (
            36,
            (
                PathBuf::from("test/rustA500.jpg"),
                PathBuf::from("test/rustB250.jpg"),
            ),
        ),
        (
            36,
            (
                PathBuf::from("test/rustA500_copy.jpg"),
                PathBuf::from("test/rustB250.jpg"),
            ),
        ),
    ];
    let actual_results = sort_ham(test_data.to_vec()).into_sorted_vec();
    assert_eq!(actual_results.len(), expected_result.len());
    for ((e_dist, e_paths), (a_dist, a_paths)) in
        expected_result.into_iter().zip(actual_results.into_iter())
    {
        assert_eq!(a_dist, e_dist);
        assert!(a_paths == e_paths || (a_paths.1, a_paths.0) == e_paths)
    }
}

#[bench]
fn bench_standard(b: &mut Bencher) {
    let mut a_hash_bytes = [0u8, 64];
    let mut b_hash_bytes = [0u8, 64];
    thread_rng().fill(&mut a_hash_bytes[..]);
    thread_rng().fill(&mut b_hash_bytes[..]);
    let a_bits = BitVec::from_bytes(&a_hash_bytes);
    let b_bits = BitVec::from_bytes(&b_hash_bytes);

    b.iter(|| _dist(a_bits.clone(), b_bits.clone()))
}

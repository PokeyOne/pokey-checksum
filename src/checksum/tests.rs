use super::*;

#[test]
fn test_checksum_two_numbers() {
    let a = 0xE666;
    let b = 0xD555;

    let cs = Checksum::new()
        .add_data(a)
        .add_data(b)
        .checksum();

    assert_eq!(cs, 0b_0100_0100_0100_0011);
}

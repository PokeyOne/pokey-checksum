use super::*;

#[test]
fn test_psuedo_header() {
    let expected_header: [u8; 12] = [
        10, 0, 1, 5,
        128, 119, 245, 12,
        0, 6, 0, 28
    ];

    let src = ip_to_u32(10, 0, 1, 5);
    let dst = ip_to_u32(128, 119, 245, 12);

    let result = construct_psuedo_header(src, dst, 28, 0);

    assert_eq!(expected_header, result);
}

#[test]
fn example_checksum() {
    let data: Vec<u8> = vec![
        // psuedo header
        10, 0, 1, 5,
        128, 119, 245, 12,
        0, 6, 0, 28,
        // Actual header and zero data
        0x09, 0x54, 0x00, 0x50, 0xb9, 0x3c, 0x1f, 0x07, 0x00, 0x00, 0x00,
        0x00, 0x70, 0x02, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04,
        0x05, 0xb4, 0x01, 0x01, 0x04, 0x02
    ];

    let expected_checksum = 0xe0ae;

    let calculated = tcp_checksum(
        data[12..].to_vec(),
        vec![],
        ip_to_u32(10, 0, 1, 5),
        ip_to_u32(128, 119, 245, 12)
    );

    println!("{expected_checksum:X} vs {calculated:X}");

    assert_eq!(expected_checksum, calculated);
}

#[test]
fn example_checksum_raw() {
    let data: Vec<u16> = vec![
        // psuedo header
        10 << 8, (1 << 8) + 5,
        (128 << 8) + 119, (245 << 8) + 12,
        6, 28,
        // Actual header and zero data
        (0x09 << 8) + 0x54, 0x50, (0xb9 << 8) + 0x3c, (0x1f << 8) + 0x07, 0x00, 0x00,
        (0x70 << 8) + 0x02, 0x40 << 8, 0x00, 0x00, (0x02 << 8) + 0x04,
        (0x05 << 8) + 0xb4, (0x01 << 8) + 0x01, (0x04 << 8) + 0x02
    ];

    let expected_checksum = 0xe0ae;

    let calculated = Checksum::new().add_all_data(&data).checksum();

    println!("{expected_checksum:X} vs {calculated:X}");

    assert_eq!(expected_checksum, calculated);
}

fn ip_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    let a = a as u32;
    let b = b as u32;
    let c = c as u32;
    let d = d as u32;

    (a << 24) + (b << 16) + (c << 8) + d
}

#[test]
fn test_ip_to_u32() {
    let result = ip_to_u32(10, 0, 1, 5);
    let expected: u32 = 0x0A_00_01_05;

    assert_eq!(result, expected);
}

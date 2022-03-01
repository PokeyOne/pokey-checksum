/// The PTCL number of TCP.
const PTCL: u8 = 6;
/// The PTCL number of TCP.
pub const TCP_PTCL: u8 = PTCL;

pub fn tcp_checksum(
    header: Vec<u8>,
    data: Vec<u8>,
    source_address: u32,
    destination_address: u32
) {
    // Create the psuedo header to attach at the begining
    let header_len: u16 = octets(header.len());
    let data_len: u16 = octets(data.len());
    let psuedo_header: [u8; 12] = construct_psuedo_header(
        source_address,
        destination_address,
        header_len,
        data_len
    );
}

/// Count the number of octets in a size.
///
/// # Examples
/// ```
/// use pokey_checksum::octets;
///
/// assert_eq!(octets(63), 8_u16);
/// ```
pub fn octets(size: usize) -> u16 {
    let remainder = size % 8;
    let octet_count = size / 8; // intentionally floored division

    // Essentially round up if there is a decimal place
    if remainder != 0 {
        (octet_count + 1) as u16
    } else {
        octet_count as u16
    }
}

/// Construct the psuedo header that is included in the checksum.
///
/// # Arguments
///
/// - `src`: The 16-bit source address.
/// - `dst`: The 16-bit destination address.
/// - `header_len`: The length of the header **in octets**.
/// - `data_len`: The length of the data **in octets**.
///
/// # Examples
/// ```
/// use pokey_checksum::construct_psuedo_header;
///
/// let result: [u8; 12] = construct_psuedo_header(1, 3, 28, 8);
/// let expected: [u8; 12] = [
///     0, 0, 0, 1, // Source Address
///     0, 0, 0, 3, // Destination Address
///     0,          // Zero
///     6,          // PTCL number (TCP is 6)
///     0, 36       // TCP Length
/// ];
/// assert_eq!(result, expected);
/// ```
pub fn construct_psuedo_header(
    src: u32,
    dst: u32,
    header_len: u16,
    data_len: u16
) -> [u8; 12] {
    // Calculate the tcp length for the psuedo header
    let tcp_size: u16 = header_len + data_len;

    [
        // Source Address (32 bits)
        (src >> 24) as u8,
        ((src >> 16) & 0xFF) as u8,
        ((src >> 8) & 0xFF) as u8,
        (src & 0xFF) as u8,
        // Destination Address (32 bits)
        (dst >> 24) as u8,
        ((dst >> 16) & 0xFF) as u8,
        ((dst >> 8) & 0xFF) as u8,
        (dst & 0xFF) as u8,
        // Zero and PTCL
        0, PTCL,
        // Size
        (tcp_size >> 8) as u8,
        (tcp_size & 0xFF) as u8
    ]
}

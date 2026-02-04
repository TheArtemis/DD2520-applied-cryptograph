
use cryptopals::set2::padding::pkcs7_pad;

#[test]
fn test_challenge9_pkcs7_pad_to_block_length() {
    let data = b"YELLOW SUBMARINE";
    let block_size = 20;

    let padded = pkcs7_pad(data, block_size);

    let expected: Vec<u8> = [
        b"YELLOW SUBMARINE".as_slice(),
        &[0x04, 0x04, 0x04, 0x04],
    ]
    .concat();

    assert_eq!(padded.len(), block_size);
    assert_eq!(padded, expected, "padded should be 'YELLOW SUBMARINE' + 4 bytes of 0x04");
}

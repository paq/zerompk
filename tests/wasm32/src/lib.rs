//! wasm32 regression test

/// Err: the malformed message was rejected (the safe outcome).
const REJECTED: u32 = 0x0000_DEAD;
/// Ok: the malformed message was accepted (the overflow is present).
const ACCEPTED: u32 = 0x0000_ACCE;

#[derive(zerompk::FromMessagePack)]
#[msgpack(map, allow_unknown_fields)]
struct Tolerant {
    #[allow(dead_code)]
    a: u8,
}

/// `fixmap(2) { "a": 1, "b": ext32(len=0xFFFFFFFF) }` with the ext body
/// truncated. A correct decoder rejects it (the skip would need 0x1_0000_0000
/// bytes). On wasm32 release, `len + 1` wraps to 0, the skip consumes nothing,
/// and the bogus message is accepted.
#[unsafe(no_mangle)]
pub extern "C" fn skip_ext32_overflow() -> u32 {
    let data: &[u8] = &[
        0x82, 0xa1, b'a', 0x01, 0xa1, b'b', 0xc9, 0xff, 0xff, 0xff, 0xff,
    ];
    match zerompk::from_msgpack::<Tolerant>(data) {
        Ok(_) => ACCEPTED,
        Err(_) => REJECTED,
    }
}

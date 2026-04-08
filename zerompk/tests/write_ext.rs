use zerompk::{ToMessagePack, Write};

struct Ext<'a>(&'a [u8]);

impl ToMessagePack for Ext<'_> {
    fn write<W: Write>(&self, writer: &mut W) -> zerompk::Result<()> {
        writer.write_ext(5, self.0)
    }
}

// ==================== fixext1 ====================

#[test]
fn vec_writer_fixext1() {
    let result = zerompk::to_msgpack_vec(&Ext(&[0xab; 1])).unwrap();
    //                      fmt   type  data
    assert_eq!(result, vec![0xd4, 0x05, 0xab]);
}

#[test]
fn slice_writer_fixext1() {
    let mut buf = [0u8; 64];
    let n = zerompk::to_msgpack(&Ext(&[0xab; 1]), &mut buf).unwrap();
    //                      fmt   type  data
    assert_eq!(&buf[..n], &[0xd4, 0x05, 0xab]);
}

#[cfg(feature = "std")]
#[test]
fn io_writer_fixext1() {
    let mut buf = Vec::new();
    zerompk::write_msgpack(&mut buf, &Ext(&[0xab; 1])).unwrap();
    //                   fmt   type  data
    assert_eq!(buf, vec![0xd4, 0x05, 0xab]);
}

// ==================== fixext2 ====================

#[test]
fn vec_writer_fixext2() {
    let result = zerompk::to_msgpack_vec(&Ext(&[0xab; 2])).unwrap();
    //                      fmt   type  data..
    assert_eq!(result, vec![0xd5, 0x05, 0xab, 0xab]);
}

#[test]
fn slice_writer_fixext2() {
    let mut buf = [0u8; 64];
    let n = zerompk::to_msgpack(&Ext(&[0xab; 2]), &mut buf).unwrap();
    //                      fmt   type  data..
    assert_eq!(&buf[..n], &[0xd5, 0x05, 0xab, 0xab]);
}

#[cfg(feature = "std")]
#[test]
fn io_writer_fixext2() {
    let mut buf = Vec::new();
    zerompk::write_msgpack(&mut buf, &Ext(&[0xab; 2])).unwrap();
    //                   fmt   type  data..
    assert_eq!(buf, vec![0xd5, 0x05, 0xab, 0xab]);
}

// ==================== fixext4 ====================

#[test]
fn vec_writer_fixext4() {
    let result = zerompk::to_msgpack_vec(&Ext(&[0xab; 4])).unwrap();
    //                      fmt   type  data..
    assert_eq!(result, vec![0xd6, 0x05, 0xab, 0xab, 0xab, 0xab]);
}

#[test]
fn slice_writer_fixext4() {
    let mut buf = [0u8; 64];
    let n = zerompk::to_msgpack(&Ext(&[0xab; 4]), &mut buf).unwrap();
    //                      fmt   type  data..
    assert_eq!(&buf[..n], &[0xd6, 0x05, 0xab, 0xab, 0xab, 0xab]);
}

#[cfg(feature = "std")]
#[test]
fn io_writer_fixext4() {
    let mut buf = Vec::new();
    zerompk::write_msgpack(&mut buf, &Ext(&[0xab; 4])).unwrap();
    //                   fmt   type  data..
    assert_eq!(buf, vec![0xd6, 0x05, 0xab, 0xab, 0xab, 0xab]);
}

// ==================== fixext8 ====================

#[test]
fn vec_writer_fixext8() {
    let result = zerompk::to_msgpack_vec(&Ext(&[0xab; 8])).unwrap();
    //                fmt     type         data..
    let expected = [&[0xd7u8, 0x05][..], &[0xab; 8]].concat();
    assert_eq!(result, expected);
}

#[test]
fn slice_writer_fixext8() {
    let mut buf = [0u8; 64];
    let n = zerompk::to_msgpack(&Ext(&[0xab; 8]), &mut buf).unwrap();
    //                fmt     type         data..
    let expected = [&[0xd7u8, 0x05][..], &[0xab; 8]].concat();
    assert_eq!(&buf[..n], &expected);
}

#[cfg(feature = "std")]
#[test]
fn io_writer_fixext8() {
    let mut buf = Vec::new();
    zerompk::write_msgpack(&mut buf, &Ext(&[0xab; 8])).unwrap();
    //                fmt     type         data..
    let expected = [&[0xd7u8, 0x05][..], &[0xab; 8]].concat();
    assert_eq!(buf, expected);
}

// ==================== fixext16 ====================

#[test]
fn vec_writer_fixext16() {
    let result = zerompk::to_msgpack_vec(&Ext(&[0xab; 16])).unwrap();
    //                fmt     type         data..
    let expected = [&[0xd8u8, 0x05][..], &[0xab; 16]].concat();
    assert_eq!(result, expected);
}

#[test]
fn slice_writer_fixext16() {
    let mut buf = [0u8; 64];
    let n = zerompk::to_msgpack(&Ext(&[0xab; 16]), &mut buf).unwrap();
    //                fmt     type         data..
    let expected = [&[0xd8u8, 0x05][..], &[0xab; 16]].concat();
    assert_eq!(&buf[..n], &expected);
}

#[cfg(feature = "std")]
#[test]
fn io_writer_fixext16() {
    let mut buf = Vec::new();
    zerompk::write_msgpack(&mut buf, &Ext(&[0xab; 16])).unwrap();
    //                fmt     type         data..
    let expected = [&[0xd8u8, 0x05][..], &[0xab; 16]].concat();
    assert_eq!(buf, expected);
}

// ==================== ext8 ====================

#[test]
fn vec_writer_ext8() {
    let result = zerompk::to_msgpack_vec(&Ext(&[0xab; 7])).unwrap();
    //                fmt     size  type         data..
    let expected = [&[0xc7u8, 0x07, 0x05][..], &[0xab; 7]].concat();
    assert_eq!(result, expected);
}

#[test]
fn slice_writer_ext8() {
    let mut buf = [0u8; 64];
    let n = zerompk::to_msgpack(&Ext(&[0xab; 7]), &mut buf).unwrap();
    //                fmt     size  type         data..
    let expected = [&[0xc7u8, 0x07, 0x05][..], &[0xab; 7]].concat();
    assert_eq!(&buf[..n], &expected);
}

#[cfg(feature = "std")]
#[test]
fn io_writer_ext8() {
    let mut buf = Vec::new();
    zerompk::write_msgpack(&mut buf, &Ext(&[0xab; 7])).unwrap();
    //                fmt     size  type         data..
    let expected = [&[0xc7u8, 0x07, 0x05][..], &[0xab; 7]].concat();
    assert_eq!(buf, expected);
}

// ==================== ext16 ====================

#[test]
fn vec_writer_ext16() {
    let data = [0xab; 256];
    let result = zerompk::to_msgpack_vec(&Ext(&data)).unwrap();
    //                fmt     size..      type        data..
    let expected = [&[0xc8u8, 0x01, 0x00, 0x05][..], &data].concat();
    assert_eq!(result, expected);
}

#[test]
fn slice_writer_ext16() {
    let data = [0xab; 256];
    let mut buf = vec![0u8; 512];
    let n = zerompk::to_msgpack(&Ext(&data), &mut buf).unwrap();
    //                fmt     size..      type        data..
    let expected = [&[0xc8u8, 0x01, 0x00, 0x05][..], &data].concat();
    assert_eq!(&buf[..n], &expected);
}

#[cfg(feature = "std")]
#[test]
fn io_writer_ext16() {
    let data = [0xab; 256];
    let mut buf = Vec::new();
    zerompk::write_msgpack(&mut buf, &Ext(&data)).unwrap();
    //                fmt     size..      type        data..
    let expected = [&[0xc8u8, 0x01, 0x00, 0x05][..], &data].concat();
    assert_eq!(buf, expected);
}

// ==================== ext32 ====================

#[test]
fn vec_writer_ext32() {
    let data = vec![0xab; 65536];
    let result = zerompk::to_msgpack_vec(&Ext(&data)).unwrap();
    //                fmt     size..                  type
    let expected = [&[0xc9u8, 0x00, 0x01, 0x00, 0x00, 0x05][..], &data].concat();
    assert_eq!(result, expected);
}

#[test]
fn slice_writer_ext32() {
    let data = vec![0xab; 65536];
    let mut buf = vec![0u8; 65536 + 64];
    let n = zerompk::to_msgpack(&Ext(&data), &mut buf).unwrap();
    //                fmt     size..                  type
    let expected = [&[0xc9u8, 0x00, 0x01, 0x00, 0x00, 0x05][..], &data].concat();
    assert_eq!(&buf[..n], &expected);
}

#[cfg(feature = "std")]
#[test]
fn io_writer_ext32() {
    let data = vec![0xab; 65536];
    let mut buf = Vec::new();
    zerompk::write_msgpack(&mut buf, &Ext(&data)).unwrap();
    //                fmt     size..                  type
    let expected = [&[0xc9u8, 0x00, 0x01, 0x00, 0x00, 0x05][..], &data].concat();
    assert_eq!(buf, expected);
}

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_uint};

pub type sha1_digest_t = [u8; 20];

#[repr(C)]
pub struct SHA1Context {
    pub h0: u32,
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub h4: u32,
    pub nblocks: u32,
    pub buf: [u8; 64],
    pub count: c_int,
}

#[no_mangle]
pub extern "C" fn SHA1_Init(hd: *mut SHA1Context) {
    unsafe {
        let hd = &mut *hd;
        hd.h0 = 0x67452301;
        hd.h1 = 0xefcdab89;
        hd.h2 = 0x98badcfe;
        hd.h3 = 0x10325476;
        hd.h4 = 0xc3d2e1f0;
        hd.nblocks = 0;
        hd.count = 0;
    }
}

fn rol(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

fn transform(hd: &mut SHA1Context, data: &[u8; 64]) {
    let mut x = [0u32; 16];
    for i in 0..16 {
        let off = i * 4;
        x[i] = ((data[off] as u32) << 24)
            | ((data[off + 1] as u32) << 16)
            | ((data[off + 2] as u32) << 8)
            | (data[off + 3] as u32);
    }

    let mut a = hd.h0;
    let mut b = hd.h1;
    let mut c = hd.h2;
    let mut d = hd.h3;
    let mut e = hd.h4;

    const K1: u32 = 0x5A827999;
    const K2: u32 = 0x6ED9EBA1;
    const K3: u32 = 0x8F1BBCDC;
    const K4: u32 = 0xCA62C1D6;

    macro_rules! M {
        ($i:expr) => {{
            let tm = x[$i & 0x0f] ^ x[($i - 14) & 0x0f] ^ x[($i - 8) & 0x0f] ^ x[($i - 3) & 0x0f];
            x[$i & 0x0f] = rol(tm, 1);
            x[$i & 0x0f]
        }};
    }

    macro_rules! R {
        ($f:ident, $k:expr, $m:expr) => {{
            let temp = a
                .rotate_left(5)
                .wrapping_add($f(b, c, d))
                .wrapping_add(e)
                .wrapping_add($k)
                .wrapping_add($m);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }};
    }

    fn f1(x: u32, y: u32, z: u32) -> u32 {
        z ^ (x & (y ^ z))
    }
    fn f2(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }
    fn f3(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (z & (x | y))
    }
    fn f4(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    R!(f1, K1, x[0]);
    R!(f1, K1, x[1]);
    R!(f1, K1, x[2]);
    R!(f1, K1, x[3]);
    R!(f1, K1, x[4]);
    R!(f1, K1, x[5]);
    R!(f1, K1, x[6]);
    R!(f1, K1, x[7]);
    R!(f1, K1, x[8]);
    R!(f1, K1, x[9]);
    R!(f1, K1, x[10]);
    R!(f1, K1, x[11]);
    R!(f1, K1, x[12]);
    R!(f1, K1, x[13]);
    R!(f1, K1, x[14]);
    R!(f1, K1, x[15]);
    R!(f1, K1, M!(16));
    R!(f1, K1, M!(17));
    R!(f1, K1, M!(18));
    R!(f1, K1, M!(19));
    R!(f2, K2, M!(20));
    R!(f2, K2, M!(21));
    R!(f2, K2, M!(22));
    R!(f2, K2, M!(23));
    R!(f2, K2, M!(24));
    R!(f2, K2, M!(25));
    R!(f2, K2, M!(26));
    R!(f2, K2, M!(27));
    R!(f2, K2, M!(28));
    R!(f2, K2, M!(29));
    R!(f2, K2, M!(30));
    R!(f2, K2, M!(31));
    R!(f2, K2, M!(32));
    R!(f2, K2, M!(33));
    R!(f2, K2, M!(34));
    R!(f2, K2, M!(35));
    R!(f2, K2, M!(36));
    R!(f2, K2, M!(37));
    R!(f2, K2, M!(38));
    R!(f2, K2, M!(39));
    R!(f3, K3, M!(40));
    R!(f3, K3, M!(41));
    R!(f3, K3, M!(42));
    R!(f3, K3, M!(43));
    R!(f3, K3, M!(44));
    R!(f3, K3, M!(45));
    R!(f3, K3, M!(46));
    R!(f3, K3, M!(47));
    R!(f3, K3, M!(48));
    R!(f3, K3, M!(49));
    R!(f3, K3, M!(50));
    R!(f3, K3, M!(51));
    R!(f3, K3, M!(52));
    R!(f3, K3, M!(53));
    R!(f3, K3, M!(54));
    R!(f3, K3, M!(55));
    R!(f3, K3, M!(56));
    R!(f3, K3, M!(57));
    R!(f3, K3, M!(58));
    R!(f3, K3, M!(59));
    R!(f4, K4, M!(60));
    R!(f4, K4, M!(61));
    R!(f4, K4, M!(62));
    R!(f4, K4, M!(63));
    R!(f4, K4, M!(64));
    R!(f4, K4, M!(65));
    R!(f4, K4, M!(66));
    R!(f4, K4, M!(67));
    R!(f4, K4, M!(68));
    R!(f4, K4, M!(69));
    R!(f4, K4, M!(70));
    R!(f4, K4, M!(71));
    R!(f4, K4, M!(72));
    R!(f4, K4, M!(73));
    R!(f4, K4, M!(74));
    R!(f4, K4, M!(75));
    R!(f4, K4, M!(76));
    R!(f4, K4, M!(77));
    R!(f4, K4, M!(78));
    R!(f4, K4, M!(79));

    hd.h0 = hd.h0.wrapping_add(a);
    hd.h1 = hd.h1.wrapping_add(b);
    hd.h2 = hd.h2.wrapping_add(c);
    hd.h3 = hd.h3.wrapping_add(d);
    hd.h4 = hd.h4.wrapping_add(e);
}

fn sha1_update(hd: &mut SHA1Context, inbuf: Option<&[u8]>) {
    if hd.count == 64 {
        let buf_copy = hd.buf;
        transform(hd, &buf_copy);
        hd.count = 0;
        hd.nblocks += 1;
    }

    let Some(inbuf) = inbuf else { return };

    let mut inlen = inbuf.len();
    let mut pos = 0;

    if hd.count != 0 {
        while inlen > 0 && hd.count < 64 {
            hd.buf[hd.count as usize] = inbuf[pos];
            hd.count += 1;
            pos += 1;
            inlen -= 1;
        }
        let _buf_copy = hd.buf;
        sha1_update(hd, None);
        if inlen == 0 {
            return;
        }
    }

    while inlen >= 64 {
        let mut block = [0u8; 64];
        block.copy_from_slice(&inbuf[pos..pos + 64]);
        transform(hd, &block);
        hd.count = 0;
        hd.nblocks += 1;
        inlen -= 64;
        pos += 64;
    }

    while inlen > 0 && hd.count < 64 {
        hd.buf[hd.count as usize] = inbuf[pos];
        hd.count += 1;
        pos += 1;
        inlen -= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn SHA1_Update(hd: *mut SHA1Context, inbuf: *mut u8, inlen: usize) {
    let hd = &mut *hd;
    if inbuf.is_null() {
        sha1_update(hd, None);
    } else {
        let slice = std::slice::from_raw_parts(inbuf, inlen);
        sha1_update(hd, Some(slice));
    }
}

#[no_mangle]
pub unsafe extern "C" fn SHA1_Final(digest: *mut u8, hd: *mut SHA1Context) {
    let hd = &mut *hd;

    sha1_update(hd, None);

    let t = hd.nblocks;
    let mut lsb = t << 6;
    let mut msb = t >> 26;
    let t_orig = lsb;
    lsb = lsb.wrapping_add(hd.count as u32);
    if lsb < t_orig {
        msb += 1;
    }
    let t_after = lsb;
    lsb <<= 3;
    msb <<= 3;
    msb |= t_after >> 29;

    if hd.count < 56 {
        hd.buf[hd.count as usize] = 0x80;
        hd.count += 1;
        while hd.count < 56 {
            hd.buf[hd.count as usize] = 0;
            hd.count += 1;
        }
    } else {
        hd.buf[hd.count as usize] = 0x80;
        hd.count += 1;
        while hd.count < 64 {
            hd.buf[hd.count as usize] = 0;
            hd.count += 1;
        }
        let _buf_copy = hd.buf;
        sha1_update(hd, None);
        for i in 0..56 {
            hd.buf[i] = 0;
        }
    }

    hd.buf[56] = (msb >> 24) as u8;
    hd.buf[57] = (msb >> 16) as u8;
    hd.buf[58] = (msb >> 8) as u8;
    hd.buf[59] = msb as u8;
    hd.buf[60] = (lsb >> 24) as u8;
    hd.buf[61] = (lsb >> 16) as u8;
    hd.buf[62] = (lsb >> 8) as u8;
    hd.buf[63] = lsb as u8;

    let buf_copy = hd.buf;
    transform(hd, &buf_copy);

    hd.buf[0] = (hd.h0 >> 24) as u8;
    hd.buf[1] = (hd.h0 >> 16) as u8;
    hd.buf[2] = (hd.h0 >> 8) as u8;
    hd.buf[3] = hd.h0 as u8;
    hd.buf[4] = (hd.h1 >> 24) as u8;
    hd.buf[5] = (hd.h1 >> 16) as u8;
    hd.buf[6] = (hd.h1 >> 8) as u8;
    hd.buf[7] = hd.h1 as u8;
    hd.buf[8] = (hd.h2 >> 24) as u8;
    hd.buf[9] = (hd.h2 >> 16) as u8;
    hd.buf[10] = (hd.h2 >> 8) as u8;
    hd.buf[11] = hd.h2 as u8;
    hd.buf[12] = (hd.h3 >> 24) as u8;
    hd.buf[13] = (hd.h3 >> 16) as u8;
    hd.buf[14] = (hd.h3 >> 8) as u8;
    hd.buf[15] = hd.h3 as u8;
    hd.buf[16] = (hd.h4 >> 24) as u8;
    hd.buf[17] = (hd.h4 >> 16) as u8;
    hd.buf[18] = (hd.h4 >> 8) as u8;
    hd.buf[19] = hd.h4 as u8;

    std::ptr::copy_nonoverlapping(hd.buf.as_ptr(), digest, 20);
}

#[no_mangle]
pub unsafe extern "C" fn SHA1_UpdateInt32(context: *mut SHA1Context, val: c_uint) {
    let context = &mut *context;
    let buf = [
        ((val >> 24) & 0xff) as u8,
        ((val >> 16) & 0xff) as u8,
        ((val >> 8) & 0xff) as u8,
        (val & 0xff) as u8,
    ];
    sha1_update(context, Some(&buf));
}

#[no_mangle]
pub unsafe extern "C" fn SHA1_UpdateString(context: *mut SHA1Context, str: *mut c_char) {
    let context = &mut *context;
    let cstr = std::ffi::CStr::from_ptr(str);
    let bytes = cstr.to_bytes_with_nul();
    sha1_update(context, Some(bytes));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_char;

    fn make_ctx() -> SHA1Context {
        let mut ctx: SHA1Context = unsafe { std::mem::zeroed() };
        SHA1_Init(&mut ctx);
        ctx
    }

    fn sha1_hex(data: &[u8]) -> String {
        let mut ctx = make_ctx();
        unsafe {
            SHA1_Update(&mut ctx, data.as_ptr() as *mut u8, data.len());
            let mut digest = [0u8; 20];
            SHA1_Final(digest.as_mut_ptr(), &mut ctx);
            digest.iter().map(|b| format!("{:02x}", b)).collect()
        }
    }

    #[test]
    fn rfc_abc() {
        assert_eq!(sha1_hex(b"abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn rfc_448bit() {
        let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        assert_eq!(sha1_hex(msg), "84983e441c3bd26ebaae4aa1f95129e5e54670f1");
    }

    /// SHA1("") = da39a3ee5e6b4b0d3255bfef95601890afd80709
    #[test]
    fn empty_input() {
        assert_eq!(sha1_hex(b""), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    /// SHA1_UpdateInt32 sends the value in big-endian byte order, so its
    /// digest must match SHA1 of the four raw bytes.
    #[test]
    fn update_int32_big_endian() {
        let val: u32 = 0xDEADBEEF;
        let bytes = [
            ((val >> 24) & 0xFF) as u8,
            ((val >> 16) & 0xFF) as u8,
            ((val >> 8) & 0xFF) as u8,
            (val & 0xFF) as u8,
        ];
        let expected = sha1_hex(&bytes);

        let mut ctx = make_ctx();
        unsafe {
            SHA1_UpdateInt32(&mut ctx, val);
            let mut digest = [0u8; 20];
            SHA1_Final(digest.as_mut_ptr(), &mut ctx);
            let got: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
            assert_eq!(got, expected);
        }
    }

    /// SHA1_UpdateString feeds the string with its NUL terminator, so its
    /// digest must match SHA1 of the bytes including the trailing '\0'.
    #[test]
    fn update_string_includes_nul_terminator() {
        let expected = sha1_hex(b"doom\0");

        let mut ctx = make_ctx();
        let mut s = *b"doom\0";
        unsafe {
            SHA1_UpdateString(&mut ctx, s.as_mut_ptr() as *mut c_char);
            let mut digest = [0u8; 20];
            SHA1_Final(digest.as_mut_ptr(), &mut ctx);
            let got: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
            assert_eq!(got, expected);
        }
    }

    /// Feeding data in two separate SHA1_Update calls must yield the same
    /// digest as a single call with the concatenated data.
    #[test]
    fn incremental_update_matches_single_update() {
        let part1 = b"Hello, ";
        let part2 = b"world!";
        let combined = b"Hello, world!";

        let expected = sha1_hex(combined);

        let mut ctx = make_ctx();
        unsafe {
            SHA1_Update(&mut ctx, part1.as_ptr() as *mut u8, part1.len());
            SHA1_Update(&mut ctx, part2.as_ptr() as *mut u8, part2.len());
            let mut digest = [0u8; 20];
            SHA1_Final(digest.as_mut_ptr(), &mut ctx);
            let got: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
            assert_eq!(got, expected);
        }
    }

    /// Exactly 64 bytes of input fills one transform block exactly.
    #[test]
    fn exactly_one_block() {
        let data = [0x61u8; 64]; // 64 × 'a'
                                 // Must not panic and must produce a non-zero digest.
        let hex = sha1_hex(&data);
        assert_eq!(hex.len(), 40);
        assert_ne!(hex, "da39a3ee5e6b4b0d3255bfef95601890afd80709"); // != empty
    }

    /// Multi-block input (>64 bytes) exercises the internal loop.
    #[test]
    fn multi_block_input() {
        // 128 bytes — exactly two transform blocks
        let data = [0x62u8; 128]; // 128 × 'b'
        let hex = sha1_hex(&data);
        assert_eq!(hex.len(), 40);
        assert_ne!(hex, sha1_hex(b""));
    }
}

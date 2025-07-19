#![no_std]

use core::slice;
use core::mem::size_of;

const BLAKE2B_OUTBYTES: usize = 64;
const BLAKE2B_BLOCKBYTES: usize = 128;
const BLAKE2B_SALTBYTES: usize = 16;
const BLAKE2B_PERSONALBYTES: usize = 16;

const BLAKE2B_IV: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

const BLAKE2B_SIGMA: [[usize; 16]; 12] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], // 0
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3], // 1
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4], // 2
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8], // 3
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13], // 4
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9], // 5
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11], // 6
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10], // 7
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5], // 8
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0], // 9
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], // 10
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3], // 11
];

#[repr(C)]
struct Blake2bParam {
    pub digest_length: u8,                     /* 1 */
    pub key_length: u8,                        /* 2 */
    pub fanout: u8,                            /* 3 */
    pub depth: u8,                             /* 4 */
    pub leaf_length: u32,                      /* 8 */
    pub node_offset: u64,                      /* 16 */
    pub node_depth: u8,                        /* 17 */
    pub inner_length: u8,                      /* 18 */
    pub reserved: [u8; 14],                    /* 32 */
    pub salt: [u8; BLAKE2B_SALTBYTES],         /* 48 */
    pub personal: [u8; BLAKE2B_PERSONALBYTES], /* 64 */
}

#[derive(Debug)]
pub struct Blake2b {
    pub h: [u64; 8],
    pub t: [u64; 2],
    pub f: [u64; 2],
    pub buf: [u8; BLAKE2B_BLOCKBYTES],
    pub buflen: usize,
    pub outlen: usize,
    pub last_node: u8,
}

impl Default for Blake2b {
    fn default() -> Self {
        Self {
            h: [0; 8],
            t: [0; 2],
            f: [0; 2],
            buf: [0; BLAKE2B_BLOCKBYTES],
            buflen: 0,
            outlen: 0,
            last_node: 0,
        }
    }
}

impl Blake2b {
    pub fn new(salt: [u8; BLAKE2B_SALTBYTES], digest_length: u8) -> Self {
        let param = Blake2bParam {
            digest_length,
            key_length: 0,
            fanout: 1,
            depth: 1,
            leaf_length: 0,
            node_offset: 0,
            node_depth: 0,
            inner_length: 0,
            reserved: [0; 14],
            salt,
            personal: [0; BLAKE2B_PERSONALBYTES],
        };

        let mut h = [
            0x6a09e667f3bcc908,
            0xbb67ae8584caa73b,
            0x3c6ef372fe94f82b,
            0xa54ff53a5f1d36f1,
            0x510e527fade682d1,
            0x9b05688c2b3e6c1f,
            0x1f83d9abfb41bd6b,
            0x5be0cd19137e2179,
        ];

        let param_bytes: &[u8] = unsafe {
            slice::from_raw_parts(
                &param as *const Blake2bParam as *const u8,
                size_of::<Blake2bParam>(),
            )
        };

        for i in 0..8 {
            let offset = i * 8;
            let value = u64::from_le_bytes(param_bytes[offset..offset + 8].try_into().unwrap());
            h[i] ^= value;
        }

        Self {
            h,
            t: [0; 2],
            f: [0; 2],
            buf: [0; BLAKE2B_BLOCKBYTES],
            buflen: 0,
            outlen: param.digest_length as usize,
            last_node: 0,
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        if input.is_empty() || self.f[0] != 0 {
            return;
        }

        let input_len = input.len();

        let mut input = input;

        if self.buflen + input.len() > BLAKE2B_BLOCKBYTES {
            let left = self.buflen;
            let fill = BLAKE2B_BLOCKBYTES - left;

            // Complete the current block
            self.buf[left..left + fill].copy_from_slice(&input[..fill]);
            self.increment_counter(BLAKE2B_BLOCKBYTES as u64);
            let buf_ptr: *const [u8; BLAKE2B_BLOCKBYTES] = &self.buf;
            self.compress(unsafe { &*buf_ptr }); // no clone, no borrow conflict
            self.buflen = 0;

            input = &input[fill..];

            // Process full blocks directly from input
            while input.len() > BLAKE2B_BLOCKBYTES {
                self.increment_counter(BLAKE2B_BLOCKBYTES as u64);
                self.compress(<&[u8; 128]>::try_from(&input[..BLAKE2B_BLOCKBYTES]).unwrap());
                input = &input[BLAKE2B_BLOCKBYTES..];
            }
        }

        // Buffer the remaining input
        self.buf[self.buflen..self.buflen + input.len()].copy_from_slice(input);
        self.buflen += input_len;
    }

    pub fn finalize(&mut self, out: &mut [u8]) {
        if out.len() < self.outlen || self.f[0] != 0 {
            return;
        }

        // Finalize the hash state
        self.increment_counter(self.buflen as u64);
        self.set_last_block();

        // Zero padding
        for i in self.buflen..BLAKE2B_BLOCKBYTES {
            self.buf[i] = 0;
        }

        // Final compression
        let buf_ptr = self.buf.as_ptr() as *const [u8; BLAKE2B_BLOCKBYTES];
        self.compress(unsafe { &*buf_ptr });

        // Output buffer
        let mut buffer = [0u8; BLAKE2B_OUTBYTES];

        for (i, word) in self.h.iter().enumerate() {
            let offset = i * 8;
            buffer[offset..offset + 8].copy_from_slice(&word.to_le_bytes());
        }

        out.copy_from_slice(&buffer[..self.outlen]);
    }

    #[inline(always)]
    fn increment_counter(&mut self, inc: u64) {
        let (new_t0, carry) = self.t[0].overflowing_add(inc);
        self.t[0] = new_t0;
        self.t[1] = self.t[1].wrapping_add(carry as u64);
    }

    fn compress(&mut self, block: &[u8; BLAKE2B_BLOCKBYTES]) {
        let mut m = [0u64; 16];
        let mut v = [0u64; 16];

        // Load message block
        for i in 0..16 {
            let start = i * 8;
            m[i] = u64::from_le_bytes(block[start..start + 8].try_into().unwrap());
        }

        // Initialize work vector
        v[0..8].copy_from_slice(&self.h);
        v[8..16].copy_from_slice(&BLAKE2B_IV);

        v[12] ^= self.t[0];
        v[13] ^= self.t[1];
        v[14] ^= self.f[0];
        v[15] ^= self.f[1];

        for r in 0..12 {
            round(&mut v, &m, r);
        }

        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }
    }

    #[inline(always)]
    fn set_last_block(&mut self) {
        self.f[0] = u64::MAX;
    }
}

#[inline(always)]
fn round(v: &mut [u64; 16], m: &[u64; 16], r: usize) {
    g(v, m, r, 0, 1, 0, 4, 8, 12);
    g(v, m, r, 2, 3, 1, 5, 9, 13);
    g(v, m, r, 4, 5, 2, 6, 10, 14);
    g(v, m, r, 6, 7, 3, 7, 11, 15);
    g(v, m, r, 8, 9, 0, 5, 10, 15);
    g(v, m, r, 10, 11, 1, 6, 11, 12);
    g(v, m, r, 12, 13, 2, 7, 8, 13);
    g(v, m, r, 14, 15, 3, 4, 9, 14);
}

#[inline(always)]
fn rotr64(x: u64, n: usize) -> u64 {
    (x >> n) | (x << (64 - n))
}

#[inline(always)]
fn g(
    v: &mut [u64; 16],
    m: &[u64; 16],
    r: usize,
    i: usize,
    j: usize,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(m[BLAKE2B_SIGMA[r][i]]);
    v[d] = rotr64(v[d] ^ v[a], 32);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = rotr64(v[b] ^ v[c], 24);
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(m[BLAKE2B_SIGMA[r][j]]);
    v[d] = rotr64(v[d] ^ v[a], 16);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = rotr64(v[b] ^ v[c], 63);
}

#[cfg(test)]
mod test {
    use crate::Blake2b;
    use print_no_std::println;

    #[test]
    fn test_blake2() {
        let mut blake = Blake2b::new(*b"HashX v1\0\0\0\0\0\0\0\0", 64);
        blake.update(b"hduoc2003");
        let mut digest = [0u8; 64];
        blake.finalize(&mut digest);

        assert_eq!(
            digest,
            [
                123, 45, 5, 153, 52, 20, 101, 218, 147, 248, 148, 76, 5, 248, 52, 182, 24, 115,
                224, 185, 180, 129, 231, 225, 213, 116, 6, 26, 177, 153, 111, 117, 237, 72, 29,
                183, 46, 145, 39, 73, 39, 10, 105, 238, 26, 91, 141, 112, 175, 89, 5, 66, 139, 102,
                77, 127, 180, 168, 177, 202, 230, 246, 226, 14
            ]
        );
    }
}

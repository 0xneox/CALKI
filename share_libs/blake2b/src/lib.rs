

extern "C" {
    pub fn blake2b(out: *mut u8, outlen: usize, input: *const u8, inputlen: usize, key: *const u8, keylen: usize) -> i64;
}

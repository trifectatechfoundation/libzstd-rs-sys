use libc::size_t;

use crate::lib::common::fse_decompress::Error;

pub type BitContainerType = usize;

/// Bitstream decoder
///
/// *  The `new` method stores a chunk of the bitStream is into a local register.
/// *  Local register size is 64-bits on 64-bits systems, 32-bits on 32-bits systems.
/// *  You can then retrieve bitFields stored into the local register, **in reverse order**.
/// *  Local register is explicitly reloaded from memory by the [`reload`] method.
/// *  A reload guarantee a minimum of ((8*sizeof(bitD->bitContainer))-7) bits when its result is BIT_DStream_unfinished.
/// *  Otherwise, it can be less than that, so proceed accordingly.
/// *  Checking if DStream has reached its end can be performed with BIT_endOfDStream().
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct BIT_DStream_t {
    pub bitContainer: usize,
    pub bitsConsumed: core::ffi::c_uint,
    pub ptr: *const core::ffi::c_char,
    pub start: *const core::ffi::c_char,
    pub limitPtr: *const core::ffi::c_char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamStatus {
    /// Fully refilled.
    Unfinished = 0,
    /// Still some bits left in the bitstream.
    EndOfBuffer = 1,
    /// Bitstream entirely consumed, bit-exact.
    Completed = 2,
    /// User requested more bits than present in bitstream.
    Overflow = 3,
}

#[rustfmt::skip]
static MASK: [u32; 32] = [
    0,          1,         3,         7,         0xF,       0x1F,
    0x3F,       0x7F,      0xFF,      0x1FF,     0x3FF,     0x7FF,
    0xFFF,      0x1FFF,    0x3FFF,    0x7FFF,    0xFFFF,    0x1FFFF,
    0x3FFFF,    0x7FFFF,   0xFFFFF,   0x1FFFFF,  0x3FFFFF,  0x7FFFFF,
    0xFFFFFF,   0x1FFFFFF, 0x3FFFFFF, 0x7FFFFFF, 0xFFFFFFF, 0x1FFFFFFF,
    0x3FFFFFFF, 0x7FFFFFFF,
];

#[inline(always)]
const fn get_middle_bits(
    bitContainer: BitContainerType,
    start: u32,
    nbBits: u32,
) -> BitContainerType {
    // if start > regMask, the bitstream is corrupted, and the result is undefined.
    assert!(nbBits < MASK.len() as u32);

    const REG_MASK: usize = size_of::<BitContainerType>() * 8 - 1;

    if cfg!(target_arch = "x86_64") {
        // x86 transform & ((1 << nbBits) - 1) to bzhi instruction, it is better
        // than accessing memory. When bmi2 instruction is not present, we consider
        // such cpus old (pre-Haswell, 2013) and their performance is not of that
        // importance.
        bitContainer >> (start & REG_MASK as u32) & (1usize << nbBits).wrapping_sub(1)
    } else {
        bitContainer >> (start & REG_MASK as u32) & MASK[nbBits as usize] as usize
    }
}

impl BIT_DStream_t {
    pub fn new(mut srcBuffer: &[u8]) -> Result<Self, Error> {
        let mut bitD = Self {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: core::ptr::null::<core::ffi::c_char>(),
            start: core::ptr::null::<core::ffi::c_char>(),
            limitPtr: core::ptr::null::<core::ffi::c_char>(),
        };

        if srcBuffer.is_empty() {
            return Err(Error::srcSize_wrong);
        }

        const USIZE_BYTES: usize = size_of::<BitContainerType>();

        if let Some(chunk) = srcBuffer.last_chunk() {
            bitD.start = srcBuffer.as_ptr() as *const core::ffi::c_char;
            bitD.limitPtr = bitD.start.wrapping_add(USIZE_BYTES);

            bitD.ptr = (srcBuffer.as_ptr() as *const core::ffi::c_char)
                .wrapping_add(srcBuffer.len())
                .wrapping_sub(USIZE_BYTES);
            bitD.bitContainer = usize::from_le_bytes(*chunk) as size_t;

            match srcBuffer.last().and_then(|v| v.checked_ilog2()) {
                None => {
                    /* endMark not present */
                    return Err(Error::GENERIC);
                }
                Some(v) => {
                    bitD.bitsConsumed = 8 - v;
                }
            }
        } else {
            bitD.start = srcBuffer.as_ptr() as *const core::ffi::c_char;
            bitD.limitPtr = bitD.start.wrapping_add(USIZE_BYTES);
            bitD.ptr = bitD.start;

            bitD.bitContainer = usize::from(srcBuffer[0]);

            if srcBuffer.len() >= 7 {
                bitD.bitContainer += usize::from(srcBuffer[6]) << (USIZE_BYTES * 8 - 16);
            }
            if srcBuffer.len() >= 6 {
                bitD.bitContainer += usize::from(srcBuffer[5]) << (USIZE_BYTES * 8 - 24);
            }
            if srcBuffer.len() >= 5 {
                bitD.bitContainer += usize::from(srcBuffer[4]) << (USIZE_BYTES * 8 - 32);
            }
            if srcBuffer.len() >= 4 {
                bitD.bitContainer += usize::from(srcBuffer[3]) << 24;
            }
            if srcBuffer.len() >= 3 {
                bitD.bitContainer += usize::from(srcBuffer[2]) << 16;
            }
            if srcBuffer.len() >= 2 {
                bitD.bitContainer += usize::from(srcBuffer[1]) << 8;
            }

            match srcBuffer.last().and_then(|v| v.checked_ilog2()) {
                None => {
                    /* endMark not present */
                    return Err(Error::corruption_detected);
                }
                Some(v) => {
                    bitD.bitsConsumed = 8 - v;
                }
            }

            bitD.bitsConsumed += ((USIZE_BYTES - srcBuffer.len()) * 8) as u32;
        }

        Ok(bitD)
    }

    /// Provides the next n bits from local register.
    /// The local register is not modified.
    ///
    /// - On 32-bit systems, `maxNbBits==24`.
    /// - On 64-bit systems, `maxNbBits==56`.
    #[inline(always)]
    const fn look_bits(&self, nbBits: u32) -> BitContainerType {
        get_middle_bits(
            self.bitContainer,
            (size_of::<BitContainerType>() as u64)
                .wrapping_mul(8)
                .wrapping_sub(self.bitsConsumed as core::ffi::c_ulong)
                .wrapping_sub(nbBits as core::ffi::c_ulong) as u32,
            nbBits,
        )
    }

    /// Like [`look_bits`], but only works when `nbBits >= 1`
    #[inline]
    pub const fn look_bits_fast(&self, nbBits: u32) -> BitContainerType {
        // quickcheck hits this
        // debug_assert!(nbBits > 1);

        const MASK: u32 = (size_of::<BitContainerType>() * 8 - 1) as u32;
        (*self).bitContainer << ((*self).bitsConsumed & MASK)
            >> (MASK.wrapping_add(1).wrapping_sub(nbBits) & MASK)
    }

    #[inline(always)]
    pub const fn skip_bits(&mut self, mut nbBits: u32) {
        self.bitsConsumed += nbBits;
    }

    /// Read (consume) next n bits from local register and update.
    ///
    /// Pay attention to not read more than nbBits contained into local register.
    #[inline(always)]
    pub const fn read_bits(&mut self, nbBits: core::ffi::c_uint) -> BitContainerType {
        let value = self.look_bits(nbBits);
        self.skip_bits(nbBits);
        value
    }

    /// Like [`read_bits`], but only works when `nbBits >= 1`
    #[inline]
    pub const fn read_bits_fast(&mut self, nbBits: core::ffi::c_uint) -> BitContainerType {
        // quickcheck hits this
        // debug_assert!(nbBits > 1);

        let value = self.look_bits_fast(nbBits);
        self.skip_bits(nbBits);
        value
    }

    ///  Simple variant of [`reload`] with two conditions:
    ///
    ///  1. bitstream is valid : bitsConsumed <= sizeof(bitD->bitContainer)*8
    ///  2. look window is valid after shifted down : bitD->ptr >= bitD->start
    #[inline]
    unsafe fn reload_internal(&mut self) {
        // quickcheck hits this
        // debug_assert!(self.bitsConsumed as usize >= 8 * size_of::<usize>());
        self.ptr = unsafe { (self.ptr).sub(self.bitsConsumed as usize / 8) };
        debug_assert!(self.ptr >= self.start);
        self.bitsConsumed &= 7;
        self.bitContainer = unsafe { core::ptr::read_unaligned(self.ptr as *const usize) }.to_le();
    }

    #[inline]
    pub fn reload_fast(&mut self) -> StreamStatus {
        if self.ptr < self.limitPtr {
            StreamStatus::Overflow
        } else {
            unsafe { self.reload_internal() };
            StreamStatus::Unfinished
        }
    }

    ///  Refill from the buffer
    ///
    ///  ## Returns
    ///
    /// status of the internal register. when `status == StreamStatus::Unfinished`,
    /// the internal register is filled with at least 25 or 57 bits.
    #[inline(always)]
    pub fn reload(&mut self) -> StreamStatus {
        if self.bitsConsumed > (size_of::<BitContainerType>() as u32) * 8 {
            static zeroFilled: BitContainerType = 0;
            self.ptr = &zeroFilled as *const BitContainerType as *const core::ffi::c_char;

            return StreamStatus::Overflow;
        }

        if self.ptr >= self.limitPtr {
            unsafe { self.reload_internal() };

            return StreamStatus::Unfinished;
        }

        if self.ptr == self.start {
            return if self.bitsConsumed < size_of::<BitContainerType>() as u32 * 8 {
                StreamStatus::EndOfBuffer
            } else {
                StreamStatus::Completed
            };
        }

        let mut nbBytes = self.bitsConsumed / 8;
        let result = if unsafe { self.ptr.sub(nbBytes as usize) } < self.start {
            nbBytes = unsafe { self.ptr.offset_from(self.start) } as u32;

            StreamStatus::EndOfBuffer
        } else {
            StreamStatus::Unfinished
        };

        self.ptr = unsafe { self.ptr.sub(nbBytes as usize) };
        self.bitsConsumed = (self.bitsConsumed).wrapping_sub(nbBytes * 8);
        self.bitContainer = unsafe { core::ptr::read_unaligned(self.ptr as *const usize) }.to_le();

        result
    }

    pub fn is_empty(&self) -> bool {
        self.ptr == self.start
            && self.bitsConsumed as usize == size_of::<BitContainerType>().wrapping_mul(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty_buffer() {
        let result = BIT_DStream_t::new(&[]);
        assert!(result.is_err(), "Should fail on empty input");
    }

    #[test]
    fn test_new_ilog2_undefined() {
        let buffer = vec![0b00000000];
        let result = BIT_DStream_t::new(&buffer);
        assert!(result.is_err(), "Should fail when ilog2 is undefined");
    }

    const END_MARK: u8 = 0b00001000;
    const END_MARK_LOG: u32 = END_MARK.ilog2();

    #[test]
    fn test_new_success() {
        let buffer = [0b10101010, 0b11110000, END_MARK];
        let stream = BIT_DStream_t::new(&buffer).expect("BIT_DStream_t::new should succeed");
        assert!(stream.ptr >= stream.start);
        assert!(stream.bitContainer != 0);
    }

    #[test]
    fn test_look_and_read_bits() {
        let buffer = [0b10101010, 0b11001100, END_MARK];
        let mut stream = BIT_DStream_t::new(&buffer).unwrap();

        let consumed = stream.bitsConsumed;

        let bits = stream.look_bits(12 + END_MARK_LOG);
        assert_eq!(bits, 0b1100_1100_1010);

        let read = stream.read_bits(12 + END_MARK_LOG);
        assert_eq!(bits, read);

        assert_eq!(stream.bitsConsumed - consumed, 12 + END_MARK_LOG);
    }

    #[test]
    fn test_look_and_read_bits_fast() {
        let buffer = [0xFF, 0xAA, END_MARK];
        let mut stream = BIT_DStream_t::new(&buffer).unwrap();

        let consumed = stream.bitsConsumed;

        let bits = stream.look_bits_fast(12 + END_MARK_LOG);
        assert_eq!(bits, 0xAAF);

        let read = stream.read_bits_fast(20 + END_MARK_LOG);
        assert_eq!(read, 0xAAFF0);

        assert_eq!(stream.bitsConsumed - consumed, 20 + END_MARK_LOG);
    }

    #[test]
    fn test_end_mark() {
        let end_mark = 0b0001_0000u8;
        let end_mark_log = end_mark.ilog2();

        let buffer = [0b10101010, 0b11001100, end_mark];
        let mut stream = BIT_DStream_t::new(&buffer).unwrap();

        let consumed = stream.bitsConsumed;

        let bits = stream.look_bits(12 + end_mark_log);
        assert_eq!(bits, 0b1100_1100_1010);

        let read = stream.read_bits(12 + end_mark_log);
        assert_eq!(bits, read);

        assert_eq!(stream.bitsConsumed - consumed, 12 + end_mark_log);
    }

    #[test]
    fn test_reload_end_of_buffer() {
        let buffer = [0b00001111, END_MARK];
        let mut stream = BIT_DStream_t::new(&buffer).unwrap();

        assert!(
            matches!(
                stream.reload(),
                StreamStatus::EndOfBuffer | StreamStatus::Completed
            ),
            "Should be at end of buffer or complete"
        );
    }

    #[test]
    fn test_reload_overflow() {
        let mut stream = BIT_DStream_t {
            bitContainer: 0,
            bitsConsumed: (size_of::<usize>() * 8 + 1) as u32,
            ptr: core::ptr::null(),
            start: core::ptr::null(),
            limitPtr: core::ptr::null(),
        };

        assert_eq!(stream.reload(), StreamStatus::Overflow);
    }
}

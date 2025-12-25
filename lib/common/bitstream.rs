use core::marker::PhantomData;

use libc::size_t;

use crate::lib::common::error_private::Error;
use crate::lib::common::mem::MEM_writeLEST;

pub(crate) type BitContainerType = usize;

#[repr(C)]
pub(crate) struct BIT_CStream_t {
    // FIXME make all fields private to this module
    pub(crate) bitContainer: BitContainerType,
    pub(crate) bitPos: core::ffi::c_uint,
    pub(crate) startPtr: *mut core::ffi::c_char,
    pub(crate) ptr: *mut core::ffi::c_char,
    pub(crate) endPtr: *mut core::ffi::c_char,
}

#[inline]
pub(crate) unsafe fn BIT_initCStream(
    bitC: *mut BIT_CStream_t,
    startPtr: *mut core::ffi::c_void,
    dstCapacity: size_t,
) -> size_t {
    (*bitC).bitContainer = 0;
    (*bitC).bitPos = 0;
    (*bitC).startPtr = startPtr as *mut core::ffi::c_char;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr = ((*bitC).startPtr)
        .add(dstCapacity)
        .offset(-(::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong as isize));
    if dstCapacity <= ::core::mem::size_of::<BitContainerType>() {
        return Error::dstSize_tooSmall.to_error_code();
    }
    0
}

static BIT_mask: [core::ffi::c_uint; 32] = [
    0, 1, 3, 7, 0xf, 0x1f, 0x3f, 0x7f, 0xff, 0x1ff, 0x3ff, 0x7ff, 0xfff, 0x1fff, 0x3fff, 0x7fff,
    0xffff, 0x1ffff, 0x3ffff, 0x7ffff, 0xfffff, 0x1fffff, 0x3fffff, 0x7fffff, 0xffffff, 0x1ffffff,
    0x3ffffff, 0x7ffffff, 0xfffffff, 0x1fffffff, 0x3fffffff, 0x7fffffff,
];

#[inline(always)]
unsafe fn BIT_getLowerBits(bitContainer: BitContainerType, nbBits: u32) -> BitContainerType {
    bitContainer & *BIT_mask.as_ptr().offset(nbBits as isize) as BitContainerType
}

#[inline]
pub(crate) unsafe fn BIT_addBits(
    bitC: *mut BIT_CStream_t,
    value: BitContainerType,
    nbBits: core::ffi::c_uint,
) {
    (*bitC).bitContainer |= BIT_getLowerBits(value, nbBits) << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}

#[inline]
unsafe fn BIT_addBitsFast(
    bitC: *mut BIT_CStream_t,
    value: BitContainerType,
    nbBits: core::ffi::c_uint,
) {
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}

#[inline]
pub(crate) unsafe fn BIT_flushBits(bitC: *mut BIT_CStream_t) {
    let nbBytes = ((*bitC).bitPos >> 3) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut core::ffi::c_void, (*bitC).bitContainer);
    (*bitC).ptr = ((*bitC).ptr).add(nbBytes);
    if (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
    (*bitC).bitPos &= 7;
    (*bitC).bitContainer >>= nbBytes * 8;
}

#[inline]
pub(crate) unsafe fn BIT_flushBitsFast(bitC: *mut BIT_CStream_t) {
    let nbBytes = ((*bitC).bitPos >> 3) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut core::ffi::c_void, (*bitC).bitContainer);
    (*bitC).ptr = ((*bitC).ptr).add(nbBytes);
    (*bitC).bitPos &= 7;
    (*bitC).bitContainer >>= nbBytes * 8;
}

#[inline]
pub(crate) unsafe fn BIT_closeCStream(bitC: *mut BIT_CStream_t) -> size_t {
    BIT_addBitsFast(bitC, 1, 1);
    BIT_flushBits(bitC);
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0;
    }
    (((*bitC).ptr).offset_from((*bitC).startPtr) as usize)
        .wrapping_add(usize::from((*bitC).bitPos > 0))
}

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
pub(crate) struct BIT_DStream_t<'a> {
    pub(crate) bitContainer: usize,
    pub(crate) bitsConsumed: core::ffi::c_uint,
    pub(crate) ptr: *const core::ffi::c_char,
    pub(crate) start: *const core::ffi::c_char,
    pub(crate) limitPtr: *const core::ffi::c_char,
    pub(crate) _marker: PhantomData<&'a [u8]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StreamStatus {
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
    const REG_MASK: u32 = BitContainerType::BITS - 1;

    // if start > regMask, the bitstream is corrupted, and the result is undefined.
    debug_assert!(nbBits < MASK.len() as u32);

    if cfg!(target_arch = "x86_64") {
        // x86 transform & ((1 << nbBits) - 1) to bzhi instruction, it is better
        // than accessing memory. When bmi2 instruction is not present, we consider
        // such cpus old (pre-Haswell, 2013) and their performance is not of that
        // importance.
        bitContainer >> (start & REG_MASK) & ((1usize << nbBits) - 1)
    } else {
        bitContainer >> (start & REG_MASK) & MASK[nbBits as usize] as BitContainerType
    }
}

impl<'a> BIT_DStream_t<'a> {
    pub(crate) fn new(srcBuffer: &'a [u8]) -> Result<Self, Error> {
        let mut bitD = Self {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: core::ptr::null::<core::ffi::c_char>(),
            start: core::ptr::null::<core::ffi::c_char>(),
            limitPtr: core::ptr::null::<core::ffi::c_char>(),
            _marker: PhantomData,
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
            bitD.bitContainer = usize::from_le_bytes(*chunk);

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
            BitContainerType::BITS
                .wrapping_sub(self.bitsConsumed)
                .wrapping_sub(nbBits),
            nbBits,
        )
    }

    /// Like [`look_bits`], but only works when `nbBits >= 1`
    #[inline]
    pub(crate) const fn look_bits_fast(&self, nbBits: u32) -> BitContainerType {
        // quickcheck hits this
        // debug_assert!(nbBits > 1);

        const MASK: u32 = (BitContainerType::BITS as usize - 1) as u32;
        self.bitContainer << (self.bitsConsumed & MASK)
            >> (MASK.wrapping_add(1).wrapping_sub(nbBits) & MASK)
    }

    #[inline(always)]
    pub(crate) const fn skip_bits(&mut self, nbBits: u32) {
        self.bitsConsumed += nbBits;
    }

    /// Read (consume) next n bits from local register and update.
    ///
    /// Pay attention to not read more than nbBits contained into local register.
    #[inline(always)]
    pub(crate) const fn read_bits(&mut self, nbBits: core::ffi::c_uint) -> BitContainerType {
        let value = self.look_bits(nbBits);
        self.skip_bits(nbBits);
        value
    }

    /// Like [`read_bits`], but only works when `nbBits >= 1`
    #[inline]
    pub(crate) const fn read_bits_fast(&mut self, nbBits: core::ffi::c_uint) -> BitContainerType {
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
    pub(crate) fn reload_fast(&mut self) -> StreamStatus {
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
    pub(crate) fn reload(&mut self) -> StreamStatus {
        if self.bitsConsumed > BitContainerType::BITS {
            static zeroFilled: BitContainerType = 0;
            self.ptr = &zeroFilled as *const BitContainerType as *const core::ffi::c_char;

            return StreamStatus::Overflow;
        }

        if self.ptr >= self.limitPtr {
            unsafe { self.reload_internal() };

            return StreamStatus::Unfinished;
        }

        if self.ptr == self.start {
            return if self.bitsConsumed < BitContainerType::BITS {
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

    pub(crate) fn is_empty(&self) -> bool {
        self.ptr == self.start && self.bitsConsumed == BitContainerType::BITS
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
            bitsConsumed: BitContainerType::BITS + 1,
            ptr: core::ptr::null(),
            start: core::ptr::null(),
            limitPtr: core::ptr::null(),
            _marker: PhantomData,
        };

        assert_eq!(stream.reload(), StreamStatus::Overflow);
    }
}

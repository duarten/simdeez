use crate::{
    libm_ext::FloatExt, InternalSimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat, SimdFloat32,
    SimdFloat64, SimdInt, SimdInt16, SimdInt32, SimdInt64, SimdInt8,
};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::ops::*;

mod simd;
pub use self::simd::*;

define_simd_type!(i8, 16, __m128i, _41);
impl_simd_int_overloads!(I8x16_41);

impl InternalSimdBaseIo for I8x16_41 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I8x16_41(_mm_setzero_si128())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I8x16_41(_mm_set1_epi8(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I8x16_41(_mm_loadu_si128(array.as_ptr() as *const __m128i))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I8x16_41(_mm_loadu_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm_storeu_si128(ptr as *mut __m128i, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I8x16_41(_mm_load_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm_store_si128(ptr as *mut __m128i, self.0);
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I8x16_41(value)
    }
}

impl SimdBaseOps for I8x16_41 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_add_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_sub_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        // There is no multiply operation for i8, although the compiler may automatically
        // find optimizations for this.
        let mut array = [0i8; 16];
        for i in 0..16 {
            array[i] = self[i].wrapping_mul(rhs[i]);
        }
        unsafe { I8x16_41::load_from_array(array) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_and_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_or_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_xor_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { Self::set1(-1).bit_xor(self) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe {
            let mask = self.cmp_lt(Self::zeroes());
            self.bit_xor(mask) - mask
        }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_andnot_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe { I8x16_41(_mm_blendv_epi8(a.0, b.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_cmpeq_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_cmplt_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_cmpgt_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_max_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { I8x16_41(_mm_min_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_add()
            .partial_horizontal_add()
            .partial_horizontal_add()
            .partial_horizontal_add()
    }
}

impl SimdInt for I8x16_41 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm_sll_epi16(self.0, rhs2);

            let mask = 0x00FFu16 >> (8 - rhs) << 8;
            let mask = _mm_set1_epi16(mask as i16);
            I8x16_41(_mm_andnot_si128(mask, shifted_i16))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm_srl_epi16(self.0, rhs2);

            let mask = 0xFF00u16 << (8 - rhs) >> 8;
            let mask = _mm_set1_epi16(mask as i16);
            I8x16_41(_mm_andnot_si128(mask, shifted_i16))
        }
    }

    #[inline(always)]
    fn horizontal_unsigned_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_unsigned_add()
            .partial_horizontal_unsigned_add()
            .partial_horizontal_unsigned_add()
            .partial_horizontal_add()
    }
}

impl SimdInt8 for I8x16_41 {
    type SimdI16 = I16x8_41;

    #[inline(always)]
    fn extend_to_i16(self) -> (Self::SimdI16, Self::SimdI16) {
        unsafe {
            let lo = _mm_cvtepi8_epi16(self.0);
            let hi = _mm_cvtepi8_epi16(_mm_shuffle_epi32(self.0, 0b_01_00_11_10));
            (I16x8_41(lo), I16x8_41(hi))
        }
    }

    #[inline(always)]
    fn unsigned_extend_to_i16(self) -> (Self::SimdI16, Self::SimdI16) {
        unsafe {
            let lo = _mm_unpacklo_epi8(self.0, _mm_setzero_si128());
            let hi = _mm_unpackhi_epi8(self.0, _mm_setzero_si128());
            (I16x8_41(lo), I16x8_41(hi))
        }
    }

    #[inline(always)]
    fn get_mask(self) -> u32 {
        unsafe { _mm_movemask_epi8(self.0) as u32 }
    }
}

define_simd_type!(i16, 8, __m128i, _41);
impl_simd_int_overloads!(I16x8_41);

impl SimdBaseOps for I16x8_41 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_add_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_sub_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_mullo_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_and_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_or_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_xor_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { Self::set1(-1).bit_xor(self) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe {
            let mask = self.cmp_lt(Self::zeroes());
            self.bit_xor(mask) - mask
        }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_andnot_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe {
            let mask = self.cmp_eq(Self::set1(-1));
            let a = mask.and_not(a);
            let b = mask.bit_and(b);
            a.bit_or(b)
        }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_cmpeq_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_cmplt_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_cmpgt_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_max_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_min_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_add()
            .partial_horizontal_add()
            .partial_horizontal_add()
    }
}

impl InternalSimdBaseIo for I16x8_41 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I16x8_41(_mm_setzero_si128())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I16x8_41(_mm_set1_epi16(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I16x8_41(_mm_loadu_si128(array.as_ptr() as *const __m128i))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I16x8_41(_mm_loadu_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm_storeu_si128(ptr as *mut __m128i, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I16x8_41(_mm_load_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm_store_si128(ptr as *mut __m128i, self.0);
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I16x8_41(value)
    }
}

impl SimdInt for I16x8_41 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I16x8_41(_mm_sll_epi16(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I16x8_41(_mm_srl_epi16(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I16x8_41(_mm_slli_epi16(self.0, BY)) }
    }

    #[inline(always)]
    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I16x8_41(_mm_srli_epi16(self.0, BY)) }
    }

    #[inline(always)]
    fn horizontal_unsigned_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_unsigned_add()
            .partial_horizontal_unsigned_add()
            .horizontal_unsigned_add()
    }
}

impl SimdInt16 for I16x8_41 {
    type SimdI32 = I32x4_41;

    #[inline(always)]
    fn extend_to_i32(self) -> (Self::SimdI32, Self::SimdI32) {
        unsafe {
            let lo = _mm_cvtepi16_epi32(self.0);
            let hi = _mm_cvtepi16_epi32(_mm_shuffle_epi32(self.0, 0b_01_00_11_10));
            (I32x4_41(lo), I32x4_41(hi))
        }
    }

    #[inline(always)]
    fn unsigned_extend_to_i32(self) -> (Self::SimdI32, Self::SimdI32) {
        unsafe {
            let lo = _mm_unpacklo_epi16(self.0, _mm_setzero_si128());
            let hi = _mm_unpackhi_epi16(self.0, _mm_setzero_si128());
            (I32x4_41(lo), I32x4_41(hi))
        }
    }
}

define_simd_type!(i32, 4, __m128i, _41);
impl_simd_int_overloads!(I32x4_41);

impl SimdBaseOps for I32x4_41 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_add_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_sub_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_mullo_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_and_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_or_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_xor_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { Self::set1(-1).bit_xor(self) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe {
            let mask = self.cmp_lt(Self::zeroes());
            self.bit_xor(mask) - mask
        }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_andnot_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        self.bitcast_f32()
            .blendv(a.bitcast_f32(), b.bitcast_f32())
            .bitcast_i32()
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_cmpeq_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_cmplt_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_cmpgt_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_max_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_min_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_add().partial_horizontal_add()
    }
}

impl InternalSimdBaseIo for I32x4_41 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I32x4_41(_mm_setzero_si128())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I32x4_41(_mm_set1_epi32(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I32x4_41(_mm_loadu_si128(array.as_ptr() as *const __m128i))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I32x4_41(_mm_loadu_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm_storeu_si128(ptr as *mut __m128i, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I32x4_41(_mm_load_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm_store_si128(ptr as *mut __m128i, self.0);
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I32x4_41(value)
    }
}

impl SimdInt for I32x4_41 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I32x4_41(_mm_sll_epi32(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I32x4_41(_mm_srl_epi32(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I32x4_41(_mm_slli_epi32(self.0, BY)) }
    }

    #[inline(always)]
    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I32x4_41(_mm_srli_epi32(self.0, BY)) }
    }

    #[inline(always)]
    fn horizontal_unsigned_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_unsigned_add()
            .horizontal_unsigned_add()
    }
}

impl SimdInt32 for I32x4_41 {
    type SimdF32 = F32x4_41;
    type SimdI64 = I64x2_41;

    #[inline(always)]
    fn bitcast_f32(self) -> Self::SimdF32 {
        unsafe { F32x4_41(_mm_castsi128_ps(self.0)) }
    }

    #[inline(always)]
    fn cast_f32(self) -> Self::SimdF32 {
        unsafe { F32x4_41(_mm_cvtepi32_ps(self.0)) }
    }

    #[inline(always)]
    fn extend_to_i64(self) -> (Self::SimdI64, Self::SimdI64) {
        unsafe {
            let lo = _mm_cvtepi32_epi64(self.0);
            let hi = _mm_cvtepi32_epi64(_mm_shuffle_epi32(self.0, 0b_01_00_11_10));
            (I64x2_41(lo), I64x2_41(hi))
        }
    }

    #[inline(always)]
    fn unsigned_extend_to_i64(self) -> (Self::SimdI64, Self::SimdI64) {
        unsafe {
            // Doing it manually as the alternative has a lot more operations
            (
                I64x2_41::load_from_array([
                    self[0] as u32 as u64 as i64,
                    self[1] as u32 as u64 as i64,
                ]),
                I64x2_41::load_from_array([
                    self[2] as u32 as u64 as i64,
                    self[3] as u32 as u64 as i64,
                ]),
            )
        }
    }
}

define_simd_type!(i64, 2, __m128i, _41);
impl_simd_int_overloads!(I64x2_41);

impl SimdBaseOps for I64x2_41 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_add_epi64(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_sub_epi64(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe {
            Self::load_from_array([self[0].wrapping_mul(rhs[0]), self[1].wrapping_mul(rhs[1])])
        }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_and_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_or_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_xor_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { Self::set1(-1).bit_xor(self) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe {
            let mask = self.cmp_lt(Self::zeroes());
            self.bit_xor(mask) - mask
        }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_andnot_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        self.bitcast_f64()
            .blendv(a.bitcast_f64(), b.bitcast_f64())
            .bitcast_i64()
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_cmpeq_epi64(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe {
            Self::load_from_array([
                if self[0] < rhs[0] { -1 } else { 0 },
                if self[1] < rhs[1] { -1 } else { 0 },
            ])
        }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        unsafe {
            Self::load_from_array([
                if self[0] <= rhs[0] { -1 } else { 0 },
                if self[1] <= rhs[1] { -1 } else { 0 },
            ])
        }
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe {
            Self::load_from_array([
                if self[0] > rhs[0] { -1 } else { 0 },
                if self[1] > rhs[1] { -1 } else { 0 },
            ])
        }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        unsafe {
            Self::load_from_array([
                if self[0] >= rhs[0] { -1 } else { 0 },
                if self[1] >= rhs[1] { -1 } else { 0 },
            ])
        }
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        let cmp = self.cmp_gt(rhs);
        cmp.blendv(rhs, self)
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        let cmp = self.cmp_lt(rhs);
        cmp.blendv(rhs, self)
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_add()
    }
}

impl InternalSimdBaseIo for I64x2_41 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I64x2_41(_mm_setzero_si128())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I64x2_41(_mm_set1_epi64x(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I64x2_41(_mm_loadu_si128(array.as_ptr() as *const _))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I64x2_41(_mm_loadu_si128(ptr as *const _))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm_storeu_si128(ptr as *mut _, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I64x2_41(_mm_load_si128(ptr as *const _))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm_store_si128(ptr as *mut _, self.0);
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I64x2_41(value)
    }
}

impl SimdInt for I64x2_41 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I64x2_41(_mm_sll_epi64(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I64x2_41(_mm_srl_epi64(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I64x2_41(_mm_slli_epi64(self.0, BY)) }
    }

    #[inline(always)]
    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I64x2_41(_mm_srli_epi64(self.0, BY)) }
    }

    #[inline(always)]
    fn horizontal_unsigned_add(self) -> Self::HorizontalAddScalar {
        // We don't do any casting at i64, so just calling this fn is fine
        self.partial_horizontal_add()
    }
}

impl SimdInt64 for I64x2_41 {
    type SimdF64 = F64x2_41;

    #[inline(always)]
    fn bitcast_f64(self) -> Self::SimdF64 {
        unsafe { F64x2_41(_mm_castsi128_pd(self.0)) }
    }

    #[inline(always)]
    fn cast_f64(self) -> Self::SimdF64 {
        unsafe { Self::SimdF64::load_from_array([self[0] as f64, self[1] as f64]) }
    }

    fn partial_horizontal_add(self) -> i64 {
        self[0].wrapping_add(self[1])
    }
}

define_simd_type!(f32, 4, __m128, _41);
impl_simd_float_overloads!(F32x4_41);

impl SimdBaseOps for F32x4_41 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_add_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_sub_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_mul_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_and_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_or_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_xor_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { I32x4_41::set1(-1).bitcast_f32().bit_xor(self) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe { Self::set1(-0.0).and_not(self) }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_andnot_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe { F32x4_41(_mm_blendv_ps(a.0, b.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmpeq_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmpneq_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmplt_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmple_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmpgt_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmpge_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_max_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_min_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::Scalar {
        unsafe {
            let mut tmp = _mm_hadd_ps(self.0, self.0);
            tmp = _mm_hadd_ps(tmp, tmp);
            _mm_cvtss_f32(tmp)
        }
    }
}

impl InternalSimdBaseIo for F32x4_41 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        F32x4_41(_mm_setzero_ps())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        F32x4_41(_mm_set1_ps(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F32x4_41(_mm_loadu_ps(array.as_ptr()))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        F32x4_41(_mm_loadu_ps(ptr as *const f32))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm_storeu_ps(ptr as *mut f32, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        F32x4_41(_mm_load_ps(ptr as *const f32))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm_store_ps(ptr as *mut f32, self.0);
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        F32x4_41(value)
    }
}

impl SimdFloat for F32x4_41 {
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_div_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        unsafe { F32x4_41(_mm_ceil_ps(self.0)) }
    }

    #[inline(always)]
    fn floor(self) -> Self {
        unsafe { F32x4_41(_mm_floor_ps(self.0)) }
    }

    #[inline(always)]
    fn round(self) -> Self {
        unsafe {
            F32x4_41(_mm_round_ps(
                self.0,
                _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
            ))
        }
    }

    #[inline(always)]
    fn fast_ceil(self) -> Self {
        self.ceil()
    }

    #[inline(always)]
    fn fast_floor(self) -> Self {
        self.floor()
    }

    #[inline(always)]
    fn fast_round(self) -> Self {
        self.round()
    }

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }

    #[inline(always)]
    fn mul_sub(self, a: Self, b: Self) -> Self {
        self * a - b
    }

    #[inline(always)]
    fn neg_mul_add(self, a: Self, b: Self) -> Self {
        -self * a + b
    }

    #[inline(always)]
    fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        -self * a - b
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        unsafe { F32x4_41(_mm_sqrt_ps(self.0)) }
    }

    #[inline(always)]
    fn rsqrt(self) -> Self {
        unsafe { F32x4_41(_mm_rsqrt_ps(self.0)) }
    }
}

impl SimdFloat32 for F32x4_41 {
    type SimdI32 = I32x4_41;

    #[inline(always)]
    fn bitcast_i32(self) -> Self::SimdI32 {
        unsafe { I32x4_41(_mm_castps_si128(self.0)) }
    }

    #[inline(always)]
    fn cast_i32(self) -> Self::SimdI32 {
        unsafe { I32x4_41(_mm_cvtps_epi32(self.0)) }
    }

    #[inline(always)]
    fn fast_inverse(self) -> Self {
        unsafe { F32x4_41(_mm_rcp_ps(self.0)) }
    }
}

define_simd_type!(f64, 2, __m128d, _41);
impl_simd_float_overloads!(F64x2_41);

impl SimdBaseOps for F64x2_41 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_add_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_sub_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_mul_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_and_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_or_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_xor_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { I64x2_41::set1(-1).bitcast_f64().bit_xor(self) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe { Self::set1(-0.0).and_not(self) }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_andnot_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe { F64x2_41(_mm_blendv_pd(a.0, b.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmpeq_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmpneq_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmplt_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmple_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmpgt_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmpge_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_max_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_min_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::Scalar {
        unsafe {
            let arr = self.transmute_into_array_ref();
            arr[0] + arr[1]
        }
    }
}

impl InternalSimdBaseIo for F64x2_41 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        F64x2_41(_mm_setzero_pd())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        F64x2_41(_mm_set1_pd(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F64x2_41(_mm_loadu_pd(array.as_ptr()))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        F64x2_41(_mm_loadu_pd(ptr))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm_storeu_pd(ptr, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        F64x2_41(_mm_load_pd(ptr))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm_store_pd(ptr, self.0);
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        F64x2_41(value)
    }
}

impl SimdFloat for F64x2_41 {
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_div_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        unsafe { F64x2_41(_mm_ceil_pd(self.0)) }
    }

    #[inline(always)]
    fn floor(self) -> Self {
        unsafe { F64x2_41(_mm_floor_pd(self.0)) }
    }

    #[inline(always)]
    fn round(self) -> Self {
        unsafe {
            F64x2_41(_mm_round_pd(
                self.0,
                _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
            ))
        }
    }

    #[inline(always)]
    fn fast_ceil(self) -> Self {
        self.ceil()
    }

    #[inline(always)]
    fn fast_floor(self) -> Self {
        self.floor()
    }

    #[inline(always)]
    fn fast_round(self) -> Self {
        self.round()
    }

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }

    #[inline(always)]
    fn mul_sub(self, a: Self, b: Self) -> Self {
        self * a - b
    }

    #[inline(always)]
    fn neg_mul_add(self, a: Self, b: Self) -> Self {
        -self * a + b
    }

    #[inline(always)]
    fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        -self * a - b
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        unsafe { F64x2_41(_mm_sqrt_pd(self.0)) }
    }

    #[inline(always)]
    fn rsqrt(self) -> Self {
        unsafe { Self::set1(1.0) / self.sqrt() }
    }
}

impl SimdFloat64 for F64x2_41 {
    type SimdI64 = I64x2_41;

    #[inline(always)]
    fn bitcast_i64(self) -> Self::SimdI64 {
        unsafe { I64x2_41(_mm_castpd_si128(self.0)) }
    }

    #[inline(always)]
    fn cast_i64(self) -> Self::SimdI64 {
        unsafe {
            Self::SimdI64::load_from_array([self[0].m_round() as i64, self[1].m_round() as i64])
        }
    }
}

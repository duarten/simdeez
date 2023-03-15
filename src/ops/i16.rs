use super::*;

impl_op! {
    fn add<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_add_epi16(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_add_epi16(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_add_epi16(a, b)
        }
        for Scalar(a: i16, b: i16) -> i16 {
            a.wrapping_add(b)
        }
    }
}

impl_op! {
    fn sub<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_sub_epi16(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_sub_epi16(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_sub_epi16(a, b)
        }
        for Scalar(a: i16, b: i16) -> i16 {
            a.wrapping_sub(b)
        }
    }
}

impl_op! {
    fn mul<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_mullo_epi16(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_mullo_epi16(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_mullo_epi16(a, b)
        }
        for Scalar(a: i16, b: i16) -> i16 {
            a.wrapping_mul(b)
        }
    }
}

impl_op! {
    fn min<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_min_epi16(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_min_epi16(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_min_epi16(a, b)
        }
        for Scalar(a: i16, b: i16) -> i16 {
            a.min(b)
        }
    }
}

impl_op! {
    fn max<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_max_epi16(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_max_epi16(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_max_epi16(a, b)
        }
        for Scalar(a: i16, b: i16) -> i16 {
            a.max(b)
        }
    }
}

impl_op! {
    fn abs<i16> {
        for Avx2(a: __m256i) -> __m256i {
            _mm256_abs_epi16(a)
        }
        for Sse41(a: __m128i) -> __m128i {
            _mm_abs_epi16(a)
        }
        for Sse2(a: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi16(_mm_setzero_si128(), a);
            _mm_sub_epi16(_mm_xor_si128(a, mask), mask)
        }
        for Scalar(a: i16) -> i16 {
            a.abs()
        }
    }
}

impl_op! {
    fn eq<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_cmpeq_epi16(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpeq_epi16(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpeq_epi16(a, b)
        }
        for Scalar(a: i16, b: i16) -> i16 {
            if a == b {
                u32::MAX as i16
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn neq<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let eq = _mm256_cmpeq_epi16(a, b);
            _mm256_xor_si256(eq, _mm256_set1_epi16(u32::MAX as i16))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let eq = _mm_cmpeq_epi16(a, b);
            _mm_xor_si128(eq, _mm_set1_epi16(u32::MAX as i16))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let eq = _mm_cmpeq_epi16(a, b);
            _mm_xor_si128(eq, _mm_set1_epi16(u32::MAX as i16))
        }
        for Scalar(a: i16, b: i16) -> i16 {
            if a != b {
                u32::MAX as i16
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn lt<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi16(a, b);
            let eq = _mm256_cmpeq_epi16(a, b);
            _mm256_andnot_si256(_mm256_or_si256(gt, eq), _mm256_set1_epi16(u32::MAX as i16))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi16(a, b);
            let eq = _mm_cmpeq_epi16(a, b);
            _mm_andnot_si128(_mm_or_si128(gt, eq), _mm_set1_epi16(u32::MAX as i16))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi16(a, b);
            let eq = _mm_cmpeq_epi16(a, b);
            _mm_andnot_si128(_mm_or_si128(gt, eq), _mm_set1_epi16(u32::MAX as i16))
        }
        for Scalar(a: i16, b: i16) -> i16 {
            if a < b {
                u32::MAX as i16
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn lte<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi16(a, b);
            _mm256_xor_si256(gt, _mm256_set1_epi16(u32::MAX as i16))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi16(a, b);
            _mm_xor_si128(gt, _mm_set1_epi16(u32::MAX as i16))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi16(a, b);
            _mm_xor_si128(gt, _mm_set1_epi16(u32::MAX as i16))
        }
        for Scalar(a: i16, b: i16) -> i16 {
            if a <= b {
                u32::MAX as i16
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn gt<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_cmpgt_epi16(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpgt_epi16(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpgt_epi16(a, b)
        }
        for Scalar(a: i16, b: i16) -> i16 {
            if a > b {
                u32::MAX as i16
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn gte<i16> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi16(a, b);
            let eq = _mm256_cmpeq_epi16(a, b);
            _mm256_or_si256(gt, eq)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi16(a, b);
            let eq = _mm_cmpeq_epi16(a, b);
            _mm_or_si128(gt, eq)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi16(a, b);
            let eq = _mm_cmpeq_epi16(a, b);
            _mm_or_si128(gt, eq)
        }
        for Scalar(a: i16, b: i16) -> i16 {
            if a >= b {
                u32::MAX as i16
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn blendv<i16> {
        for Avx2(a: __m256i, b: __m256i, mask: __m256i) -> __m256i {
            _mm256_blendv_epi8(a, b, mask)
        }
        for Sse41(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
            _mm_blendv_epi8(a, b, mask)
        }
        for Sse2(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
            _mm_or_si128(_mm_and_si128(mask, b), _mm_andnot_si128(mask, a))
        }
        for Scalar(a: i16, b: i16, mask: i16) -> i16 {
            if mask == 0 {
                a
            } else {
                b
            }
        }
    }
}

impl_op! {
    fn shl<i16> {
        for Avx2(a: __m256i, rhs: i32) -> __m256i {
            _mm256_sll_epi16(a, _mm_cvtsi32_si128(rhs))
        }
        for Sse41(a: __m128i, rhs: i32) -> __m128i {
            _mm_sll_epi16(a, _mm_cvtsi32_si128(rhs))
        }
        for Sse2(a: __m128i, rhs: i32) -> __m128i {
            _mm_sll_epi16(a, _mm_cvtsi32_si128(rhs))
        }
        for Scalar(a: i16, rhs: i32) -> i16 {
            a << rhs
        }
    }
}

impl_op! {
    fn shr<i16> {
        for Avx2(a: __m256i, rhs: i32) -> __m256i {
            _mm256_srl_epi16(a, _mm_cvtsi32_si128(rhs))
        }
        for Sse41(a: __m128i, rhs: i32) -> __m128i {
            _mm_srl_epi16(a, _mm_cvtsi32_si128(rhs))
        }
        for Sse2(a: __m128i, rhs: i32) -> __m128i {
            _mm_srl_epi16(a, _mm_cvtsi32_si128(rhs))
        }
        for Scalar(a: i16, rhs: i32) -> i16 {
            ((a as u16) >> rhs) as i16
        }
    }
}

impl_imm8_op! {
    fn shl_const<i16, const BY: i32> {
        for Avx2(a: __m256i) -> __m256i {
            _mm256_slli_epi16(a, BY)
        }
        for Sse41(a: __m128i) -> __m128i {
            _mm_slli_epi16(a, BY)
        }
        for Sse2(a: __m128i) -> __m128i {
            _mm_slli_epi16(a, BY)
        }
        for Scalar(a: i16) -> i16 {
            a << BY
        }
    }
}

impl_imm8_op! {
    fn shr_const<i16, const BY: i32> {
        for Avx2(a: __m256i) -> __m256i {
            _mm256_srli_epi16(a, BY)
        }
        for Sse41(a: __m128i) -> __m128i {
            _mm_srli_epi16(a, BY)
        }
        for Sse2(a: __m128i) -> __m128i {
            _mm_srli_epi16(a, BY)
        }
        for Scalar(a: i16) -> i16 {
            ((a as u16) >> BY) as i16
        }
    }
}

impl_op! {
    fn extend_i32<i16> {
        for Avx2(val: __m256i) -> (__m256i, __m256i) {
            let a = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(val, 0));
            let b = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(val, 1));
            (a, b)
        }
        for Sse41(val: __m128i) -> (__m128i, __m128i) {
            let a = _mm_cvtepi16_epi32(val);
            let b = _mm_cvtepi16_epi32(_mm_shuffle_epi32(val, 0b_01_00_11_10));
            (a, b)
        }
        for Sse2(val: __m128i) -> (__m128i, __m128i) {
            let arr = core::mem::transmute::<__m128i, [i16; 8]>(val);
            let a = [
                arr[0] as i32,
                arr[1] as i32,
                arr[2] as i32,
                arr[3] as i32,
            ];
            let b = [
                arr[4] as i32,
                arr[5] as i32,
                arr[6] as i32,
                arr[7] as i32,
            ];
            (core::mem::transmute(a), core::mem::transmute(b))
        }
        for Scalar(val: i16) -> (i32, i32) {
            (val as i32, 0)
        }
    }
}

impl_op! {
    fn unsigned_extend_i32<i16> {
        for Avx2(val: __m256i) -> (__m256i, __m256i) {
            let a = _mm256_cvtepu16_epi32(_mm256_extracti128_si256(val, 0));
            let b = _mm256_cvtepu16_epi32(_mm256_extracti128_si256(val, 1));
            (a, b)
        }
        for Sse41(val: __m128i) -> (__m128i, __m128i) {
            let a = _mm_cvtepu16_epi32(val);
            let b = _mm_cvtepu16_epi32(_mm_shuffle_epi32(val, 0b_01_00_11_10));
            (a, b)
        }
        for Sse2(val: __m128i) -> (__m128i, __m128i) {
            let arr = core::mem::transmute::<__m128i, [i16; 8]>(val);
            let a = [
                arr[0] as u16 as u32 as i32,
                arr[1] as u16 as u32 as i32,
                arr[2] as u16 as u32 as i32,
                arr[3] as u16 as u32 as i32,
            ];
            let b = [
                arr[4] as u16 as u32 as i32,
                arr[5] as u16 as u32 as i32,
                arr[6] as u16 as u32 as i32,
                arr[7] as u16 as u32 as i32,
            ];
            (core::mem::transmute(a), core::mem::transmute(b))
        }
        for Scalar(val: i16) -> (i32, i32) {
            (val as u16 as u32 as i32, 0)
        }
    }
}

impl_op! {
    fn zeroes<i16> {
        for Avx2() -> __m256i {
            _mm256_setzero_si256()
        }
        for Sse41() -> __m128i {
            _mm_setzero_si128()
        }
        for Sse2() -> __m128i {
            _mm_setzero_si128()
        }
        for Scalar() -> i16 {
            0
        }
    }
}

impl_op! {
    fn set1<i16> {
        for Avx2(val: i16) -> __m256i {
            _mm256_set1_epi16(val)
        }
        for Sse41(val: i16) -> __m128i {
            _mm_set1_epi16(val)
        }
        for Sse2(val: i16) -> __m128i {
            _mm_set1_epi16(val)
        }
        for Scalar(val: i16) -> i16 {
            val
        }
    }
}

impl_op! {
    fn load_unaligned<i16> {
        for Avx2(ptr: *const i16) -> __m256i {
            _mm256_loadu_si256(ptr as *const __m256i)
        }
        for Sse41(ptr: *const i16) -> __m128i {
            _mm_loadu_si128(ptr as *const __m128i)
        }
        for Sse2(ptr: *const i16) -> __m128i {
            _mm_loadu_si128(ptr as *const __m128i)
        }
        for Scalar(ptr: *const i16) -> i16 {
            unsafe { *ptr }
        }
    }
}

impl_op! {
    fn load_aligned<i16> {
        for Avx2(ptr: *const i16) -> __m256i {
            _mm256_load_si256(ptr as *const __m256i)
        }
        for Sse41(ptr: *const i16) -> __m128i {
            _mm_load_si128(ptr as *const __m128i)
        }
        for Sse2(ptr: *const i16) -> __m128i {
            _mm_load_si128(ptr as *const __m128i)
        }
        for Scalar(ptr: *const i16) -> i16 {
            unsafe { *ptr }
        }
    }
}

impl_op! {
    fn store_unaligned<i16> {
        for Avx2(ptr: *mut i16, a: __m256i) {
            _mm256_storeu_si256(ptr as *mut __m256i, a)
        }
        for Sse41(ptr: *mut i16, a: __m128i) {
            _mm_storeu_si128(ptr as *mut __m128i, a)
        }
        for Sse2(ptr: *mut i16, a: __m128i) {
            _mm_storeu_si128(ptr as *mut __m128i, a)
        }
        for Scalar(ptr: *mut i16, a: i16) {
            unsafe { *ptr = a }
        }
    }
}

impl_op! {
    fn store_aligned<i16> {
        for Avx2(ptr: *mut i16, a: __m256i) {
            _mm256_store_si256(ptr as *mut __m256i, a)
        }
        for Sse41(ptr: *mut i16, a: __m128i) {
            _mm_store_si128(ptr as *mut __m128i, a)
        }
        for Sse2(ptr: *mut i16, a: __m128i) {
            _mm_store_si128(ptr as *mut __m128i, a)
        }
        for Scalar(ptr: *mut i16, a: i16) {
            unsafe { *ptr = a }
        }
    }
}

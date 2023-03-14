use super::*;

impl_op! {
    fn bitcast_binary<f32> {
        for Avx2(a: __m256) -> __m256 {
            a
        }
        for Sse41(a: __m128) -> __m128 {
            a
        }
        for Sse2(a: __m128) -> __m128 {
            a
        }
        for Scalar(a: f32) -> u64 {
            a.to_bits() as u64
        }
    }
}

impl_op! {
    fn bitcast_f32<binary> {
        for Avx2(a: __m256) -> __m256 {
            a
        }
        for Sse41(a: __m128) -> __m128 {
            a
        }
        for Sse2(a: __m128) -> __m128 {
            a
        }
        for Scalar(a: u64) -> f32 {
            f32::from_bits(a as u32)
        }
    }
}

impl_op! {
    fn bitcast_binary<f64> {
        for Avx2(a: __m256d) -> __m256 {
            _mm256_castpd_ps(a)
        }
        for Sse41(a: __m128d) -> __m128 {
            _mm_castpd_ps(a)
        }
        for Sse2(a: __m128d) -> __m128 {
            _mm_castpd_ps(a)
        }
        for Scalar(a: f64) -> u64 {
            a.to_bits()
        }
    }
}

impl_op! {
    fn bitcast_f64<binary> {
        for Avx2(a: __m256) -> __m256d {
            _mm256_castps_pd(a)
        }
        for Sse41(a: __m128) -> __m128d {
            _mm_castps_pd(a)
        }
        for Sse2(a: __m128) -> __m128d {
            _mm_castps_pd(a)
        }
        for Scalar(a: u64) -> f64 {
            f64::from_bits(a)
        }
    }
}

impl_op! {
    fn bitcast_binary<i32> {
        for Avx2(a: __m256i) -> __m256 {
            _mm256_castsi256_ps(a)
        }
        for Sse41(a: __m128i) -> __m128 {
            _mm_castsi128_ps(a)
        }
        for Sse2(a: __m128i) -> __m128 {
            _mm_castsi128_ps(a)
        }
        for Scalar(a: i32) -> u64 {
            a as u64
        }
    }
}

impl_op! {
    fn bitcast_i32<binary> {
        for Avx2(a: __m256) -> __m256i {
            _mm256_castps_si256(a)
        }
        for Sse41(a: __m128) -> __m128i {
            _mm_castps_si128(a)
        }
        for Sse2(a: __m128) -> __m128i {
            _mm_castps_si128(a)
        }
        for Scalar(a: u64) -> i32 {
            a as i32
        }
    }
}

impl_op! {
    fn bitcast_binary<i64> {
        for Avx2(a: __m256i) -> __m256 {
            _mm256_castsi256_ps(a)
        }
        for Sse41(a: __m128i) -> __m128 {
            _mm_castsi128_ps(a)
        }
        for Sse2(a: __m128i) -> __m128 {
            _mm_castsi128_ps(a)
        }
        for Scalar(a: i64) -> u64 {
            a as u64
        }
    }
}

impl_op! {
    fn bitcast_i64<binary> {
        for Avx2(a: __m256) -> __m256i {
            _mm256_castps_si256(a)
        }
        for Sse41(a: __m128) -> __m128i {
            _mm_castps_si128(a)
        }
        for Sse2(a: __m128) -> __m128i {
            _mm_castps_si128(a)
        }
        for Scalar(a: u64) -> i64 {
            a as i64
        }
    }
}

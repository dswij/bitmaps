// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::{Debug, Error, Formatter};

use typenum::*;

use crate::types::Bits;

/// A compact array of bits.
///
/// The type used to store the bitmap will be the minimum unsigned integer type
/// required to fit the number of bits, from `u8` to `u128`. If the size is 1,
/// `bool` is used. If the size exceeds 128, an array of `u128` will be used,
/// sized as appropriately. The maximum supported size is currently 1024,
/// represented by an array `[u128; 8]`.
pub struct Bitmap<Size: Bits> {
    pub(crate) data: Size::Store,
}

impl<Size: Bits> Clone for Bitmap<Size> {
    fn clone(&self) -> Self {
        Bitmap { data: self.data }
    }
}

impl<Size: Bits> Copy for Bitmap<Size> {}

impl<Size: Bits> Default for Bitmap<Size> {
    fn default() -> Self {
        Bitmap {
            data: Size::Store::default(),
        }
    }
}

impl<Size: Bits> PartialEq for Bitmap<Size> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<Size: Bits> Debug for Bitmap<Size> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.data.fmt(f)
    }
}

impl<Size: Bits> Bitmap<Size> {
    /// Construct a bitmap with every bit set to `false`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a bitmap from a value of the same type as its backing store.
    #[inline]
    pub fn from_value(data: Size::Store) -> Self {
        Self { data }
    }

    /// Convert this bitmap into a value of the type of its backing store.
    #[inline]
    pub fn into_value(self) -> Size::Store {
        self.data
    }

    /// Count the number of `true` bits in the bitmap.
    #[inline]
    pub fn len(self) -> usize {
        Size::len(&self.data)
    }

    /// Test if the bitmap contains only `false` bits.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.first_index().is_none()
    }

    /// Get the value of the bit at a given index.
    #[inline]
    pub fn get(self, index: usize) -> bool {
        Size::get(&self.data, index)
    }

    /// Set the value of the bit at a given index.
    ///
    /// Returns the previous value of the bit.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) -> bool {
        Size::set(&mut self.data, index, value)
    }

    /// Find the index of the first `true` bit in the bitmap.
    #[inline]
    pub fn first_index(self) -> Option<usize> {
        Size::first_index(&self.data)
    }
}

impl<Size: Bits> IntoIterator for Bitmap<Size> {
    type Item = usize;
    type IntoIter = Iter<Size>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            index: 0,
            data: self.data,
        }
    }
}

impl From<[u128; 2]> for Bitmap<U256> {
    fn from(data: [u128; 2]) -> Self {
        Bitmap { data }
    }
}

impl From<[u128; 3]> for Bitmap<U384> {
    fn from(data: [u128; 3]) -> Self {
        Bitmap { data }
    }
}

impl From<[u128; 4]> for Bitmap<U512> {
    fn from(data: [u128; 4]) -> Self {
        Bitmap { data }
    }
}

impl From<[u128; 5]> for Bitmap<U640> {
    fn from(data: [u128; 5]) -> Self {
        Bitmap { data }
    }
}

impl From<[u128; 6]> for Bitmap<U768> {
    fn from(data: [u128; 6]) -> Self {
        Bitmap { data }
    }
}

impl From<[u128; 7]> for Bitmap<U896> {
    fn from(data: [u128; 7]) -> Self {
        Bitmap { data }
    }
}

impl From<[u128; 8]> for Bitmap<U1024> {
    fn from(data: [u128; 8]) -> Self {
        Bitmap { data }
    }
}

impl Into<[u128; 2]> for Bitmap<U256> {
    fn into(self) -> [u128; 2] {
        self.data
    }
}

impl Into<[u128; 3]> for Bitmap<U384> {
    fn into(self) -> [u128; 3] {
        self.data
    }
}

impl Into<[u128; 4]> for Bitmap<U512> {
    fn into(self) -> [u128; 4] {
        self.data
    }
}

impl Into<[u128; 5]> for Bitmap<U640> {
    fn into(self) -> [u128; 5] {
        self.data
    }
}

impl Into<[u128; 6]> for Bitmap<U768> {
    fn into(self) -> [u128; 6] {
        self.data
    }
}

impl Into<[u128; 7]> for Bitmap<U896> {
    fn into(self) -> [u128; 7] {
        self.data
    }
}

impl Into<[u128; 8]> for Bitmap<U1024> {
    fn into(self) -> [u128; 8] {
        self.data
    }
}

/// An iterator over the indices in a bitmap which are `true`.
///
/// This yields a sequence of `usize` indices, not their contents (which are
/// always `true` anyway, by definition).
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate bitmaps;
/// # extern crate typenum;
/// # use bitmaps::Bitmap;
/// # use typenum::U10;
/// # fn main() {
/// let mut bitmap: Bitmap<U10> = Bitmap::new();
/// bitmap.set(3, true);
/// bitmap.set(5, true);
/// bitmap.set(8, true);
/// let true_indices: Vec<usize> = bitmap.into_iter().collect();
/// assert_eq!(vec![3, 5, 8], true_indices);
/// # }
/// ```
pub struct Iter<Size: Bits> {
    index: usize,
    data: Size::Store,
}

impl<Size: Bits> Iterator for Iter<Size> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= Size::USIZE {
            return None;
        }
        if Size::get(&self.data, self.index) {
            self.index += 1;
            Some(self.index - 1)
        } else {
            self.index += 1;
            self.next()
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_arch {
    use super::*;
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    impl Bitmap<U128> {
        #[target_feature(enable = "sse2")]
        pub unsafe fn load_m128i(&self) -> __m128i {
            _mm_loadu_si128(&self.data as *const _ as *const __m128i)
        }
    }

    impl Bitmap<U256> {
        #[target_feature(enable = "sse2")]
        pub unsafe fn load_m128i(&self) -> [__m128i; 2] {
            let ptr = &self.data as *const _ as *const __m128i;
            [_mm_loadu_si128(ptr), _mm_loadu_si128(ptr.add(1))]
        }

        #[target_feature(enable = "avx")]
        pub unsafe fn load_m256i(&self) -> __m256i {
            _mm256_loadu_si256(&self.data as *const _ as *const __m256i)
        }
    }

    impl Bitmap<U512> {
        #[target_feature(enable = "sse2")]
        pub unsafe fn load_m128i(&self) -> [__m128i; 4] {
            let ptr = &self.data as *const _ as *const __m128i;
            [
                _mm_loadu_si128(ptr),
                _mm_loadu_si128(ptr.add(1)),
                _mm_loadu_si128(ptr.add(2)),
                _mm_loadu_si128(ptr.add(3)),
            ]
        }

        #[target_feature(enable = "avx")]
        pub unsafe fn load_m256i(&self) -> [__m256i; 2] {
            let ptr = &self.data as *const _ as *const __m256i;
            [_mm256_loadu_si256(ptr), _mm256_loadu_si256(ptr.add(1))]
        }
    }

    impl Bitmap<U768> {
        #[target_feature(enable = "sse2")]
        pub unsafe fn load_m128i(&self) -> [__m128i; 6] {
            let ptr = &self.data as *const _ as *const __m128i;
            [
                _mm_loadu_si128(ptr),
                _mm_loadu_si128(ptr.add(1)),
                _mm_loadu_si128(ptr.add(2)),
                _mm_loadu_si128(ptr.add(3)),
                _mm_loadu_si128(ptr.add(4)),
                _mm_loadu_si128(ptr.add(5)),
            ]
        }

        #[target_feature(enable = "avx")]
        pub unsafe fn load_m256i(&self) -> [__m256i; 3] {
            let ptr = &self.data as *const _ as *const __m256i;
            [
                _mm256_loadu_si256(ptr),
                _mm256_loadu_si256(ptr.add(1)),
                _mm256_loadu_si256(ptr.add(2)),
            ]
        }
    }

    impl Bitmap<U1024> {
        #[target_feature(enable = "sse2")]
        pub unsafe fn load_m128i(&self) -> [__m128i; 8] {
            let ptr = &self.data as *const _ as *const __m128i;
            [
                _mm_loadu_si128(ptr),
                _mm_loadu_si128(ptr.add(1)),
                _mm_loadu_si128(ptr.add(2)),
                _mm_loadu_si128(ptr.add(3)),
                _mm_loadu_si128(ptr.add(4)),
                _mm_loadu_si128(ptr.add(5)),
                _mm_loadu_si128(ptr.add(6)),
                _mm_loadu_si128(ptr.add(7)),
            ]
        }

        #[target_feature(enable = "avx")]
        pub unsafe fn load_m256i(&self) -> [__m256i; 4] {
            let ptr = &self.data as *const _ as *const __m256i;
            [
                _mm256_loadu_si256(ptr),
                _mm256_loadu_si256(ptr.add(1)),
                _mm256_loadu_si256(ptr.add(2)),
                _mm256_loadu_si256(ptr.add(3)),
            ]
        }
    }

    impl From<__m128i> for Bitmap<U128> {
        fn from(data: __m128i) -> Self {
            Self {
                data: unsafe { std::mem::transmute(data) },
            }
        }
    }

    impl From<__m256i> for Bitmap<U256> {
        fn from(data: __m256i) -> Self {
            Self {
                data: unsafe { std::mem::transmute(data) },
            }
        }
    }

    impl Into<__m128i> for Bitmap<U128> {
        fn into(self) -> __m128i {
            unsafe { self.load_m128i() }
        }
    }

    impl Into<__m256i> for Bitmap<U256> {
        fn into(self) -> __m256i {
            unsafe { self.load_m256i() }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        struct AlignmentTester<B>
        where
            B: Bits,
        {
            _byte: u8,
            bits: Bitmap<B>,
        }

        #[test]
        fn load_128() {
            let mut t: AlignmentTester<U128> = AlignmentTester {
                _byte: 0,
                bits: Bitmap::new(),
            };
            t.bits.set(5, true);
            let m = unsafe { t.bits.load_m128i() };
            let mut bits: Bitmap<U128> = m.into();
            assert!(bits.set(5, false));
            assert!(bits.is_empty());
        }

        #[test]
        fn load_256() {
            let mut t: AlignmentTester<U256> = AlignmentTester {
                _byte: 0,
                bits: Bitmap::new(),
            };
            t.bits.set(5, true);
            let m = unsafe { t.bits.load_m256i() };
            let mut bits: Bitmap<U256> = m.into();
            assert!(bits.set(5, false));
            assert!(bits.is_empty());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::collection::btree_set;
    use proptest::proptest;
    use typenum::{U1024, U64};

    proptest! {
        #[test]
        fn get_set_and_iter_64(bits in btree_set(0..64usize, 0..64)) {
            let mut bitmap = Bitmap::<U64>::new();
            for i in &bits {
                bitmap.set(*i, true);
            }
            for i in 0..64 {
                assert_eq!(bitmap.get(i), bits.contains(&i));
            }
            assert!(bitmap.into_iter().eq(bits.into_iter()));
        }

        #[test]
        fn get_set_and_iter_1024(bits in btree_set(0..1024usize, 0..1024)) {
            let mut bitmap = Bitmap::<U1024>::new();
            for i in &bits {
                bitmap.set(*i, true);
            }
            for i in 0..1024 {
                assert_eq!(bitmap.get(i), bits.contains(&i));
            }
            assert!(bitmap.into_iter().eq(bits.into_iter()));
        }
    }
}

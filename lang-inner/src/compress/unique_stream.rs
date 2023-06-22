//! # Unique stream compression
//!
//! Provides the [`UStream`] struct that compresses tables based on choosing unique elements for
//! each stream element.

#![deny(missing_docs)]

use crate::compress::ser::{DeserializeBin, SerializeBin};
use crate::compress::{Compress, Zero};
use crate::Table;
use std::ops::Index;
#[cfg(test)]
use rand::Rng;

/// Transition table compression resulting in a single stream, row offsets, and an element map
///
/// ---
/// Original
/// ```text
/// 0 2 1 0
/// 0 1 0 1
/// 0 3 0 0
/// 2 1 0 0
/// ```
/// Ordered by offset (one non-zero element per offset column)
/// ```text
///   0 2 1 0
///       0 1 0 1
///         0 3 0 0
/// 2 1 0 0
/// ```
/// Merged rows with element map and row offsets
/// ```text
/// 2 1 2 1 1 3 1 0
/// 3 3 0 0 1 2 1 0
/// 1 3 4 0
/// ```
#[derive(Debug)]
pub struct UStream<D, S, OR, OF>
where
    D: Copy + Clone + Sized + Zero + PartialEq,
    S: Index<usize, Output = D> + Sized,
    OR: Index<usize, Output = u16> + Sized,
    OF: Index<usize, Output = usize> + Sized,
{
    /// Stream representation of the original table
    pub stream: S,
    /// Stream element origins. This tells us what row the matching element in
    /// [`stream`](Self::stream) came from
    pub origin: OR,
    /// Table row offsets. These give the offsets of the rows in the [`stream`](Self::stream)
    pub offsets: OF
}

impl<D: Copy + Clone + Sized + Zero + PartialEq + std::fmt::Debug + std::fmt::Display> Compress<D>
    for UStream<D, Vec<D>, Vec<u16>, Vec<usize>>
{
    /// Compresses a transition table using the comb method. This will likely produce a decent
    /// compression, with the optimal compression being possible by performing this on all
    /// permutations or the original set
    fn compress(l: &Vec<[D; 256]>) -> Self {
        let mut stream = Vec::<D>::new();
        let mut origin = Vec::<u16>::new();
        let mut offsets = Vec::<usize>::new();
        for (row_index, row) in l.iter().enumerate() {
            let mut offset = 0;
            'main: while offset < stream.len() {
                // the range ensures we only check elements that can fit inside the stream.
                // this means we don't need to check this each iteration
                // the filter makes sure we're only looking at non-zero elements
                for (p, _) in row[..256.min(stream.len() - offset)].iter().enumerate().filter(|(_, &e)| e != *D::ZERO) {
                    // if elem is zero, it can fit. if it's non-zero, the stream value must be zero
                    // for it to fit. If it's non-zero in the stream, we start again after
                    // incrementing the offset
                    if stream[offset + p] != *D::ZERO {
                        offset += 1;
                        continue 'main;
                    }
                }
                break;
            }
            offsets.push(offset);
            let (inner, new) = row.split_at(256.min(stream.len() - offset));
            for (p, &r) in inner.iter().enumerate().filter(|(_, r)| **r != *D::ZERO) {
                stream[offset + p] = r;
                origin[offset + p] = row_index as u16;
            }
            for &r in new.iter() {
                stream.push(r);
                origin.push(if r == *D::ZERO { 0 } else { row_index as u16 });
            }
        }
        Self {
            stream,
            offsets,
            origin
        }
    }

    #[cfg(feature = "macro")]
    fn optimal(l: &Vec<[D; 256]>) -> Self {
        let mut empty = Vec::new();
        let mut single = Vec::new();
        let mut non_single = Vec::new();
        for (i, n) in l.iter().enumerate() {
            match n.iter().filter(|n| *n != D::ZERO).count() {
                0 => empty.push((i, n)),
                1 => single.push((i, n)),
                _ => non_single.push((i, n)),
            }
        }
        let mut l = l.iter().enumerate().collect::<Vec<_>>();
        l.sort_by_key(|(_, a)| a.iter().filter(|n| *n == D::ZERO).count());
        let mut stream = Vec::new();
        let mut origin = Vec::new();
        let mut offsets = Vec::new();
        for (row_index, row) in l.iter() {
            let mut offset = 0;
            'main: while offset < stream.len() {
                for (p, elem) in row.iter().enumerate() {
                    if offset + p == stream.len() {
                        break 'main;
                    }
                    if elem != D::ZERO && &stream[offset + p] != D::ZERO {
                        offset += 1;
                        continue 'main;
                    }
                }
                break;
            }
            offsets.push(offset);
            for (p, r) in row[..256.min(stream.len() - offset)].iter().enumerate() {
                let stream_previous = stream.get(offset + p).unwrap_or(D::ZERO);
                let origin_previous = origin.get(offset + p).unwrap_or(&0);
                if r == D::ZERO {
                    stream[offset + p] = *stream_previous;
                    origin[offset + p] = *origin_previous;
                } else {
                    stream[offset + p] = *r;
                    origin[offset + p] = *row_index as u16 + 1;
                }
            }
            for r in row[256.min(stream.len() - offset)..].iter() {
                stream.push(r.clone());
                origin.push(if r == D::ZERO {
                    0
                } else {
                    *row_index as u16 + 1
                });
            }
        }
        Self {
            stream,
            offsets,
            origin,
        }
    }
}

#[test]
fn check_ustream_compress() {
    let mut table = Vec::new();
    let n = 500;
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        table.push([0; 256].map(|_| if rng.gen_range(0..10) == 0 { rng.gen::<u8>() } else { 0 }));
    }
    let ustream = UStream::compress(&table);
    for i in 0..n {
        for n in 0..=255 {
            assert_eq!(table[i][n], ustream.element(i as u16, n as u8), "Compressed table not equal")
        }
    }
}

impl<D, S, OR, OF> Table<D> for UStream<D, S, OR, OF>
where
    D: Copy + Clone + Sized + Zero + PartialEq,
    S: Index<usize, Output = D> + Sized,
    OR: Index<usize, Output = u16> + Sized,
    OF: Index<usize, Output = usize> + Sized,
{
	#[inline]
    fn element(&self, row: u16, col: u8) -> D {
        let index = self.offsets[row as usize] + col as usize;
        if self.origin[index] == row { self.stream[index] } else { *D::ZERO }
    }
}

impl<D> SerializeBin for UStream<D, Vec<D>, Vec<u16>, Vec<usize>>
where
    D: Copy + Clone + Sized + Zero + PartialEq + SerializeBin,
{
    fn serialize(&self, out: &mut Vec<u8>) {
        self.stream.serialize(out);
        self.origin.serialize(out);
        self.offsets.serialize(out);
    }
}

impl<T, D> DeserializeBin<T> for UStream<D, Vec<D>, Vec<u16>, Vec<usize>>
where
    T: Iterator<Item = u8>,
    D: Copy + Clone + Sized + Zero + PartialEq + DeserializeBin<T>,
{
    fn deserialize(iter: &mut T) -> Option<Self> {
        Some(Self {
            stream: DeserializeBin::deserialize(iter)?,
            origin: DeserializeBin::deserialize(iter)?,
            offsets: DeserializeBin::deserialize(iter)?,
        })
    }
}

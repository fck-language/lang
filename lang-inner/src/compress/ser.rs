use crate::compress::{Zero, UStream};
use crate::Table;

/// # Serialization for transition maps
///
/// This trait provides the [`serialize`](SerializeBin::serialize) function to encode a transition
/// map as a bit stream
pub trait SerializeBin {
    /// Serialize `&self` into a bit stream.
    ///
    /// Each serialization uses a unique identifier as the first bit to indicate the compression
    /// method. This is indicated in the implementation documentation.
    ///
    /// See specific implementation documentation for how each type is serialized.
    fn serialize(&self, out: &mut Vec<u8>);
}

impl<T: SerializeBin> SerializeBin for &T {
    fn serialize(&self, out: &mut Vec<u8>) {
        T::serialize(self, out)
    }
}

/// # Deserialization for transition maps
///
/// This trait provides the [`deserialize`](DeserializeBin::deserialize) function to decode a bit
/// stream into a `Self`
pub(crate) trait DeserializeBin<T: Iterator<Item = u8>>
where
    Self: Sized,
{
    /// try to deserialize a bit stream into a `Self` instance. See implementations of
    /// [`SerializeBin`] for details on how each type is converted into a bit stream
    fn deserialize(iter: &mut T) -> Option<Self>;
}

impl<T: SerializeBin> SerializeBin for [T; 256] {
    fn serialize(&self, out: &mut Vec<u8>) {
        for row in self.iter() {
            row.serialize(out)
        }
    }
}

impl<T: SerializeBin> SerializeBin for Vec<T> {
    fn serialize(&self, out: &mut Vec<u8>) {
        self.len().serialize(out);
        for elem in self.iter() {
            elem.serialize(out)
        }
    }
}

impl SerializeBin for u8 {
    fn serialize(&self, out: &mut Vec<u8>) {
        out.push(*self)
    }
}

impl SerializeBin for u16 {
    fn serialize(&self, out: &mut Vec<u8>) {
        out.push((self >> 8) as u8);
        out.push((self & 0xff) as u8)
    }
}

impl SerializeBin for u32 {
    fn serialize(&self, out: &mut Vec<u8>) {
        out.push((self >> 24) as u8);
        out.push((self >> 16) as u8);
        out.push((self >> 8) as u8);
        out.push((self & 0xff) as u8)
    }
}

impl SerializeBin for usize {
    fn serialize(&self, out: &mut Vec<u8>) {
        (*self as u32).serialize(out)
    }
}

impl<T: Iterator<Item = u8>> DeserializeBin<T> for u8 {
    fn deserialize(iter: &mut T) -> Option<Self> {
        iter.next()
    }
}

impl<T: Iterator<Item = u8>> DeserializeBin<T> for u16 {
    fn deserialize(iter: &mut T) -> Option<Self> {
        Some(((iter.next()? as u16) << 8) + iter.next()? as u16)
    }
}

impl<T: Iterator<Item = u8>> DeserializeBin<T> for u32 {
    fn deserialize(iter: &mut T) -> Option<Self> {
        Some(
            ((iter.next()? as u32) << 24)
                + ((iter.next()? as u32) << 16)
                + ((iter.next()? as u32) << 8)
                + iter.next()? as u32,
        )
    }
}

impl<T: Iterator<Item = u8>> DeserializeBin<T> for usize {
    fn deserialize(iter: &mut T) -> Option<Self> {
        <u32 as DeserializeBin<T>>::deserialize(iter).map(|t| t as usize)
    }
}

impl<T: Iterator<Item = u8>, I: DeserializeBin<T> + Zero + Copy> DeserializeBin<T> for [I; 256] {
    fn deserialize(iter: &mut T) -> Option<Self> {
        let mut out = [*I::ZERO; 256];
        for i in 0..256 {
            out[i] = I::deserialize(iter)?
        }
        Some(out)
    }
}

impl<T: Iterator<Item = u8>, I: DeserializeBin<T>> DeserializeBin<T> for Vec<I> {
    fn deserialize(iter: &mut T) -> Option<Self> {
        let len = usize::deserialize(iter)?;
        let mut out = Vec::new();
        for _ in 0..len {
            out.push(I::deserialize(iter)?)
        }
        Some(out)
    }
}

pub fn serialize_bin<T: Iterator<Item = u8>>(
    iter: &mut T,
) -> Option<(UStream<u16, Vec<u16>, Vec<u16>, Vec<usize>>, UStream<u8, Vec<u8>, Vec<u16>, Vec<usize>>, UStream<u8, Vec<u8>, Vec<u16>, Vec<usize>>)> {
    Some((
        <UStream<u16, Vec<u16>, Vec<u16>, Vec<usize>> as DeserializeBin<T>>::deserialize(iter)?,
        <UStream<u8, Vec<u8>, Vec<u16>, Vec<usize>> as DeserializeBin<T>>::deserialize(iter)?,
        <UStream<u8, Vec<u8>, Vec<u16>, Vec<usize>> as DeserializeBin<T>>::deserialize(iter)?
    ))
}

pub fn deserialize_bin<T, M, N>((transition, tt, td): (T, M, N)) -> Vec<u8>
where
    T: Table<u16> + SerializeBin,
    M: Table<u8> + SerializeBin,
    N: Table<u8> + SerializeBin,
{
    let mut out = Vec::new();
    transition.serialize(&mut out);
    tt.serialize(&mut out);
    td.serialize(&mut out);
    out
}

#[test]
fn test() {
    let n = vec![[0u8; 256], [4u8; 256], [1u8; 256]];
	let mut out = Vec::new();
	n.serialize(&mut out);
    let mut out = out.iter().cloned();
    let mut ser_de_n: Option<Vec<[u8; 256]>> = DeserializeBin::deserialize(&mut out);
	assert_eq!(Some(n), ser_de_n);
    assert!(out.next().is_none())
}

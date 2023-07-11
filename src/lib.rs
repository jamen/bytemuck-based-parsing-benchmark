#![feature(slice_take)]

use std::mem::size_of;

use bytemuck::{AnyBitPattern, PodCastError};

pub fn ref_option_parse<T: AnyBitPattern>(source: &mut &[u8]) -> Option<T> {
    let bytes = source.take(..size_of::<T>())?;
    bytemuck::try_pod_read_unaligned(bytes).ok()
}

pub fn ref_result_parse<T: AnyBitPattern>(source: &mut &[u8]) -> Result<T, PodCastError> {
    use PodCastError::SizeMismatch;
    let bytes = source.take(..size_of::<T>()).ok_or(SizeMismatch)?;
    bytemuck::try_pod_read_unaligned(bytes)
}

pub fn nomlike_min_result_parse<T: AnyBitPattern>(mut source: &[u8]) -> Result<(&[u8], T), &[u8]> {
    let bytes = source.take(..size_of::<T>()).ok_or(source)?;
    let val = bytemuck::try_pod_read_unaligned(bytes).map_err(|_| source)?;
    Ok((source, val))
}

pub fn nomlike_option_parse<T: AnyBitPattern>(mut source: &[u8]) -> Option<(T, &[u8])> {
    let bytes = source.take(..size_of::<T>())?;
    let val = bytemuck::try_pod_read_unaligned(bytes).ok()?;
    Some((val, source))
}

type ParseResult<'a, O, E, I = &'a [u8]> = Result<(I, O), (I, E)>;

pub fn nomlike_full_result_parse<T: AnyBitPattern>(
    mut source: &[u8],
) -> ParseResult<T, PodCastError> {
    use PodCastError::SizeMismatch;
    let bytes = source
        .take(..size_of::<T>())
        .ok_or_else(|| (source, SizeMismatch))?;
    let val = bytemuck::try_pod_read_unaligned(bytes).map_err(|error| (source, error))?;
    Ok((source, val))
}

use bytemuck_based_parsing_benchmark::{
    nomlike_full_result_parse, nomlike_min_result_parse, nomlike_option_parse, ref_option_parse,
    ref_result_parse,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::RngCore;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut bytes = vec![0u8; u16::MAX as usize];

    rand::thread_rng().fill_bytes(&mut bytes);

    let bytes = &bytes[..];

    c.bench_function("ref_option_parse", |b| {
        b.iter(|| {
            let mut bytes = &mut black_box(bytes);
            loop {
                match ref_option_parse::<u32>(&mut bytes) {
                    Some(val) => {
                        black_box(val.to_le());
                    }
                    None => break,
                }
            }
        })
    });

    c.bench_function("ref_result_parse", |b| {
        b.iter(|| {
            let mut bytes = &mut black_box(bytes);
            loop {
                match ref_result_parse::<u32>(&mut bytes) {
                    Ok(val) => {
                        black_box(val.to_le());
                    }
                    Err(_) => break,
                }
            }
        })
    });

    c.bench_function("nomlike_min_result_parse", |b| {
        b.iter(|| {
            let mut bytes = black_box(bytes);
            loop {
                match nomlike_min_result_parse::<u32>(&mut bytes) {
                    Ok((next_bytes, val)) => {
                        black_box(val.to_le());
                        bytes = next_bytes;
                    }
                    Err(_) => break,
                }
            }
        })
    });

    c.bench_function("nomlike_full_result_parse", |b| {
        b.iter(|| {
            let mut bytes = black_box(bytes);
            loop {
                match nomlike_full_result_parse::<u32>(&mut bytes) {
                    Ok((next_bytes, val)) => {
                        black_box(val.to_le());
                        bytes = next_bytes;
                    }
                    Err(_) => break,
                }
            }
        })
    });

    c.bench_function("nomlike_option_parse", |b| {
        b.iter(|| {
            let mut bytes = black_box(bytes);
            loop {
                match nomlike_option_parse::<u32>(&mut bytes) {
                    Some((val, next_bytes)) => {
                        black_box(val.to_le());
                        bytes = next_bytes;
                    }
                    None => break,
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

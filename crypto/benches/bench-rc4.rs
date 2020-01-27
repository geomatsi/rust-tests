use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use crypto::rc4;
use stream_cipher::generic_array::typenum::{UTerm, U24, U32, U5, U8};
use stream_cipher::generic_array::GenericArray;
use stream_cipher::NewStreamCipher;
use stream_cipher::StreamCipher;

const KEY_N1: [u8; 5] = [0x01, 0x02, 0x03, 0x04, 0x05];

fn benchmark_40_bit_key(c: &mut Criterion) {
    let mut rc4 = rc4::RC4::<U5, UTerm>::new(
        &GenericArray::from_slice(&KEY_N1),
        &GenericArray::from_slice(&[]),
    );

    let mut buf: [u8; 4112] = [0x0; 4112];

    c.bench_function("key: 40 bits", |b| b.iter(|| rc4.encrypt(&mut buf)));
}

const KEY_N2: [u8; 8] = [0x64, 0x19, 0x10, 0x83, 0x32, 0x22, 0x77, 0x2a];

fn benchmark_64_bit_key(c: &mut Criterion) {
    let mut rc4 = rc4::RC4::<U8, UTerm>::new(
        &GenericArray::from_slice(&KEY_N2),
        &GenericArray::from_slice(&[]),
    );

    let mut buf: [u8; 4112] = [0x0; 4112];

    c.bench_function("key: 64 bits", |b| b.iter(|| rc4.encrypt(&mut buf)));
}

const KEY_N3: [u8; 24] = [
    0xc1, 0x09, 0x16, 0x39, 0x08, 0xeb, 0xe5, 0x1d, 0xeb, 0xb4, 0x62, 0x27, 0xc6, 0xcc, 0x8b, 0x37,
    0x64, 0x19, 0x10, 0x83, 0x32, 0x22, 0x77, 0x2a,
];

fn benchmark_192_bit_key(c: &mut Criterion) {
    let mut rc4 = rc4::RC4::<U24, UTerm>::new(
        &GenericArray::from_slice(&KEY_N3),
        &GenericArray::from_slice(&[]),
    );

    let mut buf: [u8; 4112] = [0x0; 4112];

    c.bench_function("key: 192 bits", |b| b.iter(|| rc4.encrypt(&mut buf)));
}

const KEY_N4: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
];

fn benchmark_256_bit_key(c: &mut Criterion) {
    let mut rc4 = rc4::RC4::<U32, UTerm>::new(
        &GenericArray::from_slice(&KEY_N4),
        &GenericArray::from_slice(&[]),
    );

    let mut buf: [u8; 4112] = [0x0; 4112];

    c.bench_function("key: 256 bits", |b| b.iter(|| rc4.encrypt(&mut buf)));
}

criterion_group!(
    benches,
    benchmark_40_bit_key,
    benchmark_64_bit_key,
    benchmark_192_bit_key,
    benchmark_256_bit_key
);

criterion_main!(benches);

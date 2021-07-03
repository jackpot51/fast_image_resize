use criterion::{criterion_group, criterion_main, Criterion};

use fast_image_resize::{CpuExtensions, ImageData, MulDiv, PixelType};
use std::num::NonZeroU32;

const fn p(r: u8, g: u8, b: u8, a: u8) -> u32 {
    u32::from_le_bytes([r, g, b, a])
}

// Multiplies by alpha

fn get_src_image(width: NonZeroU32, height: NonZeroU32, pixel: u32) -> ImageData<Vec<u8>> {
    let rgba: [u8; 4] = pixel.to_le_bytes();
    let buf_size = (width.get() * height.get()) as usize * 4;
    let mut buffer = vec![0u8; buf_size];
    buffer.chunks_exact_mut(4).for_each(|c| {
        c[0] = rgba[0];
        c[1] = rgba[1];
        c[2] = rgba[2];
        c[3] = rgba[3];
    });
    ImageData::new(width, height, buffer, PixelType::U8x4).unwrap()
}

fn multiplies_alpha_avx2(c: &mut Criterion) {
    let width = NonZeroU32::new(4096).unwrap();
    let height = NonZeroU32::new(2048).unwrap();
    let src_data = get_src_image(width, height, p(255, 128, 0, 128));
    let mut dst_data = ImageData::new_owned(width, height, PixelType::U8x4);
    let src_view = src_data.src_view();
    let mut dst_view = dst_data.dst_view();
    let mut alpha_mul_div: MulDiv = Default::default();
    unsafe {
        alpha_mul_div.set_cpu_extensions(CpuExtensions::Avx2);
    }

    c.bench_function("Multiplies alpha AVX2", |b| {
        b.iter(|| {
            alpha_mul_div
                .multiply_alpha(&src_view, &mut dst_view)
                .unwrap();
        })
    });
}

fn multiplies_alpha_sse2(c: &mut Criterion) {
    let width = NonZeroU32::new(4096).unwrap();
    let height = NonZeroU32::new(2048).unwrap();
    let src_data = get_src_image(width, height, p(255, 128, 0, 128));
    let mut dst_data = ImageData::new_owned(width, height, PixelType::U8x4);
    let src_view = src_data.src_view();
    let mut dst_view = dst_data.dst_view();
    let mut alpha_mul_div: MulDiv = Default::default();
    unsafe {
        alpha_mul_div.set_cpu_extensions(CpuExtensions::Sse2);
    }

    c.bench_function("Multiplies alpha SSE2", |b| {
        b.iter(|| {
            alpha_mul_div
                .multiply_alpha(&src_view, &mut dst_view)
                .unwrap();
        })
    });
}

fn multiplies_alpha_native(c: &mut Criterion) {
    let width = NonZeroU32::new(4096).unwrap();
    let height = NonZeroU32::new(2048).unwrap();
    let src_data = get_src_image(width, height, p(255, 128, 0, 128));
    let mut dst_data = ImageData::new_owned(width, height, PixelType::U8x4);
    let src_view = src_data.src_view();
    let mut dst_view = dst_data.dst_view();
    let mut alpha_mul_div: MulDiv = Default::default();
    unsafe {
        alpha_mul_div.set_cpu_extensions(CpuExtensions::None);
    }

    c.bench_function("Multiplies alpha native", |b| {
        b.iter(|| {
            alpha_mul_div
                .multiply_alpha(&src_view, &mut dst_view)
                .unwrap();
        })
    });
}

fn divides_alpha_avx2(c: &mut Criterion) {
    let width = NonZeroU32::new(4096).unwrap();
    let height = NonZeroU32::new(2048).unwrap();
    let src_data = get_src_image(width, height, p(128, 64, 0, 128));
    let mut dst_data = ImageData::new_owned(width, height, PixelType::U8x4);
    let src_view = src_data.src_view();
    let mut dst_view = dst_data.dst_view();
    let mut alpha_mul_div: MulDiv = Default::default();
    unsafe {
        alpha_mul_div.set_cpu_extensions(CpuExtensions::Avx2);
    }

    c.bench_function("Divides alpha AVX2", |b| {
        b.iter(|| {
            alpha_mul_div
                .divide_alpha(&src_view, &mut dst_view)
                .unwrap();
        })
    });
}

fn divides_alpha_sse2(c: &mut Criterion) {
    let width = NonZeroU32::new(4096).unwrap();
    let height = NonZeroU32::new(2048).unwrap();
    let src_data = get_src_image(width, height, p(128, 64, 0, 128));
    let mut dst_data = ImageData::new_owned(width, height, PixelType::U8x4);
    let src_view = src_data.src_view();
    let mut dst_view = dst_data.dst_view();
    let mut alpha_mul_div: MulDiv = Default::default();
    unsafe {
        alpha_mul_div.set_cpu_extensions(CpuExtensions::Sse2);
    }

    c.bench_function("Divides alpha SSE2", |b| {
        b.iter(|| {
            alpha_mul_div
                .divide_alpha(&src_view, &mut dst_view)
                .unwrap();
        })
    });
}

fn divides_alpha_native(c: &mut Criterion) {
    let width = NonZeroU32::new(4096).unwrap();
    let height = NonZeroU32::new(2048).unwrap();
    let src_data = get_src_image(width, height, p(128, 64, 0, 128));
    let mut dst_data = ImageData::new_owned(width, height, PixelType::U8x4);
    let src_view = src_data.src_view();
    let mut dst_view = dst_data.dst_view();
    let mut alpha_mul_div: MulDiv = Default::default();
    unsafe {
        alpha_mul_div.set_cpu_extensions(CpuExtensions::None);
    }

    c.bench_function("Divides alpha native", |b| {
        b.iter(|| {
            alpha_mul_div
                .divide_alpha(&src_view, &mut dst_view)
                .unwrap();
        })
    });
}

criterion_group!(
    benches,
    multiplies_alpha_avx2,
    multiplies_alpha_sse2,
    multiplies_alpha_native,
    divides_alpha_avx2,
    divides_alpha_sse2,
    divides_alpha_native,
);
criterion_main!(benches);

use crate::convolution::{optimisations, Coefficients};
use crate::image_view::{TypedImageView, TypedImageViewMut};
use crate::pixels::U16;

#[inline(always)]
pub(crate) fn horiz_convolution(
    src_image: TypedImageView<U16>,
    mut dst_image: TypedImageViewMut<U16>,
    offset: u32,
    coeffs: Coefficients,
) {
    let (values, window_size, bounds) = (coeffs.values, coeffs.window_size, coeffs.bounds);

    let normalizer_guard = optimisations::NormalizerGuard32::new(values);
    let precision = normalizer_guard.precision();
    let coefficients_chunks = normalizer_guard.normalized_chunks(window_size, &bounds);
    let initial: i64 = 1 << (precision - 1);

    let src_rows = src_image.iter_rows(offset);
    let dst_rows = dst_image.iter_rows_mut();
    for (dst_row, src_row) in dst_rows.zip(src_rows) {
        for (&coeffs_chunk, dst_pixel) in coefficients_chunks.iter().zip(dst_row.iter_mut()) {
            let first_x_src = coeffs_chunk.start as usize;
            let mut sum = initial;
            let src_pixels = unsafe { src_row.get_unchecked(first_x_src..) };
            for (&k, src_pixel) in coeffs_chunk.values.iter().zip(src_pixels) {
                sum += src_pixel.0 as i64 * (k as i64);
            }
            dst_pixel.0 = normalizer_guard.clip(sum);
        }
    }
}

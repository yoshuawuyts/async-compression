//! This module contains Brotli-specific types for async-compression.

use brotli::enc::backward_references::{BrotliEncoderMode, BrotliEncoderParams};

/// A compression parameter for Brotli. This is a stable wrapper around Brotli's own encoder params
/// type, to abstract over different versions of the Brotli library.
///
/// See the [Brotli documentation](https://www.brotli.org/encode.html#a9a8) for more information on
/// these parameters.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncoderParams {
    window_size: i32,
    block_size: i32,
    size_hint: usize,
    mode: BrotliEncoderMode,
}

impl EncoderParams {
    /// Sets window size in bytes (as a power of two).
    ///
    /// Used as Brotli's `lgwin` parameter.
    ///
    /// `window_size` is clamped to `0 <= window_size <= 24`.
    pub fn window_size(mut self, window_size: i32) -> Self {
        self.window_size = window_size.clamp(0, 24);
        self
    }

    /// Sets input block size in bytes (as a power of two).
    ///
    /// Used as Brotli's `lgblock` parameter.
    ///
    /// `block_size` is clamped to `16 <= block_size <= 24`.
    pub fn block_size(mut self, block_size: i32) -> Self {
        self.block_size = block_size.clamp(16, 24);
        self
    }

    /// Sets hint for size of data to be compressed.
    pub fn size_hint(mut self, size_hint: usize) -> Self {
        self.size_hint = size_hint;
        self
    }

    /// Sets encoder to text mode.
    ///
    /// If input data is known to be UTF-8 text, this allows the compressor to make assumptions and
    /// optimizations.
    ///
    /// Used as Brotli's `mode` parameter.
    pub fn text_mode(mut self) -> Self {
        self.mode = BrotliEncoderMode::BROTLI_MODE_TEXT;
        self
    }

    pub(crate) fn as_brotli(&self) -> BrotliEncoderParams {
        BrotliEncoderParams {
            lgwin: self.window_size,
            lgblock: self.block_size,
            size_hint: self.size_hint,
            mode: self.mode,
            ..Default::default()
        }
    }
}

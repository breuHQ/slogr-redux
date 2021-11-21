//! Common macros for synthetic

/// shorthand to write a frame on the underlying stream
macro_rules! write_frame {
  ($frame: expr, $dst: expr, $size: expr) => {{
    $dst.reserve(2);
    $dst.put($frame.get_delimiter().as_slice());
    $dst.reserve($size);
    $dst.put($frame.to_bytes().as_slice());
    Ok(())
  }};
}

pub(crate) use write_frame;

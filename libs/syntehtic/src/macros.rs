//! Common macros for synthetic

macro_rules! write_frame {
  ($frame: expr, $dst: expr, $size: expr) => {{
    $dst.reserve(2);
    $dst.put($frame.get_delimiter().as_slice());
    $dst.reserve($size);
    $dst.put($frame.to_bytes().as_slice());
    Ok(())
  }};
}
/// ```
/// if let Ok(Some(response)) = timeout(Duration::from_secs(120), framed.next()).await {
///   match response {
///     Ok(frame) => match frame {
///       SyntheticFrame::ServerGreeting(f) => {
///         framed
///           .send(SyntheticFrame::SetUpResponse(SetupResponseFrame::with_mode(f.mode)))
///           .await?
///       }
///       _ => return Err(SyntheticError::UnexpectedFrame),
///     },
///     Err(err) => return Err(err),
///   }
/// }
/// ```
/// Checks if the arrived has arrived within 120 secs and is the one we are expecting, else
/// raise `SyntheticError::UnexpectedFrame`
macro_rules! expect_frame {
  // When nothing is passing to the function
  ($framed: ident, $expected_frame: path, $reply_frame: path, $reply_fn: path) => {{
    if let Ok(Some(response)) = timeout(Duration::from_secs(120), $framed.next()).await {
      match response {
        Ok(r) => match r {
          $expected_frame(frame) => $framed.send($reply_frame($reply_fn(frame.mode))).await?,
          _ => return Err(SyntheticError::UnexpectedFrame),
        },
        Err(err) => return Err(err),
      }
    }
  }};
}

pub(crate) use expect_frame;
pub(crate) use write_frame;

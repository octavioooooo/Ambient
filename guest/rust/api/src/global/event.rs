/// In Rust, functions that can fail are expected to return a [Result] type.
/// [ResultEmpty] is a [Result] type that has no value and can accept
/// any kind of error through the question-mark operator `?`.
///
/// It is used as the default return type for Ambient operations that take
/// a callback.
pub type ResultEmpty = anyhow::Result<()>;

/// The default "happy path" value for an [ResultEmpty]. You can return this
/// from an event handler to signal that there are no issues.
#[allow(non_upper_case_globals)]
pub const OkEmpty: ResultEmpty = Ok(());

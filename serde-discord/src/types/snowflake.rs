/// A unique identifier represented as a 64-bit unsigned integer.
///
/// This type alias is commonly used in various applications, such as
/// Discord, to represent unique entities like users, messages, or channels.
/// The underlying type is a `u64`, which allows for a large number of unique
/// identifiers.
///
/// # TODO
/// - Implement validation or utility functions to manage operations related
///   to Snowflakes (e.g., converting to/from strings).
pub type Snowflake = u64;

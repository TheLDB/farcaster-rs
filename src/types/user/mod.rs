/// # User Root Struct
/// 
/// When getting a user, we need structs to Deserialize the JSON into
/// 
/// This struct holds one value
/// 
/// * `result: Result`
///     - Result holds a User type, which itself holds a lot of the user data
///     - To view more about the data types, explore the many structs in this file.
pub mod user;
/// # Verified Address Root Struct
/// 
/// When getting a verified address, we need structs to Deserialize the JSON into
/// 
/// This struct holds on type
/// 
/// * `result: Result`
///     - Result is a struct that holds a vector of VerifiedAddress'
///     - To view the other structs that VerifiedAddress' gets Deserialized into, explore this file
pub mod verified_address;
/// # Follower Root type
/// 
/// When getting all the followers of a caster, we need structs to Deserialize the JSON into
/// 
/// * This is the Root struct for that, and it holds a Vec<Follower>
///     - Follower is also defined in this file, along with the Avatar struct, feel free to explore these to learn more about the data types
pub mod followers;
/// # Notification Root Struct
/// 
/// When getting all of the Notifications, we need structs to Deserialize all of the JSON into
/// 
/// This holds two different values:
/// 
/// * `result`: NotifResult
///     - NotifResult is also defined in this struct, feel free to explore it, there is no documentation for it
/// 
/// * `meta`: Meta
///     - Meta holds a next value which is the link to the next page of notifications for Pagination reasons
pub mod notifications;
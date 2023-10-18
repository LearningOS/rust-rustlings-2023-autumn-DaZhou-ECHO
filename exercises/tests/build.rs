//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
fn main() {
    // In tests7, set up an environment variable called `TEST_FOO`.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
   
    // Use std::env::set_var to set the environment variable.
    std::env::set_var("TEST_FOO", format!("Your command here with {}, please checkout exercises/tests/build.rs", timestamp));
    
    // In tests8, enable "pass" feature to make the testcase return early.
    std::env::set_var("CARGO_FEATURE_PASS", "1");
}
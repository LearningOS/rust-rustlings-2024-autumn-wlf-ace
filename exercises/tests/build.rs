//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
fn main() {
    // 在 tests7 中设置环境变量 `TEST_FOO`
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 在 tests8 中启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

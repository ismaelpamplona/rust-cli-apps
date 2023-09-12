use anyhow::Result;
use std::thread::sleep;
use std::time::Duration;

/// Finds and prints any pattern that matches the content.
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{} \n", line)?;
        }
    }
    Ok(())
}

/// Simulates a slow process work.
pub fn do_hard_work() -> Result<()> {
    let delay_duration = Duration::from_millis(50);
    sleep(delay_duration);
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let _ = find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum \n\n");
}

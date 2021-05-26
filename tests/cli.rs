use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("adparse")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    
    Ok(())
}

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::{self, Write};
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("adparse")?;

    cmd.arg("hoge").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nFirst content\nSecond content\nEnd of file")?;

    let mut cmd = Command::cargo_bin("adparse")?;
    cmd.arg("content").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("First content\nSecond content"));
    
    Ok(())
}
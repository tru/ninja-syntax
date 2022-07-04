#[cfg(test)]

mod inttests {
  use assert_cmd::prelude::*;
  use std::process::Command;
  use ninja_syntax::*;
  use tempdir::TempDir;

  #[test]
  fn run_ninja() {
    let dir = TempDir::new("ninja_syntax_test").unwrap();
    let buildninja = dir.path().join("build.ninja");
    let mut nw = NinjaWriter::new(&buildninja);

    nw.comment("Just a comment");
    nw.variable("cat", "cat", 0);

    let rule = NinjaRule::new("rcat", "$cat $in > $out");
    nw.rule(&rule);

    let mut build = NinjaBuild::new(&["test.out"], "rcat");
    build.inputs(&["/etc/passwd"]);
    nw.build(&build);

    // write the file to disk
    nw.close().unwrap();

    let mut cmd = Command::new("ninja");
    cmd.arg("-C").arg(dir.path().as_os_str());
    cmd.assert().success();
  }
}
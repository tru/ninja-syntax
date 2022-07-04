# ninja-syntax module for Rust

This is a port of [ninja_syntax.py](https://github.com/ninja-build/ninja/blob/master/misc/ninja_syntax.py) into Rust.
It allows you to easily create ninja build files from Rust with a syntax that is pretty similar to the official
python module from the ninja repo.

## Example:

```
use ninja_syntax::*;
use std::path::Path;

fn main() {
  let mut nw = NinjaWriter(Path::new("build.ninja"));
  nw.comment("Hello this is a comment");
  nw.newline();

  let rule = NinjaRule::new("cc", "cc $in -o $out");
  nw.rule(&rule);

  let mut build = NinjaBuild::new(&["test.o"], "cc");
  build.inputs(&["test.c"]);
  nw.build(&build);

  // write the file to disk
  nw.close().unwrap();
}
```


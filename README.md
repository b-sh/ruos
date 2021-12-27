# Intro

Just some experimental bare metal rust application targeting tricore architecture based boards.

UseCase: 
- rust for tricore

UseCase not in scope:
- other frontend languages like clang etc. - will probably do in another repo just to develop/test and 
debug the llvm tricore integration corretness

# Experimental

Idea:
- Compiling rust code with no_std etc. (minimalistic application) with --emit-llvm
  - "rustc src/main_no_core.rs --emit=llvm-bc --target=tricore-unknown-none-elf"
  - or "rustc src/main_no_core.rs --emit=llvm-ir --target=tricore-unknown-none-elf"
- Using bitcode to create elf objects for tricore based boards.
  - "llc main_no_core.bc --march=tricore"
  - or "llc main_no_core.ll --march=tricore"
  - creates Assmbler main_no_core.s file
- Using cargo build for the whole build process
  - "cargo build -Z build-std=core"
- Ultimate goal is to get some https://github.com/Infineon/AURIX_code_examples running

Exploring several things so direction might change very quick.

Current experiment:
- minimalistic application build by cargo build for tricore target
  - dependencies custom rustc and llvm repo with tricore experimental state
  - build run "cargo build -Z build-std=core"
  - build fails at the moment
- rust integration https://github.com/b-sh/rust/tree/bsh/tricore
  - to make the rust aware of the tricore target
  - if everything compiled just fine "rustc --print target-list" will show tricore-unknown-none-elf
- llvm integration https://github.com/b-sh/llvm-project/tree/bsh/tricore - release/13.x
  - compiled using vagga configuration or just the normal instructions of LLVM
    - warning: it needs around 20GB space for DEBUG build
  - linked into rustc see details in Backend section
- TODO whole vagga setup for the build process

# Details

## Frontend

Rust
- https://docs.rust-embedded.org/embedonomicon/custom-target.html
- https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html

~~~
# Just basing target description on someting similar
rustc +nightly -Z unstable-options --print target-spec-json --target riscv32i-unknown-none-elf >> tricore-unknown-none-elf.json
# And then modified to meet the backend requirements
# Build - this only works because there is a .cargo/config setting the default target
cargo +nightly build -Z build-std=core
# some other way cargo rustc -- --emit=llvm-bc
~~~

Using experimental tricore integration https://github.com/b-sh/rust/tree/bsh/tricore.

## Backend

LLVM way of choice with backend support in several experimetal repos.

- Need to make rust aware of custom LLVM otherwise 
~~~
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names --target tricore-unknown-none-elf --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 1)
  --- stderr
  error: Could not create LLVM TargetMachine for triple: tricore: No available targets are compatible with triple "tricore"
~~~
- https://internals.rust-lang.org/t/rustc-with-manually-built-llvm-toolchain/10407

- some other option https://stackoverflow.com/questions/52924569/how-can-i-compile-a-rust-program-with-a-custom-llc 


- https://rustc-dev-guide.rust-lang.org/building/new-target.html
  - ./x.py build library/std
  - rustup toolchain link stage1 build/x86_64-unknown-linux-gnu/stage1
  - cargo +stage1 build -Z build-std=core
  - set config.toml for rustc compilation - initial part is created automatically by x.py questioner
  - using pre-built LLVM based on - https://github.com/b-sh/llvm-project/tree/bsh/tricore
~~~
# Includes one of the default files in src/bootstrap/defaults
profile = "codegen"
changelog-seen = 2

[target.x86_64-unknown-linux-gnu]
llvm-config = "/path/to/llvm-project/llvm/build/bin/llvm-config"
llvm-filecheck = "/path/to/llvm-project/llvm/build/bin/FileCheck"
~~~
  - LLVM is missing some includes to be in build
    - just copied and rebuilt rustc
  - the other option using llvm in tree of rustc
  - https://rustc-dev-guide.rust-lang.org/getting-started.html

# References

- https://github.com/TriDis/llvm-tricore
- https://github.com/HeidiWindkraft/free_tricore_toolchain 
- https://github.com/kumailxp/tricore_llvm
- https://opus4.kobv.de/opus4-fau/files/1108/tricore_llvm.pdf 
- https://reup.dmcs.pl/wiki/images/7/7a/Tricore-llvm-slides.pdf 

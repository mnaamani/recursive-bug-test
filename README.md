Testing different rust compilers and how they handle infinite recursion bug:

// On apple M1 processor with rosetta2 emulation
1.46-x86_64-apple-darwin installed - rustc 1.46.0 (04488afe3 2020-08-24)

Mokhtars-MacBook-Pro:dbug-fmt-test mokhtar$ cargo run
 Compiling dbug-fmt-test v0.1.0 (/Users/mokhtar/dbug-fmt-test)
  Finished dev [unoptimized + debuginfo] target(s) in 0.75s
   Running `target/debug/dbug-fmt-test`
rosetta error: unexpectedly need to EmulateForward on a synchronous exception x86_rip=0x100b32b50 arm_pc=0x100b886e0 num_insts=5 inst_index=2 x86 instruction bytes: 0x56415741e5894855 0x4820ec8348535441
Trace/BPT trap: 5


nightly-x86_64-apple-darwin unchanged - rustc 1.53.0-nightly (07e0e2ec2 2021-03-24)

Mokhtars-MacBook-Pro:dbug-fmt-test mokhtar$ cargo run
 Compiling dbug-fmt-test v0.1.0 (/Users/mokhtar/dbug-fmt-test)
  Finished dev [unoptimized + debuginfo] target(s) in 0.37s
   Running `target/debug/dbug-fmt-test`

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
Abort trap: 6

# VM dispatch method benchmark

A benchmark of various VM dispatch methods.

## Methods

- `native` - native code included as a baseline.
- `treewalk` - basic tree-walk interpreter. An interpret function with a `match` in it that evaluates the result of an AST node
- `compact treewalk (dtable)` and `compact treewalk (switch)` - same as treewalk, but the AST is "compressed" into a compact bytecode representation; so a big `Instruction::Int` becomes encoded as 5 bytes (opcode + u32). `dtable` uses a function dispatch table for dispatching opcodes and `switch` uses a match.
- `stack (dtable)` and `stack (switch)` - stack machine; each opcode operates on an implicit stack, eg. `Int 1` pushes the literal integer 1 onto the stack, `Add` pops two integers off the stack and adds them together. Similarly to `compact treewalk`, the `dtable` variant uses a function dispatch table and `switch` uses a `match` for dispatching opcodes.
- `register (dtable)` and `register (switch)` - register machine; each operation has registers as its operands like on x86 - eg. `%0 = Add %1, %2`. `dtable` and `switch` meaning's the same again.

The `compact treewalk (dtable)` method is used by the Unreal Engine VM (and it dates back to the good ol' days of UnrealScript.)

The `stack (switch)` is used by my own programming language, [Mica](https://github.com/mica-lang/mica).

The `register (switch)` method is used by Lua.

## Benchmark

The benchmarked (byte)code is computing 10 factorial, using the following function (in C-like pseudocode):

```c
uint32_t factorial(uint32_t n) {
    uint32_t x = 1;
    uint32_t i = 1;
    while (i <= n) {
        x = x * i;
        i = i + 1;
    }
    return x;
}
```

`while` is used in this example for explicitness, as `for` is just syntax sugar over it.

Note that no compound assignments are used. Each VM implements assignment as if it were evaluating the right-hand side fully and then assigning the result to a variable, for simplicity sake. While each of the VMs could use more specialized instructions, I decided not to have them to purely benchmark dispatch/implementation methods.

Each implementation is [tested](tests/tests.rs) for correctness.

The various types of VMs are implemented in Rust and optimized to not contain any bounds checks, so in reality it's almost as if they were written in C. However, one caveat of using Rust is that we cannot test direct threading-based dispatch, since that requires tail calls or computed goto, and Rust has neither of them. (Tail calls can be achieved by relying on the optimizer, but in a real-world scenario you probably don't want your stack to overflow in debug mode, where this optimization is disabled.)

The benchmarking code uses the [criterion](https://lib.rs/crates/criterion) crate to measure performance. Benchmarks were conducted on an AMD Ryzen 5 1600 running Windows.

```text
native                  time:   [6.1612 ns 6.2052 ns 6.2548 ns]

treewalk                time:   [473.45 ns 476.01 ns 479.08 ns]

compact treewalk (dtable)
                        time:   [553.99 ns 554.77 ns 555.73 ns]

compact treewalk (switch)
                        time:   [713.00 ns 714.09 ns 715.26 ns]

stack (dtable)          time:   [649.28 ns 652.44 ns 655.76 ns]

stack (switch)          time:   [417.78 ns 433.00 ns 453.07 ns]

register (dtable)       time:   [372.83 ns 380.13 ns 387.56 ns]

register (switch)       time:   [118.89 ns 119.13 ns 119.40 ns]
```

See the [Criterion.rs documentation](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html#time) to see what these numbers mean.

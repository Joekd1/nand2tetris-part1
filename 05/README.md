## Chapter 5: Computer Architecture

### Reading notes

This is the part where everything comes together. We use the chips built
in the previous chapter to construct the **entire Hack computer** step by step:

- We build the memory chip which contains a *RAM16k*,
and built-in *Screen* and *Keyboard* chips.
- We build the CPU using tha previously built *ALU*, registers and PC.
- We put everything together in the top-level *Computer* chip.

### Memory

 The main chips that constitutes the `Memory` chip are : RAM16K, Screen memory
 and Keyboard Memory.

The design is inspired by the previous memory designs:

- A Dmux chip directs the load between the first 16k Memory and the rest of the chip.
- The load is then directed to the data mem or to the Screen. The keyboard is read-only.
- The output is selected using a couple of Mux16 chips and some other logic gates
  - An Or16 chip,an Or chip, and a Mux16 selects the keyboard output.
  - This makes sure that the keyboard outputs only when 0x6000 is selected
  and 0 for the higher addresses.
  - A mux16 then selects between the Keyboard and the Screen.
  - The last Mux16 then selects between the Memory Mapped I/o and the Data Memory.

### CPU (Central Processing Unit)

The *CPU* is responsible for fetching, decoding and executing the Hack machine language
instructions. The main chips are the ALU, Aregister, Dregister and PC.

1. **Instruction Decoding:**
    - The `instruction[16]` input is the current instruction to be executed.
    - The most significant bit (`instruction[15]`) distinguishes between A-instructions
    (value begins with `0`) and C-instructions (value begins with `1`).
    - The A instruction is stored directly in the Aregister.
    - The C instruction is used as a capsule of control bits.
    - A `Mux16` is used to select between the A instruction or C instruction.
    - The *A register* either accepts the A instruction, or the result of the
    *ALU* computation.

2. **Instrution Execution:**
    - An A instruction is stored directly in the A register.
    - The C instruction is of the form `1xxaccccccdddjjj`
    - The `a` bit determines if the *ALU* will be fed from the Aregister or *inM*
    - A `Mux16` chip selects between M or A and feeds it to the *ALU* along with
    the value stored in the D register.
    - `cccccc` encodes which functions will be executed by the *ALU*.
    - These bits are fed directly to the six *ALU* functions.
    - `ddd` encodes where the value of the computation will be stored (A|D|M).
    - These bits are used to determine the *load* of the A register and D register.
    Additionlly, they determine the output *writeM*.
    - `jjj` encodes the jump information.

3. **Instruction Fetching:**
    - The `PC` chip determines which instruction will be fetched and executed next.
    - The default behavior of the `PC` is PC++
    - In the case of a jump PC should output the value stored in A.
    - The jump is detrmined by the `jjj` bits and the `ng` and `zr` output
    of the *ALU*
    - I used a `DMux8way` to determine which jump condition is encoded by `jjj`
    - I use basic `And` and `Or` gates to determine if the requested jump is satisfied.
    - An `Or8way` gate determines if there's a jump.
    - The result of this entire computation is the *isJMP* bit, which is fed to the
    *PC* load.
    - The reset input is fed directly from the reset bit of the entire chip.

**Notes** : There could be a better way to handle the jump logic.
While my method works, It feels a little bit verbose.
This was more challenging than the previous chips.

### Computer

The topmost `Computer.hdl!` can be realized using three chips:
    - `ROM32K` instruction memory chip
    - `Memory` chip built previously in this chapter
    - `CPU` chip built in this chapter

The summary of the design:
    - THe `PC` chip feeds the `ROM32K` causing it to emit the `instruction`
    - The `CPU` executes the `instruction` and may manipulate `Memory`
    - The `CPU`figures out which instruction to fetch next and outputs in to `PC`
    - The cycle starts again

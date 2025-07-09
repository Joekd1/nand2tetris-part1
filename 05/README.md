## Chapter 5: Computer Architecture

### Reading notes

This is the part where everything com together. We use the chips built
in the previous chapter to construct the **entire Hack computer** step by step:

- We build the memory chip which contains a *RAM16k*,
and built-in *Screen* and *Keyboard* chips.
- We build the CPU using tha previously built *ALU*, registers and PC.
- We put everything together in the top-level *Computer* chip.

### Memory

 The main chips in that constututes the *Memory* chip are : RAM16K, Screen memory
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

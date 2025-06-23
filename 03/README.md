## Chapter 3 : Memory

### Reading notes:
- Our computer needs to store values and "remember" them later.
- This can be done using a new set of *memory chips*
- In order to implement this functionality, it is necessary to model time.
- Our model of time will be discrete. We ignore continuous change and care about the state of the world only during successive cycles
- The discrete model of time helps in ignoring the randomness of communication and synchronizing the operation of multiple chips.
- These cycles are achieved with the help of a *clock* and lower level Data-flip flop (DFF) chip.
- The duration of the clock cycle should be slightly longer than the most time consuming operation.
- The clock signal, realized using a binary signal, is broadcasted to every memory chip.
- Adding the dimension of time transforms our *classical logic* into *sequential logic*.
- The family of memeory chips will be implemented gradually: DFF, Bit chip, Register chip, and successive RAM implementation of different sizes.

### Bit:

The Bit chip is expected to remember and emit its state over time. This can be implemented using a DFF and a multiplexer, with the *load* bit of the multiplexer being fed to the *sel* bit of the chip. This has the effect of either outputting the *in* value or the *currentVal* fed previously by the DFF, depending on the *load* bit. 

### Register:

A register is an array of n-bits, 16-bits in our case.

First try: It can be implemented by using the same load for the 16 Bit chips. Is this an efficient way?

### RAM8:

A *RAM8* chip features 8 registers. Each register can be selected using the RAM's 3-bit input address. Reading an inpit means we select a register number *address* and pipe its output to the RAM's output. The act of writing means given an address and a load, we set the value of the reigster number *address* to *in*.

First try: I use 8 registers with the same input, different loads, and different outputs. The output is selected suing a *Mux8way16* chip, developed in chapter 1. The load is directed to a specific register using a *Dmux8way* chip.



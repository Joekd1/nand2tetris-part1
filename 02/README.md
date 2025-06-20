## Chapter 2 : Boolean Arithmetic

### Reading notes:
- This chapter focuses on building an Arithmetic Logic Unit (ALU)
- The ALU will be used later to build the CPU
- We start from the logic gates built on chapter 1
- We build a half-adder, full-adder, an adder and finally the ALU
- Implementing addition and using the Two Complement method for signed integers is enough to represent all arithmetic operations.
- Notes: for simplification we only use integers (no floating-point arithmetic) and we ignore integer overflow.

### Half-adder :

The half-adder computes the sum of two 1-bit inputs, producing a *sum* and a *carry*.  

From the truth table we can see that the sum is logically equivalent to an Xor gate while the carry is logically equivalent to an AND gate.

### Full-adder:

The addition of two binary integers with more than one digit may result in a carry, requiring us to compute the sum of three binary digits. 
The full-adder computes the sum of three 1-bit inputs, producing a sum and a carry. 
Addition is a binary operation, so in order to compute the sum of a, b and c we proceed this way:
- First compute the sum of a and b using a hal-adder. This produces the sum S1 and carry C1
- We use a second half-adder to compute the sun of S1 and c. This produces S2 and C2
- Note that S2 is the rightmost digit of the entire sum so it is the *sum* of the full-adder
- The leftmost digit C is 1 only if either C1 or C2 is 1. C1 and C2 cannot be both 1.
- C can be obtained by an Or gate
- Therefore the full-adder can be implemented with two half-adders and one Or gate.

More details about the entire process can be found in *Discrete Mathematics with Applications* by *Susanna Epp* (5th ed) pages 97 - 99

### Adder16 :

The Adder16 proceeds bitwise, from right to left. In step 0 the least significant bits are added and the carry is fed to the next addition. The last overflow carry bit is ignored.  

First try: Can be implemented with a half-adder for the first addition and full-adders for the rest. Will investigte if it can optimised.
Verdict: The implementation looks good. There's a question of whether to use a full-adder evem for the least significiant digits. I'll still use the half-adder as it seems clearer.

### Incrementer:

An incremeter takes a number and output that number + 1. It could be implemented with the general adder, but a dedicated gate is more optimal, since incrementing is a common operation. 

First try: Can be implemented with half-adders. Is this the most optimal way? 

verdict: I think this is optimized enought for educational purposes. There might be a way to stop propagating the carry when the first 0 is encountered, since in addition a bit only flips when all other previous bits are 1. Howver, this might be a bit too complex to implement for now. I'll look into it later.

### ALU:

This chip is designed to compute a set of arithmetic operations. It takes a two 16-bit integers and depending on the value of six 1-bit control bits, performs a given operation. The general specification of the chip is already provided. 

First try: I compute each operation using the specific gate and use Mux16 to select whether it is performed or not. Each output is then fed as the input of the next operation. 

Gotchas: In order to output zr I needed to use *Or8way* gate. This requires splitting the output of the 16-bit into two 8-bit parts. It's impossible to index or subscript internal pins in HDL. I used a work around that uses two Mux16 but outputs the lower and upper bits seperately. I have a "smell" that this is not the most optimal approach. There are perhaps way more Mux16 gates than required. Will investiaget this and see. 

Verdict: I used one Mux16 and routed the 8-bit parts, ng and the full output from it. I think it's sufficently optimized now. As the code gets more complex I am getting more careful with the naming, to make it more readable.

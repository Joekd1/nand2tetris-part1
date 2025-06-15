## Chapter 1 : Boolean Logic

### Reading notes:
- The universality of the Nand gate: every boolean function can be realized using Nand only.
- Two ways to represent a boolean function : truth tables and boolean expression. 
- It is important to be able to convert between the two.
- A gate is a physical device that implements a simple Boolean function.
- Logic design : Finding the most efficient way to implement a logic gate from its specification. 
- We use HDL (Hardware Desciption Language) to design and simulate the gates.

### Not Gate:

Given the rules of Boolean algebra : ` ~ a = ~ (a ^ t)` which is equivalent to a Nand(a, True)

### And Gate:

`a ^ b = ~ (~ ( a ^ b ))` which can be represented as Not(Nand(a, b))

### Or Gate:

According to De Morgan's Laws `a ∨ b = ~ ( ~ a ∧ ~ b) ` which can be represented as Not(And(Not(a), Not(b)))

### Xor Gate:

Xor can be represented in Boolean notation as : `(a V b) ^ ~ (a ^ b)` which can be represented as And(Or(a,b), Not(And(a,b)))

### Multiplexer Gate:

The multiplexer selects between inputs a and b based on select bit *sel*. This can be noted as : `(~ sel ^ a) V (sel ^ b) `

### Demultiplexer Gate:

From the truth table provided in the book : `a=¬sel∧in,b=sel∧in`

### Multi-bit Not/And/Or and Mux:

It's a simple matter of arranging the one-bit gates into n-bit arrays that operate on each bit separately. In this case we implement 16-bit version of the gates. 
- mistakes and gotchas: I first implemented the gates by copying the implementation of each gate 16 times. It's easier and and cleaner to use the Not, Or, and And gates already implemented. This is more modular as you don't have to worry about how a gate is implemented.

### Multi-way gates:

I found some of those a little bit challenging. But working through the truth tables, you can find a way to fork the one-bit or multi-bit varient depending on the gate.

- Miastakes and gotachas: I made the rookie mistake of thinking of sel[0] as the left-most bit, which led to some confusion.

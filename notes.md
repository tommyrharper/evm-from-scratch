# Notes

## Eth Yellow Paper

### Execution Environment - 9.3 - pg 13

- σ - the system state
- *g* - the remaining gas for computation
- A - accrued substate
- *I* - tuple with important execution env. information - Ia => address of smart contract, Io => sender address etc
- o - resultant output
- Ξ - execution function

```
(σ′, g′, A′, o) ≡ Ξ(σ, g, A, I)
```

### Execution Overview - 9.4 - pg 13

- Execution function is formally defined with a recursive function `X`
- This uses an iterator function `O` (which defines the result of a single cycle of the state machine)
- Functions `Z` which determines if the present state is an exceptional halting state
- `H` specifying the output data of the instruction if and only if the present state is a normal halting state

- μ - the machine state
- μg - gas, μ′g => remaining machine gas
- () - empty sequence
- ∅ - empty set
  - different to () => the output of H, which evaluates to ∅ when execution is to continue but a series (potentially empty) when execution should halt.

```
Ξ(σ, g, A, I) ≡ (σ′,μ′g, A′, o)
(σ′, μ′, A′, ..., o) ≡ X((σ, μ, A, I)) 
μg ≡ g
μpc ≡ 0
μm ≡ (0, 0, ...)
μi ≡ 0
μs ≡ ()
μo ≡ ()
```

### 9.4.1. Machine State - pg 13

- The machine state μ is defined as the tuple:
-  (g, pc, m, i, s) 
   -  g - which are the gas available
   -  pc -the program counter pc ∈ N256
   -  m - the memory contents
   -  i - the active number of words in memory (counting continuously from position 0)
   -  s - the stack contents. 

- μm - The memory contents are a series of zeroes of size 256
- w - the current operation to be executed
- δ - the stack items removed
- α - the stack items added
- C - instruction cost function evaluating to the full cost, in gas, of executing the given instruction.


(w = JUMP ∧ μs[0] ∈/ D(Ib))

## Execution evn info

• Ia, the address of the account which owns the code that is executing.
• Io, the sender address of the transaction that orig-
inated this execution. (141)
• Ip, the price of gas in the transaction that origi- nated this execution.
• Id, the byte array that is the input data to this execution; if the execution agent is a transaction, this would be the transaction data.
• Is, the address of the account which caused the code to be executing; if the execution agent is a transaction, this would be the transaction sender.
• Iv, the value, in Wei, passed to this account as part of the same procedure as execution; if the execution agent is a transaction, this would be the transaction value.
• Ib, the byte array that is the machine code to be executed.
• IH, the block header of the present block.
• Ie, the depth of the present message-call or contract-creation (i.e. the number of CALLs or
CREATE(2)s being executed at present).
• Iw, the permission to make modifications to the
state.
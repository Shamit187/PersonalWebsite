# Verilog Design

## Datatypes

Datatypes in verilog is not enough to develop efficient tesbenches. Hence system verilog introduces new C-like datatypes.

Verilog had
:::list
- reg 
- wire
- integer
- real
- time
- realtime
:::

System verilog introduces
:::list
- logic
- bit
- byte
- shortint
- int
- longint
- shortreal
:::

datatypes can hold 4 types of values
:::list
- 0
- 1
- x (we don't know what value it holds)
- z (high impedance, wire disconnected / shorted)
:::

!!! Note
*Concatenation Operation*: c= {a,b} concatenates a and b into c. c={2{a}} concatenates a with itself 2 times. *Shorthand*: '1 will fill all the bits with 1. ` operator only works with 1/0/x/z, nothing multi-bit.

!!! Warning
'F0 is wrong but 8'hF0 is correct. Only ' is filling operation and can only take one bit.

!!! Note
*Autopadding of Data*: Pads the data to the left with 0/x/z depending on the msb of the written data. if the msb is 0/1, it pads with 0. If the msb is x/z, it pads with x/z.

# Primes of the form x^2 + ny^2

# Personal Website Modification
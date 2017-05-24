# About

Hashfuck is a programming language which was inspired from the various Brainfuck derivatives found on [esolangs.org]. Hashfuck's goal was to make the hardest possible language to program in.

# Example program

The following program prints the ASCII character 0x01 an infinite times to STDOUT:

```
sha256:93f74a28b6d648aec2170182353d0f0fc69072ec1581e49a53cc2f1533455106
```

# Interpreter

1. The input hash is hashed by the algorithm specified before-hand.

2. Bytes from a hash are mapped to Brainfuck code. E.g.:

   0x93 = 147
147 % 8 = 3

Brainfuck has 8 instructions. A result of 3 yields the Brainfuck instruction '-'.

This mapping is done for the whole hash until 0xff is encountered. The interpreter will stop there.

3. If the interpreter reaches the end of the string, the hash is hashed again with the algorithm specified and the new hash will be interpreted again (step 2).

# Hello World!

It is unknown if "Hello World!" is possible at all. 

The "easy" solution would've been to find an input hash which, after rehashing, yields a hash that contains the whole "Hello World!" program.

The shortest "Hello World!" [https://www.reddit.com/r/tinycode/comments/1oqgwm/shortest_hello_world_brainfuck_code/cdsn4mb/](I've seen) so far is:

```--[+++++++<---->>-->+>+>+<<<<]<.>++++[-<++++>>->--<<]>>-.>--..>+.<<<.<<-.>>+>->>.+++[.<]```


Which is 88 characters long. Therefore, "Hello World!" in Brainfuck does not fit into a single SHA-512 hash which has a length of 64 bytes.

One would have to find a hash, which yields after re-hashing the rest of the program. Even if such a splitted solution exists, it's highly unprobably to be found.


# Regex-Engine
This repository shows how to build a Regex-Engine from Scratch

Several Steps are needed to convert a Regex into executable code.

1. Create an AST from the Regex to define the structure.
2. Use Thompson construction to build the NFA that decribes the Regex.
3. Create a DFA from the NFA with subset construction. 
4. Use Hopcroft's or Brzozowski’s algorithm to simplify the DFA.
5. Convert the DFA to executable code.

In this repository all these steps are implemented and a learning project to see how all these algorithms work together to create executable code from a regex.


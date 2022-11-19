# doumi (dumi)

> A very simple text-format based on a [deadfish](https://esolangs.org/wiki/Deadfish) interpreter

Ops Supported:
---

| Op | Description |
|----|-------------|
| i  | Incement the stack    |
| d  | Decrement the stack   |
| o  | Output top value of stack      |
| s  | Square top value of stack      |
| r  | Reset stack       |

Other Features:
---

- single-line comments

Example:
```deadfish
# A comment
ii
o # should print 2
```

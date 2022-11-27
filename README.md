# doumi (dumi)

> A very simple superset text-format based on a [deadfish](https://esolangs.org/wiki/Deadfish) interpreter

Ops Supported:
---

| Op | Description |
|----|-------------|
| i  | Incement the stack    |
| d  | Decrement the stack   |
| o  | Output top value of stack      |
| p  | Output top value of stack and interpret as ASCII Chr      |
| s  | Square top value of stack      |
| r  | Reset stack       |


Other Features:
---
- repl-mode
- single-line comments

Example:
```bash
# A comment
ii
o # should print 2
```


Hello, World! in Doumi:
---

```bash
iiiiiiiiisddddddddd           p   # H
iiiiiiiiiiiiiiiiiiiiiiiiiiiii p   # e
iiiiiii                       pp  # ll
iii                           p   # o
r iiiiiisiiiiiiii             p   # ,
r iiiiiisdddd                 p   # SPC
r iiiiiiiiisiiiiii            p   # W
r iiiiiiiiiiisdddddddddd      p   # o
iii                           p   # r
dddddd                        p   # l
dddddddd                      p   # d
r iiiiiisddd                  p   # !
```

Extended ASCII
---
```bash
iiisiisiiiiiii # 128

# print the extended ascii chars
#   ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ
pipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipip
ipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipip
ipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipipip
```

More examples [here](./testprograms)

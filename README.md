## Calculator with LR Parser

### Grammar

- E' -> E
- E -> T + E
- E -> T - E
- E -> T
- T -> F \* T
- T -> F / T
- T -> F
- F -> n
- F -> ( E )

### Tools

- [LALR Parser table generator](https://jsmachines.sourceforge.net/machines/lalr1.html)

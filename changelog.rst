CHANGELOG
=========

0.5.0
-----
- Add env module with `Env` type
- Use env as REPL environment instead of a HashMap
- Pass REPL env as argument to `rep` function
- Implement `def!`
- Implement `let*`

0.4.0
-----
- Add `Float` type
- Basic arithmetic operations work with ints and floats

0.3.0
-----
- Add REPL environment with arithmetic operations
- Implement basic eval

0.2.3
-----
- REPL ignore blank lines

0.2.2
-----
- Remove `Ast` type and use `Type` instead

0.2.1
-----
- Rename `MalType` to `Type`
- `True` and `False` are now inside the `Bool` type
- Rename `Integer` to `Int`

0.2.0
-----
- Add initial types (types module) and AST type
- Add basic lexical and syntax analysis. Read nil, true, false,
  integers, symbols and lists. (reader module)
- Add printing capabilities (printer module)

0.1.0
-----
- Add stub read, eval, print functions
- Add test generator from MAL text tests
- Implement a REPL in main

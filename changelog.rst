CHANGELOG
=========

0.45.2
------
- Fix unbalanced string detection

0.45.1
------
- Fix `rest` core function

0.45.0
------
- Implement `readline` core function

0.44.0
------
- Implement new core functions: `symbol`, `keyword?`, `keyword`,
  `vector?`, `vector`, `sequential?`, `hash-map`, `map?`, `assoc`,
  `dissoc`, `get`, `contains?`, `keys`, `vals`

0.43.0
------
- Implement `apply` core function
- Implement `map` core function
- Add predicate functions: `nil?`, `true?`, `false?` and `symbol?`

0.42.0
------
- Implement `try*/catch*` special forms and `throw` core function

0.41.0
------
- Add error module to implement basic exceptions
- Convert all string errors to exceptions

0.40.0
------
- Add `cond` macro to core mal definitions

0.39.0
------
- Implement Clojure-style variadic function parameters

0.38.0
------
- Implement `nth`, `first`, `rest` core functions

0.37.0
------
- Improve closure printing

0.36.0
------
- Add `is_macro` attribute to closure type
- Add new special form `defmacro!`
- Add `is_macro_call` function
- Add `macroexpand` function
- Use `macroexpand` before special form switch in eval
- Add `macroexpand` special form

0.35.0
------
- Implement the core function `vec`
- Improve `quasiquote` and add `vec` in its processing

0.34.0
------
- Implement quoting related reader macros

0.33.0
------
- Implement `quasiquote` and `quasiquoteexpand` special forms

0.32.1
------
- Improve code using clippy suggestions

0.32.0
------
- Implement `quote` special form

0.31.0
------
- Implement `cons` and `concat` core functions

0.30.0
------
- Add step 7 mal tests

0.29.0
------
- Add ability to run mal programs by command line and add \*ARGV\*
  symbol

0.28.0
------
- Implement @ macro

0.27.1
------
- Fix bug with string functions

0.27.0
------
- Add MAL tests and test runner (runtests.py)

0.26.0
------
- Move escape and unescape string functionality into a utils module

0.25.0
------
- Add error checking to reader to make sure parens are properly
  matched

0.24.1
------
- Fix wrong behavior while reading comments

0.24.0
------
- Read functions handle errors and return a `Result`

0.23.0
------
- Implement `atom`, `atom?`, `deref` and `reset!` functions
- Print atom as `(atom X)`
- Implement `swap!` function

0.22.0
------
- Implement `Atom` type
- Add doc to Env public functions
- Comments are detected in reader.read_str instead of read

0.21.0
------
- Definitions using the interpreter itself are read from a file
- Add load-file to core.mal definitions

0.20.1
------
- Fix eval to use the outermost environment
- Update Env to use RefCell on data and not in outer.
- Remove RefCell wrapping the REPL env

0.20.0
------
- Implement `read-string` function
- Implement `slurp` function
- Implement `eval` as a new symbol in the REPL

0.19.0
------
- Implement tail call optimization in eval

0.18.0
------
- Implement `HashMap` type

0.17.0
------
- Implement `String` type
- Add lazy_static crate and use it for regex compilation
- Implement string functions: `prn`, `println`, `pr-str`, `str`  

0.16.0
------
- Add comment support to the interpreter. Lines starting with ; are
  considered comments and ignored.

0.15.0
------
- Implement `Keyword` type

0.14.1
------
- Fix `Vector` functionality to be used as `List`

0.14.0
------
- Implement PartialEq for Type

0.13.0
------
- Implement `convert_to_vec` in Type and update `is_empty`, `count`
  and `=` functions to work with Vectors

0.12.0
------
- Implement `not` using the interpreter itself

0.11.0
------
- Implement `Vector` type

0.10.0
------
- Implement core functions: `prn`, `list`, `list?`, `empty?`, `count`,
  `=`, `<`, `<=`, `>`, and `>=`

0.9.0
-----
- Add `core` module with `Namespace` struct and arithmetic functions
  (+, -, *, /)

0.8.0
-----
- `Env` derive `Clone`
- The REPL environment is wrapped with Rc and RefCell

0.7.0
-----
- Implement `fn*` and the Closure type

0.6.1
-----
- Use Rc in Env outer environment instead of references with lifetime

0.6.0
-----
- Update `Env::new` to take two new parameters: `binds` and `exprs`
- Split tests in multiple files
- Add support to printer to print function values
- Add `print_readably` argument to print
- Implement `do`, `if`

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

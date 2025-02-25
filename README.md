# Advent of Code 2024

This year I'm solving the problems in a new language every day! That is certainly a crazy thing to do, but the idea is to get a broader perspective on software develpment. The focus will mostly be on getting a feel for what it's like to use the language itself, but I will also be checking out the ecosystem around it. This might include build systems, dependency management, packaging, testing and the quality of the documentation. Each language probably gets 3 - 12 hours of attention as part of this process.

To make the process easier I have developed my own build system / test runner (located in the `runner` folder). That would normally not be advisable (it takes a while) but it was really fun to make and I have learned a lot!

## Running the code

1. Build the build system (only needed once):
    ```bash
    task build
    ```

    This creates a program in `.bin/` called `advent`.

2. For every terminal you open, run this:
    ```bash
    source setup.sh
    ```

    You should now have the `advent` command available in the terminal.

3. To run the code for day 1, run this:
    ```bash
    advent -d 1
    ```
    
    That will build the program for that day and run all the test cases.

4. If that didn't work you might need to install a compiler or something. Check the `Taskfile.yaml` for any suitable `install-` command. There should be one such command for each language, like this one:
    ```bash
    task install-gleam
    ```
    Note: these might not work on you computer depending on the OS.

## Configuration per day

Each day has its code in the `days/d<nn>` folder. The config is in a subfolder called `.advent` which contains a `run.kdl` file and a `testcases` folder.

Note: Only the small test cases are included in git, so to have the big test case one can add `b.secret.in` and `b.secret.out` into the `testcases` folder.

The idea is that all files needed for a particular day should be in the folder for that day, which makes it very convenient to update the files, and difficult to update the config for another day by mistake.

## Programming Languages

The language used per day:

1. Roc
2. Gleam
3. Elixir
4. OCaml
5. Clojure

## Insights about each language

### Rust (used for the runner)
- Best language I have ever used
- Great for CLI programs
- Tooling is reliable and intuitive
- Mistakes are found early and error messages are helpful
- If it compiles it probably works
- Much more!

### Roc
- Promising language with some interesting features
- Compiles to machine code, so should be fast
- Language features are still being added and removed, so best to not use the language yet
- Compiler has to be installed manually due to early days, but that will likely improve

### Gleam
- Very nice language!
- Statically typed, which helps a lot during development
- Compiles to machine code, so should be fast
- Code is easy to read and write
- Great tooling, just like in Rust

### Elixir
- Dynamically typed, which means more confusion
- Easy to read and write

### OCaml
- Interesting type system, e.g. `int list` instead of `list<int>`
- Compiles to machine code, so should be fast
- Build system and package manager look very capable

### Clojure
- Would not recommend it!
- Compiler seems very slow, even for tiny programs
- It's dynamically typed, which means that type errors are found at runtime and the cause is not clear from the error message
- Most error messages complain about something not being the correct Java class, which is surprising for a language that looks like Lisp
- The link to Java is just way too strong; sometimes one function returns one kind of list and another function needs another kind of list
- It's hard to search for "Clojure" online because it means something else
- The code formatter makes the code inconvenient to work with

### Odin
- Very cool language with a lot of interesting and useful features!
- Fast, flexible, expressive
- Enums are simple but convenient
- Feels a bit like Go, but nicer
- Instead of "methods" you have functions in modules (e.g. `list.map(foo, ...)` instead of `foo.map(...)`), which is fine and I can probably get used to it
- There are lots of cool features left to check out
- I might actually start using this language in some project

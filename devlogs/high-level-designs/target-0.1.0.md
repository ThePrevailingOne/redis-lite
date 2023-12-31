# Target 0.1.0

## High-level Overview
- Firstly, we want to design the server such that it is extensible enough to add new components. In the first iteration, architecture design should be simple and minimal.
- Breaking down the steps to process an incoming request from client:
  - Parse request
  - Process request:
    - Identify commands
    - Run command accordingly
    - Access Memory (if needed)
  - Return response to client

- Hence, following an event driven architecture, a pipeline design will fit.
- Additionally, patterns like:
  - Chain of Responsibility: in pipeline, handling errors
  - State: handle state of memory (combined with locks, for concurrency safety)
  - Strategy: to process commands and different strategies

- The current main components used are:
  - Sessions and SessionManager
  - RequestTokens, Request, and RequestParser
  - Command and CommandManager
  - Memory and MemoryManager
  - Response

- As of now, the steps to complete are:
  - [x] Create TCPListener
  - [x] Manage multiple user sessions
    - [x] Use async and create "tasks" (kind of like goroutines) 
    - [x] Create Session
    - [x] ~~Create SessionManager~~
    - [x] Implement barebone handle_session
      - [x] Read user input
      - [x] Respond to user
    - [x] Incorporate
  - [ ] ~~Tokenize request~~
    - [ ] ~~Create RequestTokens~~
    - [ ] ~~Create RequestTokenizer (or just a func of it)~~
    - [ ] ~~Incorporate~~
  - [x] Parse ~~tokens to~~ request
    - [x] Create Request
    - [ ] Create RequestParser (or just a func of it)
    - [x] Incorporate
  - [ ] ~~Transform request to commands via CommandManager~~
    - [x] Create Command
      - [x] PING
      - [x] ECHO
      - [x] GET
      - [x] SET
    - [ ] ~~Create CommandManager (or just a function of it)~~
    - [x] Incorporate
  - [x] Implement Memory
    - [x] Create Memory
    - [ ] ~~Create MemoryManager (add MPSC + task)~~
    - [x] Incorporate
  - [x] Implement Response
    - [x] Create Response
    - [x] Incorporate
    

- It should be preferable to create unit tests and place the mods and crates in the right folder
- To keep code readable keep data immutable if possible
- If data is mutable, then it needs to act as a state/memory
- Change println! with logger instead for better clarity :D

## Plannings, Decisions & Learning Points
- Session manager is not needed as of this milestone since session is a very simple struct
- Tokenizing request is probably not needed since the data format of RESP is very straightforward
- Request is technically just an AST where we have:
  - Node: it's a basic node with types attached to it
  - Simple: it's a leaf, so it stores the extra bytes that can be accessed
  - Aggregate: it's an inner leaf, so it stores other nodes that can be accessed

- Since there is no inheritance, interface can be segregated into multiple traits
- To traverse buffer effectively, using reference effectively should pay off
- Using Rust enum gives a nice (yet unusual) approach to enumerating a class
- Ownership & reference is once again a bit confusing (TODO: need to read more abt this)
- TODO: optimization/design
  - Parsing can be faster for bulk string (since length is given)
  - Create a parser class (have the parser store the buffer/reference the buffer)
    - So all parsing functions have a unified access to the buffer without passing over and over again across parameters
    - Lifetime of buffer is more obvious
- About Trait and Enum:
  - Trait and Enum kind of fills similar role in terms of interfacing, however:
    - Trait is usually dynamically type when used while Enum is statically typed. Since Enum's size is already known (implementing Sized(?)), it can be statically typed. As for Trait, the size is unknown since it's basically just an interface. In Rust, Trait does not represent a concrete object while Enum does. So, usually, Trait is accessed via `Box<dyn Trait>` (stored in memory and dynamically sized)
    - `impl Trait` and `Box<dyn Trait>` are used differently. `impl Trait` is used closer as a mask(?) for the actual object/struct implementing the trait. Rust is able to infer what is the actual object implicated `impl Trait` (The size is known). So, using impl Trait for different constructs/object types may not be the best approach since when Rust tries to resolve/infer the actual object, it will result in a compile error
    - Additionally, Trait can be considered an open-set of objects while Enum is a closed-set of objects. Which means, Enum is staically typed and hence, all the possible objects will need to implement certain methods of the Enum
    - Enum should be used if we can ensure that the objects using that Enum is more likely fixed since it's most likely violating OCP when implementing the methods.
- Request and Response is actually of RESP data type, so the only thing required is actually a deserializer/parser and serializer
- Among [all other data format](https://en.wikipedia.org/wiki/Comparison_of_data-serialization_formats), RESP can be considered easy enough to use while the use case are a bit more specific.
- Testing and Project Structure is quite straightforward to use.
- [Just(file)](https://github.com/casey/just) is nicer to use in comparison to Make(file) for making scripts
- Learning to justify when to clone and use references(&). References are a bit complicated
  - References are always tied to a lifetime to keep Rust memory safe.
  - Lifetime specifiers are essential to annotate and dictate to rust the lifetimes of objects
- Handling errors is interesting in Rust
  - Thanks to the existence of `Result<T, S>`, errors don't have to expicitly be of type `std::error::Error`
  - While in general Rust runtime exceptions result in `std::error::Error`, we can catch it and convert it to another error.
  - `?` can quickly throw error to the outer closure.
  - Additionally, there's also `Option<T>` that helps identify "nulls".
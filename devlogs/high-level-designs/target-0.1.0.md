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
  - [ ] Transform request to commands via CommandManager
    - [ ] Create Command
      - [ ] PING
      - [ ] ECHO
      - [ ] GET
      - [ ] SET
    - [ ] Create CommandManager (or just a function of it)
    - [ ] Incorporate
  - [ ] Implement Memory
    - [ ] Create Memory
    - [ ] Create MemoryManager (add MPSC + task)
    - [ ] Incorporate
  - [ ] Implement Response
    - [ ] Create Response
    - [ ] Incorporate
    

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
    

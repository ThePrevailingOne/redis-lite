# High Level Design

## Overview
### Initial Target 0.1.0
To aim for a Minimum Viable Product (MVP), this redis server should be able to:
- Handle multiple client connection
- Handle SET & GET

Memory data types that can be stored will also be limited to:
- key: string
- value: string

To allow the server to work with redis-cli client, we will adapt with the format of RESP. To accomodate most needs of the command, the main data types supported for MVP are:
- Simple string
- Simple Errors
- Integers
- Bulk strings
- Arrays
- Nulls

### Target 0.1.1
In the next milestone, the target will be aimed at handling other data types:
- key: string, int
- value: string, int

### Target 0.2.0
In the next milestone, the target will be ainmed at persistence handling, which can be broken down to:
- usage of write-ahead log (WAL) append-only file (AOF); along with
- periodic in-memory storage snapshot flush to disk

### Target 0.3.0
In the next milestone, the target will revolve around transactions. specific targets TBD.



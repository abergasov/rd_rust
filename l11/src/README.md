project structure:

```lua
pkg/
|-- logger # AppLogger is abstract trait for app logger. It can be replaced under the hood with any logger implementation.
|-- service
|-- | -- client # client handler function for parse messages from tcp connection
|-- | -- server # server handler function for parse messages from tcp connection
|-- utils # Common utilities
clietn.rs
server.rs
```
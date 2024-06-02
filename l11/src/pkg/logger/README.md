AppLogger is abstract trait for app logger. It can be replaced under the hood with any logger implementation.

Vendor free interface which allow multiple writers to write logs in the same time.

```rust
pub trait AppLogger {
    fn info(&self, message: &str, args: &[StringWith]);
    fn error(&self, message: &str, err: Error, args: &[StringWith]);
    fn fatal(&self, message: &str, err: Error, args: &[StringWith]);
    fn with(&self, args: &[StringWith]) -> Box<dyn AppLogger + Send + Sync>;
}
```

Sample implementation of AppLogger is provided in `logger` package.

```rust
pub struct SimpleLogger {
    context: Vec<StringWith>,
}
```

Currents implementation is not best for performance and production usage, but it's good for development and debugging.

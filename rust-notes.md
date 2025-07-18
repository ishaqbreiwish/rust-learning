# Rust Learning

## Ownership

- each vairable can only have one owner at a time, and when that owner goes out of scope tha value is dropped
- you can get around this with `.clone()` which creates a deep copy
  - however if you do `let b = a` then you cannot now reference a because the value at a is now owned by b
- you can make variables equal to references to other vars using `&` or `&mut`

  - `&` is immutable and thus you cannot change the value only view it
  - `&mut` is mutable and allows you to diretly modify the value you are referencing

    - if you reference a var with one type of reference you cannot reference it with another type, ie.:
      ```Rust
      let a = &name
      let b = &mut name // will not compile
      ```
    - however due to NLL, if a is not used again after the declaration of b then it will compile, since a is dropped if its not used

      ```Rust
      let a = &name
      println!("{}", a)

      let b = &mut name // will compile
      ```

## Enums & Matching

- enums let you define types that could be one of several variants
  - each variant can hold data
  - enums must be capitalized
- you use `match` to destructure and handle each case

  ```Rust
  // example
  enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }

  fn handle_message(msg: Message) {
      match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
      }
  }
  ```

## Async/Await

- asyncs allow you to let other tasks to run so that slower tasks don't end up being blockers on your whole program
  - Futures/Promises – Represent a value that’s not ready yet
  - await – Says "pause here until that value is ready."
  - async fn – Marks a function that returns a future instead of an immediate result
- async fn returns a future (impl Future)
- `.await` yields control until the future completes
- `tokio::main` macro sets up the async runtime
- you can run multiple tasks concurrently using `tokio::spawn` or `.join!`

## Implementations (Impl)

- it’s where you define how methods or traits work for a struct or enum

  ```Rust
      struct Circle {
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            3.14 * self.radius * self.radius
        }
    }
  ```

## Vocabulary

- **Non-Lexical Lifetimes (NLL)**: A variable’s lifetime ends when it is no longer used, not necessarily when its scope ends

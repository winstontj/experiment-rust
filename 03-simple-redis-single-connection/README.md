# Simple Redis - Single Connection

Learning to use the Tokio Runtime.

This part of the code is the result of the following tutorial
https://tokio.rs/tokio/tutorial/spawning

## How to Use

Prerequisite
- Make sure that no program is running at port `6379`

Using this directory as the working directory, 
- Open a terminal, use `cargo run` to build and start the simple redis server
- Open a different terminal, using the example provided run the command `cargo run --example hello-redis`
- The server terminal will display

```
Socket GOT Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world")])
Socket GOT Array([Bulk(b"get"), Bulk(b"hello")])
```

- The client terminal will display

```
Get the value from server result=Some(b"world")
```

## Lesson Learned

- Async process in Rust is not supported by default. It needs a special runtime carts that can translate code into asynchronous function.
- The concept of concurrency and parallel processing. Its been a while since I am thinking about this. Coming from JS world, there is no such thing as parallel processing. Concurrency is there, but it is always a single thread in the processor that process the data.
- Learning the concept of ownership in Rust (well, I should have learned it from the book, but hey, I learned it the hard way :p )
- Still not used to the whole situation with unwrap() and just await, and why the question mark in the await???? :thinking_face:

## Next Step

Follow the next tutorial and get a better understanding on how tokio runtime works.

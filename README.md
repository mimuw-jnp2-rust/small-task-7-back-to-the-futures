# Small task 7 - Back to the futures

![](meme.jpg)

This week's assignment will test your understanding of async.

Your goal is to implement two future combinator functions:
- `flat_map` (takes a future and a function which returns a future; returns a future which is the result of applying the function to the value of the first future)
- `join` (takes two futures and returns a future which yields the values of both futures)

We provide the correct implementation of `map` which you can take as an example.

You will have to use the `pin_project` macro as we disabled the ability to write `unsafe` code. You should read [its documentation](https://docs.rs/pin-project/latest/pin_project/index.html) to understand how it works. Reading the [`pin` module documentation](https://doc.rust-lang.org/nightly/core/pin/index.html#) is also a good idea, especially from [this part](https://doc.rust-lang.org/nightly/core/pin/index.html#projections-and-structural-pinning) to the end.

This time there are two test files. You are free to complete only one of them.

You must not use any external libraries for this task (other than `pin_project`).
You must not modify the tests (obviously).

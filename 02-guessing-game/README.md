# Guessing Game

A little program that read user input, compare it to a pre-generated data then ends the program once the inputted data is equal to the pre-genereated data.
Then print congratulations message with the number of tries the user did.

Source: Rust Book
https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

## Lesson learned

- how to add package. Use `cargo add <package name>` for normal development flow.
- The `match` keyword is a good one to replace excessive if else or humongous try catch block
- The enum in rust looks interesting
- using other library like randomizer, comparing data, and parsing string

## Sample output

```
Try to guess a number, if you can. :p!
Input a number between 1-100
55
You guessed 55. Number of guess: 1
Its too small, aim larger.

Input a number between 1-100
88
You guessed 88. Number of guess: 2
Too big, aim smaller.

Input a number between 1-100
66
You guessed 66. Number of guess: 3
Its too small, aim larger.

Input a number between 1-100
77
You guessed 77. Number of guess: 4
Too big, aim smaller.

Input a number between 1-100
75
You guessed 75. Number of guess: 5
Too big, aim smaller.

Input a number between 1-100
70
You guessed 70. Number of guess: 6
Its too small, aim larger.

Input a number between 1-100
73
You guessed 73. Number of guess: 7
Its too small, aim larger.

Input a number between 1-100
76
You guessed 76. Number of guess: 8
Too big, aim smaller.

Input a number between 1-100
75
You guessed 75. Number of guess: 9
Too big, aim smaller.

Input a number between 1-100
74
You guessed 74. Number of guess: 10
Yess you got it.

CONGRATULATIONS. You completed in 10 tries.
```

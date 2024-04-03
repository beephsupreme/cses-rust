# Chessboards and Queens

#### [Source Code](https://github.com/beephsupreme/cses-rust/blob/master/src/solutions/chessboard_and_queens.rs)

## Description

Your task is to place eight queens on a chessboard so that no two queens are attacking each other. As an additional
challenge, each square is either free or reserved, and you can only place queens on the free squares. However, the
reserved squares do not prevent queens from attacking each other.

How many possible ways are there to place the queens?

## Input

The input has eight lines, and each of them has eight characters. Each square is either free (.) or reserved (*).

## Output

Print one integer: the number of ways you can place the queens.

## Examples

Input

```
........
........
..*.....
........
........
.....**.
...*....
........
```

Output

```
65
```

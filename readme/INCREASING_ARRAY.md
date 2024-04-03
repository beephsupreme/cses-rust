# Increasing Array

#### [Source Code](https://github.com/beephsupreme/cses-rust/blob/master/src/solutions/increasing_array.rs)

## Description

You are given an array of `n` integers. You want to modify the array so that it is increasing, i.e., every element is at
least as large as the previous element.
On each move, you may increase the value of any element by one. What is the minimum number of moves required?

## Input

The first input line contains an integer `n`: the size of the array.

Then, the second line contains `n` integers `x_1 ,x_2 , ... , x_n`: the contents of the array.

## Output

Print the minimum number of moves.

## Constraints

`1 ≤ n ≤ 2e5`

`1 ≤ x_i ≤ 1e9`

## Examples

Input

```
5  
3 2 5 1 7
```

Output  
`5`

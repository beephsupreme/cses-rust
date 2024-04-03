# Apple Division

#### [Source Code](https://github.com/beephsupreme/cses-rust/blob/master/src/solutions/apple_division.rs)

## Description

There are `n` apples with known weights. Your task is to divide the apples into two groups so that the difference
between
the weights of the groups is minimal.

## Input

The first input line has an integer `n`: the number of apples.
The next line has `n` integers `p_1,p_2, ...,p_n`: the weight of each apple.

## Output

Print one integer: the minimum difference between the weights of the groups.

## Constraints

```
1 ≤ n ≤ 20
1 ≤ p_i ≤ 1e9
```

## Examples

Input

```
5
3 2 7 4 1
```

Output  
`1`

Explanation: Group 1 has weights 2, 3 and 4 (total weight 9), and group 2 has weights 1 and 7 (total weight 8).

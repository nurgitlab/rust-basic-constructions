# Basic constructions for Rust
Repository for basic constructs for working with Rust


## **Vartical table of methods Vec<T>**

| Command | Description |
| --- | --- |
| `vec.push(value)` | add the element to end |
| `vec.pop()` | remove last element |
| `vec.len()` | lenght of structure |
| `vec.capacity(value)` | total capacity |
| `vec.contains(&x)` | is element contains |
| `vec.sort()` | sort |
| `vec.iter()` | iterator |

## How to check word[i]=='a'

```rust
let c = word.chars().nth(i).unwrap();

if (c == 'a') {
    //logic
}
```

## Create and fill vector

```rust
let mut ans: Vec<i32> = Vec::with_capacity(l * 2);

ans.resize(l * 2, 0);
```
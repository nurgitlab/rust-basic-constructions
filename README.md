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

### How to check word[i]=='a'

```rust
let c = word.chars().nth(i).unwrap();

if (c == 'a') {
    //logic
}
```

### Create and fill vector

```rust
let mut ans: Vec<i32> = Vec::with_capacity(l * 2);

ans.resize(l * 2, 0);
```

### For from 10 to 1

```rust
for i in (0..10).rev() {
    right_sum[i] = c2;
    c2 += nums[i];
}
```

## **Hashmap/Structures**

## How to sort array

```rust
let mut numbers = vec![9, 8, 7, 10, 4];
numbers.sort_by(|a, b|a.cmp(b));
println!("{:?}", numbers);
```

## How to change elements in line (12.34 -> 12[.]34)

```rust
    address.replace(".", "[.]")
```

## HashSet
So, here's example of creating HashSet and check, is element contain in.

```rust
use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let set: HashSet<char> = jewels.chars().collect();
        let mut ans = 0;
        for i in 0..stones.len() {
            let c = stones.chars().nth(i).unwrap();

            if set.contains(&c) {
                ans+=1;
            }
        }

        return ans;
    }
}
```

## HashMap

```rust
use std::collections::HashMap;

fn main() {
    // 1. Create
    let mut students = HashMap::new();

    // 2. Add
    students.insert("Alex".to_string(), 20); // String as key
    students.insert("Marya".to_string(), 19);
    students.insert("Siegfried".to_string(), 21);

    // 3. Get (Option<&i32>)
    println!("Age of Alex: {:?}", students.get("Alex")); // Some(20)
    println!(": {:?}", students.get("Petr")); // None

    // 4. Change value
    students.insert("Alex".to_string(), 21); // Rewrite

    // 5. Delete
    students.remove("Siegfried");
    println!("After remove: {:?}", students);

    // 6. Is key exist
    println!("Is Marya in Map? {}", students.contains_key("Marya")); // true
}
```

TODO
Rewrite methods by strings/arrays/chars and etc in local MD files.
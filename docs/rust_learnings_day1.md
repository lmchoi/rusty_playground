# Rust Learning Summary

Here's a comprehensive summary of all the key Rust concepts we've covered, organized by your questions with relevant code snippets:

## 1. Basic Array Operations

### Finding max in array:
```rust
let nums = [1, 5, 3];
let max = *nums.iter().max().unwrap_or(&0); // 5
```

### Dropping first/last elements:
```rust
let arr = [1, 2, 3];
let no_first = &arr[1..];    // [2, 3]
let no_last = &arr[..arr.len()-1]; // [1, 2]
```

### Reversing:
```rust
let mut vec = vec![1, 2, 3];
vec.reverse(); // [3, 2, 1]
```

## 2. Sorting Techniques

### Basic sort:
```rust
let mut nums = [3, 1, 2];
nums.sort(); // [1, 2, 3]
```

### Sort with indices:
```rust
let mut indexed: Vec<_> = nums.iter().enumerate().collect();
indexed.sort_by_key(|&(i, &v)| v); // Sorts by value, keeps original indices
```

### Sort key-value pairs:
```rust
let mut kv = [("b", 2), ("a", 1)];
kv.sort_by_key(|&(k, _)| k); // Sorts by key
```

## 3. Option/Result Handling

### Unwrapping safely:
```rust
let maybe_num: Option<i32> = Some(5);
let num = maybe_num.unwrap_or(0); // 5
```

### Early returns:
```rust
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { return Err("Division by zero".into()); }
    Ok(a / b)
}
```

## 4. Iterator Patterns

### Summing values:
```rust
let sum: i32 = kv_pairs.iter().map(|(_, v)| v).sum();
```

### Finding top N values:
```rust
let top_three: Vec<_> = nums.iter().copied().take(3).collect();
```

## 5. Type System Fundamentals

### Array vs Slice:
```rust
let arr: [i32; 3] = [1, 2, 3]; // Fixed-size
let slice: &[i32] = &arr[..2];   // Dynamic view
```

## 6. Common Pitfalls Solved

### Safe lookahead:
```rust
let next = nums.get(index + 1).copied().unwrap_or_default();
```

### Parameterized tests (without deps):
```rust
#[test]
fn test_multiple_cases() {
    let cases = [(1,1,2), (2,3,5)];
    for (a, b, expected) in cases {
        assert_eq!(a + b, expected);
    }
}
```

## Key Concepts to Remember:

1. **Ownership** - Rust's core safety feature
2. **Borrowing** - Shared (&T) vs mutable (&mut T) references
3. **Lifetimes** - Ensuring references stay valid
4. **Pattern Matching** - Powerful control flow with `match`
5. **Error Handling** - Proper use of `Option`/`Result`

## Recommended Practice Path:

1. Master basic ownership/borrowing
2. Learn common iterator methods
3. Practice error handling patterns
4. Explore trait system and generics
5. Build small projects (CLI tools are great starters)

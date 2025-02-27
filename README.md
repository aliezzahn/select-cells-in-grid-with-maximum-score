# Select Cells in Grid with Maximum Score

A Rust library to solve the problem of selecting cells in a grid to maximize the score.

![CI](https://github.com/aliezzahn/select-cells-in-grid-with-maximum-score/actions/workflows/ci.yml/badge.svg)
![CD](https://github.com/aliezzahn/select-cells-in-grid-with-maximum-score/actions/workflows/cd.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Problem Description

Given a 2D grid of integers, the goal is to find the maximum score achievable by moving only right or down from the top-left corner to the bottom-right corner. The score is the sum of the values in the selected cells.

## Usage

Add this library to your `Cargo.toml`:

```toml
[dependencies]
select-cells-in-grid-with-maximum-score = "0.1"
```

Then, use the `max_score` function in your code:

```rust
use select_cells_in_grid_with_maximum_score::max_score;

fn main() {
    let grid = vec![
        vec![1, 2, 3],
        vec![4, 3, 2],
        vec![1, 1, 1],
    ];
    let result = max_score(grid);
    println!("Maximum score: {}", result); // Output: Maximum score: 8
}
```

## Examples

### Example 1

```rust
let grid = vec![
    vec![1, 2, 3],
    vec![4, 3, 2],
    vec![1, 1, 1],
];
assert_eq!(max_score(grid), 8);
```

### Example 2

```rust
let grid = vec![
    vec![8, 7, 6],
    vec![8, 3, 2],
];
assert_eq!(max_score(grid), 15);
```

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/aliezzahn/select-cells-in-grid-with-maximum-score).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Author

- **Ali Ezzahn**
  - Email: [aliezzahn@gmail.com](mailto:aliezzahn@gmail.com)
  - GitHub: [aliezzahn](https://github.com/aliezzahn)

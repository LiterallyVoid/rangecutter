# rangecutter

Utilities for working with adjacent ranges.

```rust
use std::ops::Range;
use rangecutter::RangeExt;

let source = "Line 1\nLine 2\n";

fn first_newline(s: &str) -> Option<Range<usize>> {
    let sep = '\n';

    let start = s.find(sep)?;
    let end = start + sep.len_utf8();

    Some(start..end)
}

let mut cursor = 0..source.len();

let lines_with_endings = std::iter::from_fn(|| {
    let separator = match first_newline(&source[cursor.clone()]) {
        Some(newline) => cursor.compose(&newline),
        None if !cursor.is_empty() => {
            cursor.end..cursor.end
        }
        None => return None,
    };

    let line;
    (line, cursor) = cursor.cut(&separator);

    let line_with_separator = line.concat(&separator);

    Some(&source[line_with_separator])
}).collect::<Vec<_>>();

assert_eq!(
    lines_with_endings,
    [
        "Line 1\n",
        "Line 2\n",
    ],
);

```

## Limitations

Only fully-bounded inclusive ranges (`Range<T>`) are supported.

License: MIT

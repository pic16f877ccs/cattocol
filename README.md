# cattocol

### Combine two text into one text as columns or by lines.

- Without the ansi escpe sequences.
- With the ansi escpe sequences.

### Examples

```rust

use cattocol::CatToCol;

let txt_one = String::from("Text cat\nby line.\nTest line.");
let txt_two = String::from("Concat text.\nTwo line.\nMin.\nMax");
let cat_to_col = CatToCol::new().fill(' ').repeat(1);
let combine_iter = cat_to_col.combine_col(&txt_one, &txt_two);

println!("{}", combine_iter.collect::<String>());

//Text cat   Concat text.
//by line.   Two line.
//Test line. Min.
//           Max
```

## License
GNU General Public License v3.0 


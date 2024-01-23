# to_bopomofo

Converts a pinyin string to a Bopomofo string

### example

```rust
use to_bopomofo::to_bopomofo;
let bopomofo1 = to_bopomofo("ni");
println!("bopomofo:{bopomofo1:?}"); // bopomofo:Some("ㄋㄧ")
let bopomofo2 = to_bopomofo("wrong pinyin");
println!("bopomofo:{bopomofo2:?}"); // bopomofo:None
```
# to_bopomofo

Converts a pinyin string to a Bopomofo string

### install

```
cargo add to_bopomofo
```

### use

```rust
use to_bopomofo::to_bopomofo;

fn main() {
    let bopomofo1 = to_bopomofo("ni");
    println!("bopomofo:{bopomofo1:?}"); // bopomofo:Some("ㄋㄧ")
    let bopomofo2 = to_bopomofo("wrong pinyin");
    println!("bopomofo:{bopomofo2:?}"); // bopomofo:None
}

```

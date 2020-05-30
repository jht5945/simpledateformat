# simpledateformat

SimpleDateFormat.java style like date format


Usage:

```rust
let f = match fmt("yyyy-MM-dd HH:mm:ss z") {
    Ok(f) => f, Err(err) => {
        println!("Parse fmt error: {}", err);
        return;
    },
};
println!("Formated date: {}", f.format(&Local::now()));
```

Output:
```
Formated date: 2020-05-30 13:32:04 +08:00
```



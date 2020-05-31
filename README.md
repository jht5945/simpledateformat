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


```rust
format_human(Duration::from_secs(2 * 24 * 60 * 60 + 1));
```

Output:
```
2days 0hour 0min 1s
```



Learning embedded development in rust with the [discovery book](https://rust-embedded.github.io/discovery).


## Running

1. Establish connection with ST-LINK through `openocd` in a new terminal:
```
$ cd /tmp
$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

2. Start gdb session and run target binary on the F3 with:
```
$ cargo run --bin {{binary to run}}
```


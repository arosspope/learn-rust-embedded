
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

## gdb

1. Start gdb, where {gdb} could be one of `[arm-none-eabi-gdb, gdb-multiarch, gdb]`:
```
$ {gdb} -q target/thumbv7em-none-eabihf/debug/led-roulette
```

2. Connect to the OpenOCD GDB server:
```
(gdb) target remote :3333
Remote debugging using :3333
0x00000000 in ?? ()
```

3. Load the built elf:
```
(gdb) load
Loading section .vector_table, size 0x188 lma 0x8000000
Loading section .text, size 0x38a lma 0x8000188
Loading section .rodata, size 0x8 lma 0x8000514
Start address 0x8000188, load size 1306
Transfer rate: 6 KB/sec, 435 bytes/write.
```

4. Set break point at main:
```
(gdb) break main
Breakpoint 1 at 0x800018c: file src/05-led-roulette/src/main.rs, line 10.
```

5. Enter/Exit/Quit GDB's Text User Interface (TUI):
```
(gdb) layout src
...
(gdb) tui disable
...
(gdb) quit
```

## itmdump

1. Make sure openocd + itmpdump are both running in the same directory:
```
/tmp$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
...
/tmp$ itmdump -F -f itm.txt
```
2. Instruct OpenOCD to redirect the ITM output into the same file that itmdump is watching:
```
(gdb) # globally enable the ITM and redirect all output to itm.txt
(gdb) monitor tpiu config internal itm.txt uart off 8000000

(gdb) # enable the ITM port 0
(gdb) monitor itm port 0 on
```

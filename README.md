# Traditional astrology for rust
Rust library by Stéphane Bressani (s.bressani@bluewin.ch)

Using swissephem c library by Astrodienst AG by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)

The source code is released under an Creative Commons License, which allows it to be used also on commercial projects. This software uses the swiss ephemeris which is licensed GPL.

Therefore, if you want to use libswe-sys in your commercial projects, you must adhere to the GPL license or buy a Swiss Ephemeris commercial license.

# Use

Go to exemple sand rename data_example.json to data.json and run cargo run
--example debug

# Version
0.1.12
* Adding stdio.h

0.1.11
* Adding lib (compile)

0.1.10
* Adding missing include (yanked)

0.1.9
* Try math.h again include a path

0.1.8
* Try math.h again with include a path (yanked)

0.1.7
* Add math.h found on internet https://github.com/KnightOS/libc/blob/master/include/math.h


0.1.6
* Add standard path
  https://stackoverflow.com/questions/60188673/rust-ffi-wasm-yew-cargo-web-start-fatal-error-math-h-file-not-found
  for include (math.h)

0.1.5
* Add manually math.h from /Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk/usr/include/math.h on my mac

0.1.4
* Add -lm flag for math.h in compilation of wasm (not compile)

0.1.3
* Add clone macro to missing structures

0.1.2
* Add clone macro

0.1.1
* Change license to Creative Commons

0.1.0
* Some function of the lib bridge from c to rust

# Traditional astrology for rust
Rust library by Stéphane Bressani (s.bressani@bluewin.ch)

Using swissephem c library by Astrodienst AG by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)

The source code is released under an Creative Commons License, which allows it to be used also on commercial projects. This software uses the swiss ephemeris which is licensed GPL.

Therefore, if you want to use libswe-sys in your commercial projects, you must adhere to the GPL license or buy a Swiss Ephemeris commercial license.

# Use

Go to exemple and rename data_example.json to data.json and run cargo run
--example debug

I recommand to use the library crate "astrology" for draw the chart

# Version
0.1.58
* Add a crate for enum match (Theme) in crate astrology/libastro

0.1.57
* Add a crate for enum match use in crate libastro

0.1.56
* It's compile this time, 0.1.55 is yanked

0.1.55
* Add another path /usr/local/opt/llvm/include

0.1.54
* Add another path for osx build wasm /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include

0.1.53
* Add another path for osx insade /usr/include -> /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include

0.1.52
* Add /usr/include for the math.h and other c header

0.1.51
* Support for mac os catalina with good path llvm this time (brew install llvm)

0.1.50
* Support wasm for mac os catalina with the path
/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin

0.1.49
* Support wasm for mac os

0.1.48
* Add theme support for "astrology" crate bridge

0.1.47
* Add comments to sweconst for rust documentation

0.1.46
* Add text english/french for aspects

0.1.44 and 0.1.45
* Add documentation to cargo
* Add text english/french for signs

0.1.43
* Fix fortuna part

0.1.42
* Fix little bug in fortuna part

0.1.41
* Fix unwrap (bug negative sign)

0.1.40
* Fix fortuna part

0.1.39
* Fix fortuna part

0.1.38
* Add algorythme for fortuna part in swerust03

0.1.37
* Fix aspect in sweconst

0.1.36
* Add minors aspects in sweconst

0.1.35
* Add function for Aspects in sweconst

0.1.34
* Add serialize to Aspects in sweconst

0.1.33
* Bug fix in majors aspects

0.1.32
* Add majors aspects

0.1.31
* Add compute for south node

0.1.30
* Add all swissephem files for all datas missing like chiron, ceres etc...

0.1.29
* Fix color for render on with background

0.1.28
* Add a method in Bodie struct in sweconst

0.1.27
* Add bodies colors in sweconst

0.1.26
* Change earth element color to orange/yellow

0.1.25
* Add hexadecimal color for zodiac, pub enum Element in sweconst

0.1.24
* Add Copy macro to Angle in sweconst

0.1.23
* Add enum iter to Angle in sweconst

0.1.22
* Add object position (stationary/direct/retrograde) to Object

0.1.21
* Field set to public in swerust17 for split deg/min/sec of a planet

0.1.20
* Add PartialEq macro to Object struct in sweconst

0.1.19
* Add enum to Object structure in sweconst

0.1.18
* Add PartialEq macro to Angle in sweconst

0.1.17
* Add one enum for angle in house (Asc/Fc/Desc/Mc/Nothing)

0.1.16
* Change in exemple, add whole signs ..°..'.."
* Fields objects in sweconst are public now

0.1.15
* Small change in library, fields for house are public now

0.1.14
* Small change in library, fields for house are public now

0.1.13
* Remove lib.h (try with wasm32-wasi)

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

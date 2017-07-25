Rust Curses
===========

An example of ncurses in Rust using [pancurses](https://github.com/ihalila/pancurses).  

This example is a rudimentary map of my shared Engineering office space at [Azusa Pacific University](https://apu.edu).  In this example, you simply walk around and talk to people.

## Dependencies

To compile and run from source, you must have rust and cargo installed.

Additionally, since this requries ncurses, you must have both ncurses and the ncurses headers installed.  For example, on Fedora:

`$ sudo dnf install rust cargo ncurses ncurses-devel`

## How to run

To run this, simply call:

`$ cargo run`

## Building (optional)

Everything is done via cargo.  Manually building is only helpful for packaging or compile testing.  To build:

`$ cargo build`

This will fetch all dependencies and build the binary under target/debug.  The binary that is produced can be ran natively on any similar environment (same ncurses version, etc.) even if rust is not installed.

## Code comments

This was more a learning exercize for Rust than for ncurses.  There are two structs identified - one is a generic "Point" for a simple x,y coordinate for plotting in ncurses.  The second, "WinRef" is used to identify what kind of object is represented at a coordinate.

These are then organized into a HashMap that correlates Point instances to WinRefs.  rtypes of "furn" indicate furniture and other rtypes indicate office people.  All furniture plots are organized into a vector containing all the points for a common character.  Office people are organized into a separate hashmap that containes some phrase for each person.

When the cursor moves, the code first does a boundary check to make sure that the cursor won't go out of bounds or run into an object.  If a user would run into an object, the code identifies if the object is a person, and if it is, it attempts to lookup the phrase for the person, which it then displays at the top of the screen.

The code is rudimentary and could certainly be refactored to be more elegant.  For example, pancurses has the ability to plot lines of characters rather than needing to specify every character by explicit plot.  Further, the input loop has common, repeated logic for directional keys that could be refactored into helper functions.

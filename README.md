# shadowhand

Dead simple desktop automation tool. Written in Rust.  
It takes an instruction text file (see instruction_example file) and executes it.  

To build:  
`cargo build --release`

Usage:
`shadowhand instruction_file`



Runtime dependencies
--------------------

Linux users may have to install libxdo-dev. For example, on Ubuntu:

```Bash
apt install libxdo-dev
```
On Arch: 

```Bash
pacman -S xdotool
```

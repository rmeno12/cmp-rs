# cmp-rs

## How to use
`cmp-rs -i <input_filename> -o <output_filename>` where `<input_filename>` is the name of an ASCII text file.

## File structure
```
32 bit integer for number of encodings
8 bit integer for length of encoding
8 bit integer for char value
. bits for encoding
repeat for all encodings
. bits for encoded value
```

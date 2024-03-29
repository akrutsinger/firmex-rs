# firmex-rs
Firmware parsing library. Could be used to extract contents of the file and save to disk. I don't clain to be good at naming things.

## Limitations
Made this to help try and learn rust. It only supports two types of firmware containers, which are from the Tesla Wall Connector that I've been slowly [reverse engineering](https://akrutsinger.github.io/)

## Examples
* `cargo run --example extract-sbfh --  --file-path <filename>` - extracts the contents of the SBFH firmware container and saves it to a poorly named file.
* `cargo run --example parse-mrvl -- --file-path <filename> <path to resutling dir>` - reads a Marvell firmware file and prints some of the file header and segment header information.

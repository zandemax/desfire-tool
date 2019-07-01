# desfire-tool
Command Line Tool for working with MIFARE DesFire Cards

This is a Command Line tool for personalizing, reading and writing MIFARE DESFire (EV1/EV2) Cards.
It is written in Rust and based on libnfc and libfreefare.

## Installation

### Requirements

For this tool to work, you have to have [libnfc|https://github.com/nfc-tools/libnfc] and [libfreefare|https://github.com/nfc-tools/libfreefare] installed.
There are binary packages available for many platforms, consult the documentation of these packages for further info.

### Installation

coming soon

## Usage

To use this tool, you first need a supported reader. For a list of supported readers, visit [this wiki page|http://nfc-tools.org/index.php/Devices_compatibility_matrix].

As a start, place your tag on your reader and run

```bash
desfire-tool card info
```

You can display a list of commands by running 

```bash
desfire-tool help
```

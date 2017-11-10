# cli-for-zermelo [![Build Status](https://travis-ci.org/splintah/cli-for-zermelo.svg?branch=master)](https://travis-ci.org/splintah/cli-for-zermelo)
A command line application written in Rust that retrieves today's schedule from Zermelo.

# Installation
[Install Rust](https://rustup.rs). Then run the following in your terminal.

```bash
$ git clone https://github.com/splintah/cli-for-zermelo.git
$ cd cli-for-zermelo
$ cargo build --release
```

The executable is now located in `target/release/`.
You may want to move this executable into a folder from your path.

# Usage
There are two ways to use this program:
- With a config file: [Config](#config).
- Without a config file, with command line flags: [Flags](#flags).

The recommended way is to use a config file.

# Config
Create a config toml file somewhere, for example: `~/.cli-for-zermelo.toml`.
This file will contain your settings for this program.

When you first want to authenticate, write the following in your config file, replacing your school and authentication code to the values found in the Zermelo Portal (Koppelingen -> Koppel App):
```toml
school = "your-school"

[temp]
auth_code = "123 456 689 012"
```

Now run the program with the flag `--config [your-config]`, replacing "[your-config]" with the path to yout config file.
This will get an access token, and it automatically sets it in your config file.
Your config file will look something like the following now:
```toml
school = "your-school"
access_token = "abcdefghijklmnopqrstuvwxyz"
```

When you want to run the program again, run it with the flag `--config [your-config]` again.
You may want to set an alias to this.

# Flags
- authenticate:
    - short: `-u`
    - long: `--auth`
    - value: string
    - Authenticate with your code found in the Zermelo Portal (Koppelingen -> Koppel App). School has to be set. When you include spaces, wrap the code inside double brackets.
- school:
    - short: `-s`
    - long: `--school`
    - value: string
    - The school identifier found in the Zermelo Portal (Koppelingen -> Koppel App).
- access_token:
    - short: `-a`
    - long: `--accesstoken`
    - value: string
    - The access token retrieved with your authentication code.
- config:
    - short: `-c`
    - long: `--config`
    - value: path
    - The location of the config file.

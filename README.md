# rcov
Test case coverage enforcer for rust using tarpaulin.

**Pre-requisites:**
Tarpaulin depends on cargo which depends on SSL. Make sure you've installed your distros SSL development libraries and they are on your path before attempting to install tarpaulin. For example for Debian/Ubuntu:
    > apt-get update && apt-get install libssl-dev pkg-config cmake zlib1g-dev
    > https://github.com/xd009642/tarpaulin

Install tarpaulin using command - 
    > cargo install cargo-tarpaulin

**Build rcov release**
Build rcov using command - cargo build release

**rcov installation**
Install rcov into cargo's bin directory using command - cargo install --path=<PATH to rcov's root folder>

To use rcov, navigage to the root folder of rust project and run command - rcov

You can pass the parameters to tarpaulin as an input to rcov. e.g. rcov tarpaulin -v

Default minimum test case coverage percentage is 80. This can be overriden using '-m' flag.

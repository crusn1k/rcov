# rcov
Test case coverage enforcer for rust using tarpaulin.

** Pre-requisites: **
Install tarpaulin using command - cargo install cargo-tarpaulin

** Build rcov release **
Build rcov using command - cargo build release

** rcov installation **
Install rcov into cargo's bin directory using command - cargo install --path=<PATH to rcov's root folder>

To use rcov, navigage to the root folder of rust project and run command - rcov

You can pass the parameters to tarpaulin as input to rcov. e.g. rcov -v

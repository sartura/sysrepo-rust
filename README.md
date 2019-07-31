# sysrepo-rust
The goal of this application is to provide a simple example describing the use of Sysrepo C libraries with the Rust language.
As in other Rust projects that use C libraries a wrapper "wrapper.h" file is used to include all the required C headers. The wrapper header is then used by bindgen to generate Rust bindings from the header from build.rs.

The example application is based on the Sysrepo sr_get_item example.

# Installation
Make sure that rustc and cargo are installed, then run cargo build. 
Once the project has finished building it can be started with cargo run.

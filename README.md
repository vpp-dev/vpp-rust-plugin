# vpp-rust-plugin
An experiment with making a VPP plugin in Rust.
This is a very early stage code, and mostly for exploring this myself,
soliciting some interest in the community, and learning myself
some Rust in the process :-)

Hit me up on twitter at @ayourtch if you have any questions, comments,
feedback or a genius idea.

Right now the code does not do a whole lot.

Just says hello when called from the CLI it registers.

# Compiling

"cargo build" will build the file in target/debug/librust_plugin.so

You will need to copy this file manually into the plugin directory
of your VPP instance.






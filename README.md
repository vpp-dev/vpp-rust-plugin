# vpp-rust-plugin
This is an experiment with making a VPP plugin in Rust.

It is a very early stage code, and mostly for exploring this myself,
soliciting some interest in the community, and learning myself
some Rust in the process :-)

Hit me up on twitter at @ayourtch if you have any questions, comments,
feedback or a genius idea.

Right now the code does not do a whole lot.

Just says hello when called from the CLI it registers.

Also, you will notice that some of the types are not exactly
(or not at all) the same as in VPP proper - these are shortcuts,
which may be fixed later on. Or not.

# Compiling

After installing rust (https://www.rust-lang.org/en-US/install.html),
"cargo build" will build the file in target/debug/librust_plugin.so

You will need to copy this file manually into the plugin directory
of your VPP instance.

Assuming the "vpp" tree is in your home directory, it will look like this:

cp target/debug/librust_plugin.so ~/vpp/build-root/install-vpp_debug-native/vpp/lib64/vpp_plugins/








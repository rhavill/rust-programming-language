/*
As you add more integration tests, you might want to make more than one file in 
the tests directory to help organize them; for example, you can group the test 
functions by the functionality they’re testing. As mentioned earlier, each file 
in the tests directory is compiled as its own separate crate.

Treating each integration test file as its own crate is useful to create 
separate scopes that are more like the way end users will be using your crate. 
However, this means files in the tests directory don’t share the same behavior 
as files in src do, as you learned in Chapter 7 regarding how to separate code 
into modules and files.

The different behavior of files in the tests directory is most noticeable when 
you have a set of helper functions that would be useful in multiple integration 
test files and you try to follow the steps in the “Separating Modules into 
Different Files” section of Chapter 7 to extract them into a common module. For 
example, if we create tests/common.rs and place a function named setup in it, 
we can add some code to setup that we want to call from multiple test functions 
in multiple test files:

When we run the tests again, we’ll see a new section in the test output for the 
common.rs file, even though this file doesn’t contain any test functions nor did 
we call the setup function from anywhere.

To avoid having common appear in the test output, instead of creating 
tests/common.rs, we’ll create tests/common/mod.rs. This is an alternate naming 
convention that Rust also understands. Naming the file this way tells Rust not 
to treat the common module as an integration test file. When we move the setup 
function code into tests/common/mod.rs and delete the tests/common.rs file, the 
section in the test output will no longer appear. Files in subdirectories of the 
tests directory don’t get compiled as separate crates or have sections in the 
test output.

After we’ve created tests/common/mod.rs, we can use it from any of the 
integration test files as a module. 

*/

pub fn setup() {
    // setup code specific to your library's tests would go here
}
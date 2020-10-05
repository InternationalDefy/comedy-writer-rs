# What is this?
A random sentances generator writter in Rust. With the inputs sentances template and power of it to construct a ComedyWriter class, it generates sentances with random words in random order. We all know that random words in a regular article is funny, that's why our program is called comedy-writer.
# Where are we?
Currently we have a pretty Alpha version of this.Firstly, we only have a SimplifedChinese dictionary, so all the sentances are generated in SimplifiedChinese.Secondly, the sentances template needs to be hard-coded so you can only use it by changing generator template in main.rs. 
# How to use?
(Temp)Open main.rs and edit **fn main()**, to instantiate a ComedyWriter class and add nodes{sentance template to it}
Sentance Template:
construct with macro **sentance!([meta])**
meta : [element = "WordClass"][word = "SomeWord"][icon = 'SomeIcon']
# What's in future?
1. More sentances template to be implemented with inputs of direction and output of a whole article.
2. Implement ComedyWriter class to a crate.
3. Implement a .io interface, and use comedy-writer-rs as a backend server.

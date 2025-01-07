# My Workspace for LeetCode Problems

I like to write LeetCode solutions in Rust, but because the language is still tricky I kinda need rust-analyzer while writing. LeetCode's online editor only has syntax highlighting, not inline errors and hints, so I prefer to work in VSCode. 

Creating a new crate every time I work on a new problem is annoying though, and so many direnv caches will probably fill up a lot of space (idrk tho, love u anyways direnv <3).

So anyways this crate contains all problems I've worked on in the ```problems``` module. To test a solution with ID ```n```, use ```cargo run -- -i n```.

Every problem is it's own file with at least 2 elements: a ```Solution``` struct with one function, who's name and signature are stated in the problem on [leetcode.com](https://leetcode.com), and a ```run``` function with the signature ```fn()```. 

The ```Solution```'s ```impl``` block can be copy pasted directly into LeetCode for submission, and it is all that you need to submit. The ```run``` function runs the example cases, each in an ```assert_eq!()``` statement, with a call to the solution's function and the expectd result. These are found in the problem description, make sure you check your tests are the exact same as the examples, or you're going to be debugging problems you've already solved correctly. 

To hook everything together when adding a problem, add the file as a public module in ```src/problems/mod.rs```. Then insert it into the ```hm``` hashmap in the ```main``` function with it's ID and a function pointer to the problem's ```run``` function. I'd like to automate this with a macro soon, but the ```paste``` crate doesn't let me concat things like ```[<problems::id $id ::run>]```. May require refactoring. 
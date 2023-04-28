# LRU Cache in Rust

### Motivation

A standard implementation of an LRU cache is written with a doubly-linked list of Nodes 
(node contains, say, the key and the value and pointers to neighbouring nodes). A HashMap is used for mapping from keys to 
the nodes within the linked  list. When a key is used, the node is moved to the front in O(1).

Writing a doubly-linked list in Rust is notoriously painful. Hence, I went with a really simple
approach which almost feels like cheating but works well in practice. 

_NOTE: Obviously, this is just a toy implementation._

### Approach

The approach is very similar to [LSM-Trees/Log-structured file system](https://en.wikipedia.org/wiki/Log-structured_file_system#:~:text=A%20log%2Dstructured%20filesystem%20is,in%201988%20by%20John%20K.).

_Aside: An LRU Cache, in this case, is an in-memory database so it's not really surprising 
that an approach used for databases can also be used for this._

In essence, for every `get` and `set` just append to a vector, leaving the previous indexes' 
contents as tombstones. Then, periodically run a clean up. In a real world implementation, 
this clean up would have been run in a background thread but in this implementation I run
it within `get` and `set` (once there are too many tombstones).

Algorithmic complexity is O(1) amortized.

### LinkedList Update to the above 

After writing the above sections, I implemented a doubly linked list LRU cache and it is actually not too bad. (Check out
the `src` folder for the code). However,
adding more functionality to it will be a pain (e.g. `peek`ing as returning a reference to something within a `RefCell` is not
very seamless) though we don't need it for LRU Cache. 

Interestingly, the linked list implementation is *not* faster in some experiments I ran and they have approximately
the same speed. (But, again, depends on many things such as the workload and what's the content of the
`value` as in real life you'd probably keep strings or other objects rather than just `i32` integers). 
The reason for this is probably cache locality of the vectors.

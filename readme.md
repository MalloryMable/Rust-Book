### Explanation
Each project/directory corosponds to one chapter of The Rust Book
Code inside of projects serve roughly as notes on what was covered in the chapter as well as accesible examples for later use.
###Note
Some modules use an example crate named rand-vec which produces a vector of random size with a set of random values.
The vector returned is generic and bounded by the datatype passed to it
I don't know why you would be running any of these programs, but if you are and rand-vec is required it will be denoted in a readme markdown file.
If this is the case the path to rand-vec is declared relative to the location of the Cargo.toml file.
This means that rand-vec should be stored in the same directory that the project in question is in. 
Ex:<br />
~<br />
└── mallorys-rust<br />
&emsp;├── rand-vec<br />
&emsp;│&ensp;├── Cargo.toml<br />
&emsp;│&ensp;└── src<br />
&emsp;└── executable_folder<br />
&emsp;&emsp;├── Cargo.toml<br />
&emsp;&emsp;└── src <br />

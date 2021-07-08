#### 7/7/21

Currently having trouble pulling data from the JSON file. I believe it's because of how Rust desires Strucs: it wants everything to be explicet 
and well thought out. Therefore I can't just pull the data and use the key willy-nilly. The Key has to be a struct: hence enum and an impliment 
method on the data that should divert the data portions to the correct struct, and we have to create a struct for each data.

However that doesn't sound right: it shouldn't be that hard.

#### 7/8/21

How to handle this. Rust just can't skip over something (or can it?) so I have to account for everything that comes up in the JSON file. Easy 
solution: just skip the things I don't need. This can result in JSON files for every possible dice, and the advantage of doing it this way is 
that I can plug in different JSON files and still run the script.

OK, here's the solution: do this in Python first then do this in Rust. Why? I can develop the algorithm in Python and therefore know what I should
be asking, then apply those questions and concepts to Rust.
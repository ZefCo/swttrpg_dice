#### 7/7/21

Currently having trouble pulling data from the JSON file. I believe it's because of how Rust desires Strucs: it wants everything to be explicet and well thought out. Therefore I can't just pull the data and use the key willy-nilly. The Key has to be a struct: hence enum and an impliment method on the data that should divert the data portions to the correct struct, and we have to create a struct for each data.

However that doesn't sound right: it shouldn't be that hard.
pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    //std::mem::size_of::<str>();
}

 // --> exercises/04_traits/08_sized/src/lib.rs:6:25
 // 	  |
 // 	6 |     std::mem::size_of::<str>();
 // 	  |                         ^^^ doesn't have a size known at compile-time

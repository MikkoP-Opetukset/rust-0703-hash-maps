#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;

/// Entry point for Chapter 8.3: Storing Keys with Associated Values in Hash Maps
///
/// A `HashMap<K, V>` stores values by key. All keys must have the same type
/// and all values must have the same type. Data lives on the heap, and the
/// order of entries is not guaranteed.
fn main() {
    creating_hash_maps();
    accessing_values();
    ownership_and_hash_maps();
    updating_values();
    counting_words();
}

/// # Creating Hash Maps
/// - `HashMap::new()` creates an empty map. `HashMap` is not in the prelude,
///   so it needs a `use` declaration.
/// - `insert` adds a key-value pair.
/// - A map can also be built by collecting an iterator of tuples.
fn creating_hash_maps() {
    println!("\n{:=>80}", "");
    println!("creating_hash_maps()\n");

    let mut scores = HashMap::new();
    scores.insert(String::from("Aria"), 120);
    scores.insert(String::from("Bren"), 95);
    println!("scores = {scores:?}");

    let names = vec![String::from("Cora"), String::from("Dax")];
    let levels = vec![8, 11];
    let level_by_name: HashMap<_, _> = names.into_iter().zip(levels).collect();
    println!("level_by_name = {level_by_name:?}");

    // All values must have the same type.
    // scores.insert(String::from("Eli"), "many"); // Uncomment to see the error.
}

/// # Accessing Values
/// - `get` takes a reference to the key and returns an `Option<&V>`.
/// - `copied()` and `unwrap_or` turn that into a plain value with a default.
/// - A `for` loop over `&map` visits every pair in arbitrary order.
fn accessing_values() {
    println!("\n{:=>80}", "");
    println!("accessing_values()\n");

    let mut scores = HashMap::new();
    scores.insert(String::from("Aria"), 120);
    scores.insert(String::from("Bren"), 95);

    let aria = scores.get("Aria").copied().unwrap_or(0);
    println!("Aria has {aria} points.");

    match scores.get("Zed") {
        Some(points) => println!("Zed has {points} points."),
        None => println!("Zed is not on the scoreboard."),
    }

    for (name, points) in &scores {
        println!("{name}: {points}");
    }
}

/// # Ownership and Hash Maps
/// - Types that implement `Copy`, like `i32`, are copied into the map.
/// - Owned values such as `String` are moved into the map, which becomes
///   their owner.
/// - References can be inserted only if the referenced values live at least
///   as long as the map (covered later during the course).
fn ownership_and_hash_maps() {
    println!("\n{:=>80}", "");
    println!("ownership_and_hash_maps()\n");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut profile = HashMap::new();
    profile.insert(field_name, field_value);
    println!("profile = {profile:?}");

    // Both Strings were moved into the map.
    // println!("{field_name}: {field_value}"); // Uncomment to see the error.
}

/// # Updating Values
/// - Inserting an existing key overwrites its value.
/// - `entry(key).or_insert(value)` inserts only when the key is missing.
/// - `or_insert` returns `&mut V`, which can be used to update the value in
///   place based on the old value.
fn updating_values() {
    println!("\n{:=>80}", "");
    println!("updating_values()\n");

    let mut scores = HashMap::new();

    scores.insert(String::from("Aria"), 100);
    scores.insert(String::from("Aria"), 150);
    println!("overwritten: {scores:?}");

    scores.entry(String::from("Aria")).or_insert(0);
    scores.entry(String::from("Bren")).or_insert(80);
    println!("after or_insert: {scores:?}");

    // `bonus` is not declared `mut`: `mut` would only allow assigning a
    // different reference to `bonus`. `or_insert` returns a `&mut i32`, a
    // mutable reference to the number stored inside the map. `*bonus += 20`
    // follows that reference and changes the number in the map, while
    // `bonus` keeps pointing to the same place.
    let bonus = scores.entry(String::from("Bren")).or_insert(0);
    *bonus += 20;
    println!("after update in place: {scores:?}");
}

/// # Counting Words
/// - A common use of `entry` is counting occurrences.
/// - `or_insert(0)` gives a mutable reference that the code increments.
/// - The mutable reference goes out of scope at the end of each iteration, so
///   the borrow rules are satisfied.
fn counting_words() {
    println!("\n{:=>80}", "");
    println!("counting_words()\n");

    let text = "the knight met the dragon and the dragon met the wizard";
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        // See the comment on `bonus` in `updating_values()` for why `count`
        // is not declared `mut`.
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    // HashMap order is arbitrary, so sort for stable output.
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (word, count) in sorted {
        println!("{word}: {count}");
    }
}

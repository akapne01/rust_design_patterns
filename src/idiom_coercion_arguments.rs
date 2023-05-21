// https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html
// https://doc.rust-lang.org/reference/type-coercions.html
// You should always prefer using *borrowed type*
// over *borrowing the owned type*.
// Such as 1) &str over &String
// 2) &[T] over &Vec<T>
// 3) &T over &Box<T>
// Why?
// Using borrowed types you can avoid layers of indirection for
// those instances where the owned type already provides a layer
// of indirection.
// For instance, a String has a layer of indirection, so a
// &String will have 2 layers of indirection.
// We can avoid this by using &str instead, and letting
// &String coerce to a &str whenever the function is invoked.

fn has_three_vowels_using_reference_to_string(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => {
                vowel_count = 0;
            }
        }
    }
    false
}

fn has_three_vowels_using_reference_to_str_slice(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => {
                vowel_count = 0;
            }
        }
    }
    false
}

pub fn run() {
    println!("Idiom: Coercion Arguments");

    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    let cucumber = "Cucumber".to_string();

    println!("{}: {}", ferris, has_three_vowels_using_reference_to_string(&ferris));
    println!("{}: {}", curious, has_three_vowels_using_reference_to_string(&curious));
    println!("{}: {}", cucumber, has_three_vowels_using_reference_to_string(&cucumber));

    println!("{}: {}", ferris, has_three_vowels_using_reference_to_str_slice("Ferris"));
    println!("{}: {}", curious, has_three_vowels_using_reference_to_str_slice("Curious"));
    println!("{}: {}", cucumber, has_three_vowels_using_reference_to_str_slice("Cucumber"));

    // Using &str gives more flexibility than using &String.
    // &str is a reference to a String slice located in Stack (compiled binary).
    // &String points to a String in the heap memory. The reference itself is located
    // in Stack memory, but the contents of the String are in Heap memory. 

    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence_string.split(' ') {
        if has_three_vowels_using_reference_to_str_slice(word) {
            println!("{} has three consecutive vowels!", word);
        }
    }
}
use crate::{search, search_insensitive};

const QUOTE: &str = "\
We knew the world would not be the same.
A few people laughed, a few people cried.
Most people were silent.
I remembered the line from the Hindu scripture, the Bhagavad Gita;
Vishnu is trying to persuade the Prince that he should do his duty and,
to impress him, takes on his multi-armed form and says,
'Now I am become Death, the destroyer of worlds.'
I suppose we all thought that, one way or another.";

// TODO: Regex search tests!!!

#[test]
fn single_search() {
    let query = "silent";

    assert_eq!(vec!["Most people were silent."], search(query, QUOTE))
}

#[test]
fn multiple_search() {
    let query = "world";
    assert_eq!(
        vec![
            "We knew the world would not be the same.",
            "'Now I am become Death, the destroyer of worlds.'"
        ],
        search(query, QUOTE)
    )
}

#[test]
fn multiple_search_insensitive() {
    let query = "we";

    assert_eq!(
        vec![
            "We knew the world would not be the same.",
            "Most people were silent.",
            "I suppose we all thought that, one way or another."
        ],
        search_insensitive(query, QUOTE)
    )
}

/// Naively counts the number of words in some text.
///
/// A word is non-whitespace followed by whitespace or the last word in the text.
pub fn count_words_naive(text: &str) -> usize {
    let mut word_count = 0;
    let mut in_word = false;

    for char in text.chars() {
        if char.is_whitespace() {
            if in_word {
                in_word = false;
                word_count += 1;
            }
            continue;
        }

        in_word = true;
    }

    if in_word == true {
        word_count += 1;
    }

    word_count
}

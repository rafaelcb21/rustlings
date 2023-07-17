// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
//pub fn capitalize_first(input: &str) -> String {
//    let mut c = input.chars(); // é um objeto Chars([]) retorna um iterador de chars
//    let first_char = match c.next() {
//        None => String::new(),
//        Some(first) => first.to_string().to_uppercase(),
//    };
//    
//    // 1) abordagem usando o objeto Chars
//    //let vector_chars: Vec<char> = c.collect(); // converte em Char em Vec, ou seja transformar um iterador em Vec
//    //let rest: String = vector_chars.into_iter().collect(); // usa o vetor de char, transforma em um iterador e junta os characteres
//    
//    // 2) abordagem sem precisar transformar em vetor o Char
//    //let rest: String = c.collect(); // converte o Char em String
//    
//    // 3) abordagem usando &str diretor
//    let (_, rest) = input.split_at(1);
//    
//    //println!("{:?}", first_char + rest.as_str());
//    //println!("{:?}", first_char + &rest);
//    first_char + &rest
//}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars(); // é um objeto Chars([]) retorna um iterador de chars
    let word = match c.next() {
        None => String::new(),
        Some(first) => {
            let first_char = first.to_string().to_uppercase();
            let (_, rest) = input.split_at(1);
            first_char + &rest
        },
    };
    word
}
// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.into_iter().map(|word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.into_iter().map(|word| capitalize_first(word)).collect()
}

// EXPLICAÇÃO DO collect()
// O comportamento do método collect() em relação ao tipo de retorno é 
// determinado pelo contexto de uso. No caso das duas funções 
// o contexto é diferente, o que resulta em comportamentos diferentes do collect().
// 
// No Rust, o método collect() é flexível e polimórfico.
// Ele pode coletar elementos de um iterador em várias 
// estruturas de dados, como um vetor (Vec<T>), uma string (String) 
// ou outros tipos que implementam a trait FromIterator<T>.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

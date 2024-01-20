
use log::{debug, warn};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum EncodingError {
    #[error("input must be ASCII string")]
    NonAsciiInput,
    #[error("no valid sentences found")]
    NoValidSentences,
}

#[derive(Error, Debug)]
pub enum DecodingError {
    #[error("character set cannot be empty")]
    EmptyCharacterSet,
    #[error("invalid code: {0}")]
    InvalidCode(usize),
}

/// Encodes a given text into a vector of word counts per sentence.
///
/// This function performs text-based steganography using the Words Per Sentence (WPS) method.
/// It splits the input text into sentences, counts the number of words in each sentence,
/// and returns a vector of these counts. The function ensures that the input text is ASCII-encoded
/// and contains valid sentences.
///
/// # Arguments
/// * `txt` - A string slice (`&str`) representing the text to be encoded.
///
/// # Returns
/// * `Ok(Vec<usize>)` - A vector of word counts per sentence if encoding is successful.
/// * `Err(EncodingError)` - An `EncodingError` in case of:
///   - Non-ASCII input (`EncodingError::NonAsciiInput`).
///   - Input text with no valid sentences (`EncodingError::NoValidSentences`).
///
/// # Errors
/// This function returns an error in the following cases:
/// - If the input text is not ASCII-encoded, an `EncodingError::NonAsciiInput` error is returned.
/// - If the input text does not contain any valid sentences (i.e., no words or only punctuation),
///   an `EncodingError::NoValidSentences` error is returned.
///
/// # Examples
/// ```
/// use stego_wps::encode;
/// use stego_wps::EncodingError;
///
/// let text = "Hello world. This is a test sentence!";
/// match encode(text) {
///     Ok(encoded) => println!("Encoded text: {:?}", encoded),
///     Err(e) => match e {
///         EncodingError::NonAsciiInput => println!("Input text must be ASCII"),
///         EncodingError::NoValidSentences => println!("No valid sentences found"),
///     },
/// }
/// ```
pub fn encode(txt: &str) -> Result<Vec<usize>, EncodingError> {
    if !txt.is_ascii() {
        warn!("Non-ASCII string encountered");
        return Err(EncodingError::NonAsciiInput);
    }

    let encoded: Vec<usize> = txt
        .split(|c: char| ['.', '!', '?'].contains(&c))
        .map(|s| s.split_whitespace().count())
        .filter(|&count| count > 0)
        .collect();

    if encoded.is_empty() {
        warn!("No valid sentences found in the input text");
        return Err(EncodingError::NoValidSentences);
    }

    debug!("Encoded text: {:?}", encoded);
    Ok(encoded)
}

/// Decodes a vector of word counts per sentence into a string using a specified character set.
///
/// This function is part of a text-based steganography system using the Words Per Sentence (WPS) method.
/// It decodes a sequence of word counts (represented as a vector of `usize`) back into a string based on a given
/// character set. Each word count is used to find a corresponding character in the character set.
/// The function ensures that the character set is not empty and that each word count is valid within the context of the character set.
///
/// # Arguments
/// * `encoded` - A slice of `usize` representing the encoded word counts.
/// * `character_set` - A string slice (`&str`) representing the character set used for decoding.
///
/// # Returns
/// * `Ok(String)` - A `String` decoded from the encoded word counts if decoding is successful.
/// * `Err(DecodingError)` - A `DecodingError` in case of:
///   - Empty character set (`DecodingError::EmptyCharacterSet`).
///   - Invalid word count that does not correspond to any character in the set (`DecodingError::InvalidCode`).
///
/// # Errors
/// This function returns an error in the following cases:
/// - If the character set provided is empty, an `DecodingError::EmptyCharacterSet` error is returned.
/// - If a word count in the `encoded` vector does not correspond to a character in the character set (i.e., out of range),
///   a `DecodingError::InvalidCode` error is returned with the invalid count.
///
/// # Examples
/// ```
/// use stego_wps::decode;
/// use stego_wps::DecodingError;
///
/// let encoded = vec![3, 5, 1];
/// let character_set = "ABCDE";
/// match decode(&encoded, character_set) {
///     Ok(decoded) => println!("Decoded string: {}", decoded),
///     Err(e) => match e {
///         DecodingError::EmptyCharacterSet => println!("Character set cannot be empty"),
///         DecodingError::InvalidCode(code) => println!("Invalid code: {}", code),
///     },
/// }
/// ```
pub fn decode(encoded: &[usize], character_set: &str) -> Result<String, DecodingError> {
    if character_set.is_empty() {
        warn!("Character set is empty");
        return Err(DecodingError::EmptyCharacterSet);
    }

    let charset_len = character_set.len();
    let decoded: Result<String, _> = encoded
        .iter()
        .filter(|&&code| code != 0)
        .map(|&code| {
            character_set
                .chars()
                .nth((code - 1) % charset_len)
                .ok_or(DecodingError::InvalidCode(code))
        })
        .collect();

    match decoded {
        Ok(d) => {
            debug!("Decoded string: {}", d);
            Ok(d)
        }
        Err(e) => {
            warn!("Decoding error: {:?}", e);
            Err(e)
        }
    }
}

/// Compares a secret message with a cover text to calculate the necessary changes in word count per sentence to encode the message.
///
/// This function is part of a text-based steganography system using the Words Per Sentence (WPS) method.
/// It calculates the differences in word count needed for each sentence of the cover text to match the encoded form of the secret message.
/// The function returns a vector where each element represents the change in the number of words required for a corresponding sentence
/// in the cover text to encode a character of the secret message using the provided character set.
///
/// # Arguments
/// * `secret_message` - A string slice (`&str`) representing the secret message to be encoded.
/// * `cover_text` - A string slice (`&str`) representing the cover text used for encoding.
/// * `character_set` - A string slice (`&str`) representing the character set used for encoding.
///
/// # Returns
/// * `Ok(Vec<isize>)` - A vector of `isize` where each element represents the necessary change in word count for each sentence.
///    Positive values indicate additional words needed, while negative values indicate words to be removed.
/// * `Err(String)` - An error message if encoding fails or if a character in the secret message is not found in the character set.
///
/// # Panics
/// This function panics if a value conversion to `isize` is out of range, indicating a potential issue with very large values.
///
/// # Errors
/// This function returns an error if:
/// - The cover text cannot be successfully encoded.
/// - A character in the secret message is not found in the character set.
///
/// # Examples
/// ```
/// use stego_wps::compare;
///
/// let secret_message = "HELLO";
/// let cover_text = "This is a sentence. Another one here.";
/// let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
/// let changes = compare(secret_message, cover_text, character_set).expect("Comparison failed");
/// println!("Changes needed: {:?}", changes);
/// ```
pub fn compare(
    secret_message: &str,
    cover_text: &str,
    character_set: &str,
) -> Result<Vec<isize>, String> {
    if secret_message.is_empty() {
        return Ok(vec![]);
    }

    // Create a HashMap from the character set for quick lookups
    let charset_map: HashMap<char, isize> = character_set
        .chars()
        .enumerate()
        .map(|(i, c)| {
            isize::try_from(i).map_or(Err("Value out of range".to_string()), |val| {
                Ok((c, val + 1))
            })
        })
        .collect::<Result<HashMap<char, isize>, _>>()?;

    // Encode the cover text
    let cover_encoded =
        encode(cover_text).map_err(|e| format!("Error encoding cover text: {e:?}"))?;

    // Calculate the position of each character in the character set for the secret message
    let secret_positions = secret_message
        .chars()
        .map(|c| {
            charset_map
                .get(&c)
                .copied()
                .ok_or_else(|| format!("Character '{c}' not found in character set"))
        })
        .collect::<Result<Vec<isize>, _>>()?;

    // Calculate the changes needed
    let mut changes = vec![0; cover_encoded.len()];
    for (i, &pos) in secret_positions.iter().enumerate() {
        if i < cover_encoded.len() {
            changes[i] = pos
                - isize::try_from(cover_encoded[i])
                    .unwrap_or_else(|_| panic!("Value out of range for isize conversion"));
        } else {
            changes.push(pos);
        }
    }

    // Adjust for any extra sentences in the cover text
    for i in secret_positions.len()..cover_encoded.len() {
        changes[i] = -(isize::try_from(cover_encoded[i])
            .unwrap_or_else(|_| panic!("Value out of range for isize conversion")));
    }

    Ok(changes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_valid_input() {
        let input = "This is a sentence. This is another.";
        let result = encode(input).expect("Failed to encode");
        assert_eq!(result, vec![4, 3]);
    }

    #[test]
    fn test_encode_non_ascii_input() {
        let input = "This is a sentence with non-ascii char ö.";
        match encode(input) {
            Ok(_) => panic!("Should have failed on non-ASCII input"),
            Err(EncodingError::NonAsciiInput) => (), // Pass the test
            Err(_) => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_decode_basic() {
        let encoded = vec![1, 26, 5]; // Corresponding to some encoded numbers
        let character_set = "abcdefghijklmnopqrstuvwxyz";
        let result = decode(&encoded, character_set).expect("Failed to decode");
        assert_eq!(result, "aze"); // Assuming this is the expected decoded string
    }

    #[test]
    fn test_decode_with_empty_input() {
        let encoded = vec![];
        let character_set = "abcdefghijklmnopqrstuvwxyz";
        let result = decode(&encoded, character_set).expect("Failed to decode");
        assert_eq!(result, ""); // Decoding an empty vector should result in an empty string
    }

    #[test]
    fn test_basic_wps_encode() {
        let input = "Hello, this is a test.\n        Does this work?\n        I sure hope so.";
        let expected = vec![5, 3, 4];
        let result = encode(input).expect("Failed to encode");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_incomplete_sentence_encode() {
        let input = "Hello. This is a great tool. .. Bad sentence punctuation";
        let expected = vec![1, 5, 3];
        let result = encode(input).expect("Failed to encode");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_normal_decode() {
        let hidden_msg = "SECRET";
        let cover_text = "\n        Hello Bob, I hope you are well and good, I would like to know if you are free tomorrow.\n        Hmm, How about a picnic?\n\n        At the park?\n\n        I would very much look forward to that, but will jane bring her dog, i was just wondering?\n\n        Anyway i would like to.\n        But how many days until Sara will be making her famous lemon drizzle cake, it was to die for, before.\n        ";
        let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let encoded = encode(cover_text).unwrap(); // Note that the 'expect' method could be used here as well.
        let result = decode(&encoded, character_set).expect("Failed to decode");
        assert_eq!(result, hidden_msg);
    }

    #[test]
    fn test_compare_exact_match() {
        let secret_message = "HELLO";
        let cover_text = "This is a sentence. And another one.";
        let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let result = compare(secret_message, cover_text, character_set).expect("Failed to compare");
        assert_eq!(result, vec![4, 2, 12, 12, 15]);
    }

    #[test]
    fn test_compare_length_mismatch() {
        let secret_message = "HELLO";
        let cover_text = "This is a sentence. This is another. And yet another.";
        let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let result = compare(secret_message, cover_text, character_set).expect("Failed to compare");
        assert_eq!(result, vec![4, 2, 9, 12, 15]);
    }

    #[test]
    fn test_compare_non_ascii_input() {
        let secret_message = "HELLO";
        let cover_text = "This sentence contains ö.";
        let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let result = compare(secret_message, cover_text, character_set);
        assert!(result.is_err());
    }

    #[test]
    fn test_compare_empty_input() {
        let secret_message = "";
        let cover_text = "";
        let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let result = compare(secret_message, cover_text, character_set).expect("Failed to compare");
        assert_eq!(result, Vec::<isize>::new());
    }
}

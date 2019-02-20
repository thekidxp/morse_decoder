use std::collections::HashMap;

pub struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

#[allow(unused_variables, dead_code)]
impl MorseDecoder {
    pub fn new() -> MorseDecoder {
        MorseDecoder {
            morse_code: [
                (".-".to_owned(), "A".to_owned()),
                ("-...".to_owned(), "B".to_owned()),
                ("-.-.".to_owned(), "C".to_owned()),
                ("-..".to_owned(), "D".to_owned()),
                (".".to_owned(), "E".to_owned()),
                ("..-.".to_owned(), "F".to_owned()),
                ("--.".to_owned(), "G".to_owned()),
                ("....".to_owned(), "H".to_owned()),
                ("..".to_owned(), "I".to_owned()),
                (".---".to_owned(), "J".to_owned()),
                ("-.-".to_owned(), "K".to_owned()),
                (".-..".to_owned(), "L".to_owned()),
                ("--".to_owned(), "M".to_owned()),
                ("-.".to_owned(), "N".to_owned()),
                ("---".to_owned(), "O".to_owned()),
                (".--.".to_owned(), "P".to_owned()),
                ("--.-".to_owned(), "Q".to_owned()),
                (".-.".to_owned(), "R".to_owned()),
                ("...".to_owned(), "S".to_owned()),
                ("-".to_owned(), "T".to_owned()),
                ("..-".to_owned(), "U".to_owned()),
                ("...-".to_owned(), "V".to_owned()),
                (".--".to_owned(), "W".to_owned()),
                ("-..-".to_owned(), "X".to_owned()),
                ("-.--".to_owned(), "Y".to_owned()),
                ("--..".to_owned(), "Z".to_owned()),
                (".----".to_owned(), "1".to_owned()),
                ("..---".to_owned(), "2".to_owned()),
                ("...--".to_owned(), "3".to_owned()),
                ("....-".to_owned(), "4".to_owned()),
                (".....".to_owned(), "5".to_owned()),
                ("-....".to_owned(), "6".to_owned()),
                ("--...".to_owned(), "7".to_owned()),
                ("---..".to_owned(), "8".to_owned()),
                ("----.".to_owned(), "9".to_owned()),
                ("-----".to_owned(), "0".to_owned()),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn decode_bits(&self, encoded: &str) -> String {
        let time_unit = MorseDecoder::find_time_unit(encoded);
        encoded
            .trim_matches('0')
            .split('0')
            .map(|chunk| chunk.chars().count())
            .map(|number| {
                if number == 0 {
                    return "0".to_owned();
                } else if number == time_unit {
                    return ".".to_owned();
                } else if number == 3 * time_unit {
                    return "-".to_owned();
                }
                "".to_owned()
            })
            .collect::<Vec<String>>()
            .join("")
        // let trimmed = encoded.trim_matches('0');
        // let time_unit = MorseDecoder::find_time_unit(trimmed);
        // trimmed.to_owned()
    }

    fn find_time_unit(encoded: &str) -> usize {
        encoded
            .trim_matches('0')
            .split('0')
            .map(|chunk| chunk.chars().count())
            .filter(|count| count > &0usize)
            .min()
            .unwrap()
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .trim()
            .split("   ")
            .map(|word| {
                word.split(" ")
                    .filter_map(|c| self.morse_code.get(c))
                    .cloned()
                    .collect()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_letter() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse("...."), "H");
    }

    #[test]
    fn test_one_word() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(".... . -.--"), "HEY");
    }

    #[test]
    fn test_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
            "HEY JUDE"
        );
    }

    #[test]
    fn test_with_leading_and_trailing_whitespace() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_morse("    .... . -.--   .--- ..- -.. .  "),
            "HEY JUDE"
        );
    }

    #[test]
    fn test_decode_bits_with_one_letter_dots_and_dashes() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_bits("00000111111111000111000111111111000"),
            "-.-".to_string()
        );
    }

    #[test]
    fn test_decode_bits_with_hey() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_bits("110011001100110000001100000011111100110011111100111111"),
            ".... . -.--".to_string()
        );
    }

    #[test]
    fn test_decode_bits_with_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"), ".... . -.--   .--- ..- -.. .".to_string());
    }
}

// impl MorseDecoder {
//     pub fn decode_morse(&self, encoded: &str) -> String {
//         let words: Vec<&str> = encoded.split("   ").collect();
//         let mut answer = String::from("");

//         for word in words {
//             let letters: Vec<&str> = word.split(" ").collect();
//             for letter in letters {
//                 if letter != "" {
//                     answer.push_str(self.morse_code.get(letter).unwrap());
//                 }
//             }
//             answer.push(' ');
//         }
//         answer.trim().to_string()
//     }
// }

/*
Part 2 instructions:

In this kata you have to write a Morse code decoder for wired electrical telegraph.

Electric telegraph is operated on a 2-wire line with a key that, when pressed, connects the wires together, which can be detected on a remote station. The Morse code encodes every character being transmitted as a sequence of "dots" (short presses on the key) and "dashes" (long presses on the key).

When transmitting the Morse code, the international standard specifies that:

    "Dot" – is 1 time unit long.
    "Dash" – is 3 time units long.
    Pause between dots and dashes in a character – is 1 time unit long.
    Pause between characters inside a word – is 3 time units long.
    Pause between words – is 7 time units long.

However, the standard does not specify how long that "time unit" is. And in fact different operators would transmit at different speed. An amateur person may need a few seconds to transmit a single character, a skilled professional can transmit 60 words per minute, and robotic transmitters may go way faster.

For this kata we assume the message receiving is performed automatically by the hardware that checks the line periodically, and if the line is connected (the key at the remote station is down), 1 is recorded, and if the line is not connected (remote key is up), 0 is recorded. After the message is fully received, it gets to you for decoding as a string containing only symbols 0 and 1.

For example, the message HEY JUDE, that is ···· · −·−− ·−−− ··− −·· · may be received as follows:

1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011

As you may see, this transmission is perfectly accurate according to the standard, and the hardware sampled the line exactly two times per "dot".

That said, your task is to implement two functions:

    Function decodeBits(bits), that should find out the transmission rate of the message, correctly decode the message to dots ., dashes - and spaces (one between characters, three between words) and return those as a string. Note that some extra 0's may naturally occur at the beginning and the end of a message, make sure to ignore them. Also if you have trouble discerning if the particular sequence of 1's is a dot or a dash, assume it's a dot.

    Function decodeMorse(morseCode), that would take the output of the previous function and return a human-readable string.

NOTE: For coding purposes you have to use ASCII characters . and -, not Unicode characters.

The Morse code table is preloaded for you as MORSE_CODE dictionary; in Java MorseCode class is provided; in Haskell the codes are in a Map String String and can be accessed like this: morseCodes ! ".--" - feel free to use it.

All the test strings would be valid to the point that they could be reliably decoded as described above, so you may skip checking for errors and exceptions, just do your best in figuring out what the message is!

Good luck!

After you master this kata, you may try to Decode the Morse code, for real.
*/

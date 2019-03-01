use std::{collections::HashMap, iter::repeat};

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
            .split(repeat('0').take(7 * time_unit).collect::<String>().as_str())
            .map(|word| {
                word.split(repeat('0').take(3 * time_unit).collect::<String>().as_str())
                    .map(|letter| {
                        letter
                            .split({ repeat('0').take(time_unit).collect::<String>().as_str() })
                            .map(|symbol| {
                                if symbol.chars().count() == time_unit {
                                    return ".";
                                } else {
                                    return "-";
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("")
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("   ")
    }

    fn decode_bits_advanced(&self, encoded: &str) -> String {
        unimplemented!()
    }

    fn find_time_unit(encoded: &str) -> usize {
        let ones_size = encoded
            .trim_matches('0')
            .split('0')
            .map(|chunk| chunk.chars().count())
            .filter(|count| count > &0usize)
            .min()
            .unwrap_or_default();
        let zeroes_size = encoded
            .trim_matches('0')
            .split('1')
            .map(|chunk| chunk.chars().count())
            .filter(|count| count > &0usize)
            .min()
            .unwrap_or_default();
        if ones_size > 0 && zeroes_size > 0 {
            if  ones_size <= zeroes_size {
                return ones_size;
            } else {
                return zeroes_size;
            }
        } else {
            if ones_size > 0 {
                return ones_size;
            } else {
                return zeroes_size;
            }
        }
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
    fn test_decode_bits_with_one_letter_dots_and_dashes_short_time() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_bits("00000111010111000"), "-.-".to_string());
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
    fn test_decode_bits_with_hey_short_time() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_bits("101010100010001110101110111"),
            ".... . -.--".to_string()
        );
    }

    #[test]
    fn test_decode_bits_with_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"), ".... . -.--   .--- ..- -.. .".to_string());
    }
    
    #[test]
    fn test_letter_m() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(&decoder.decode_bits("000011111100111111000000")), "M");
    }

    #[test]
    fn hey_jude_advanced() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(&decoder.decode_bits_advanced("0000000011011010011100000110000001111110100111110011111100000000000111011111111011111011111000000101100011111100000111110011101100000100000")), "HEY JUDE");
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

/* One of the top solutions
impl MorseDecoder {
    pub fn decode_bits(&self, encoded: &str) -> String {
    
      let encoded = encoded.trim_matches('0');
    
      //Detect correct chunk size
      let mut chunk_size = 1;
      for i in (1..20).rev() {
        let mut good_chunk = true;
        for chunk in encoded.as_bytes().chunks(i) {
          let c = chunk[0];
          if !chunk.iter().all(|&x| x == c) {
            good_chunk = false;
            break;
          }
        }
        if good_chunk {
          chunk_size = i;
          break;
        }
      }
      
      //Remove extra numbers
      let mut encoded = encoded.as_bytes().chunks(chunk_size)
      .map(|x| x[0] as char).collect::<String>();

      encoded = encoded.split("0").map(|s| match s {
        "" => "",
        "111" => "-",
        "1" => ".",
        _ => panic!(format!("Unknown input '{}'", s)),
      }).collect::<Vec<&str>>().join("0");
      encoded = encoded.replace("0000000", "   ");    //word separator
      encoded = encoded.replace("000", " ");    //Letter separator
      encoded = encoded.replace("0", "");    //1 separator. Not needed

      encoded
    }
    
    pub fn decode_morse(&self, encoded: &str) -> String {
            encoded
            .trim()
            .split("   ")
            .map(|x| x.split(' ')
                      .filter_map(|y| { self.morse_code.get(y) })
                      .cloned()
                      .collect())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
*/


/*
Part of Series 3/3
This kata is part of a series on the Morse code. Make sure you solve the first part and the second part and then reuse and advance your code to solve this one.

In this kata you have to deal with "real-life" scenarios, when Morse code transmission speed slightly varies throughout the message as it is sent by a non-perfect human operator. Also the sampling frequency may not be a multiple of the length of a typical "dot".

For example, the message HEY JUDE, that is ···· · −·−−   ·−−− ··− −·· · may actually be received as follows:

0000000011011010011100000110000001111110100111110011111100000000000111011111111011111011111000000101100011111100000111110011101100000100000

As you may see, this transmission is generally accurate according to the standard, but some dots and dashes and pauses are a bit shorter or a bit longer than the others.

Note also, that, in contrast to the previous kata, the estimated average rate (bits per dot) may not be a whole number – as the hypotetical transmitter is a human and doesn't know anything about the receiving side sampling rate.

For example, you may sample line 10 times per second (100ms per sample), while the operator transmits so that his dots and short pauses are 110-170ms long. Clearly 10 samples per second is enough resolution for this speed (meaning, each dot and pause is reflected in the output, nothing is missed), and dots would be reflected as 1 or 11, but if you try to estimate rate (bits per dot), it would not be 1 or 2, it would be about (110 + 170) / 2 / 100 = 1.4. Your algorithm should deal with situations like this well.

Also, remember that each separate message is supposed to be possibly sent by a different operator, so its rate and other characteristics would be different. So you have to analyze each message (i. e. test) independently, without relying on previous messages. On the other hand, we assume the transmission charactestics remain consistent throghout the message, so you have to analyze the message as a whole to make decoding right. Consistency means that if in the beginning of a message '11111' is a dot and '111111' is a dash, then the same is true everywhere in that message. Moreover, it also means '00000' is definitely a short (in-character) pause, and '000000' is a long (between-characters) pause.

That said, your task is to implement two functions:

1. Function decodeBitsAdvanced(bits), that should find an estimate for the transmission rate of the message, take care about slight speed variations that may occur in the message, correctly decode the message to dots ., dashes - and spaces (one between characters, three between words) and return those as a string. Note that some extra 0's may naturally occur at the beginning and the end of a message, make sure to ignore them. If message is empty or only contains 0's, return empty string. Also if you have trouble discerning if the particular sequence of 1's is a dot or a dash, assume it's a dot. If stuck, check this for ideas.

2. Function decodeMorse(morseCode), that would take the output of the previous function and return a human-readable string. If the input is empty string or only contains spaces, return empty string.

NOTE: For coding purposes you have to use ASCII characters . and -, not Unicode characters.

The Morse code table is preloaded for you as MORSE_CODE dictionary, feel free to use it. (For C, the function morse_code acts like the dictionary. For C++ a map is used. For C#, it's called Preloaded.MORSE_CODE).

const char *morse_code (const char *dotsdashes);

Of course, not all messages may be fully automatically decoded. But you may be sure that all the test strings would be valid to the point that they could be reliably decoded as described above, so you may skip checking for errors and exceptions, just do your best in figuring out what the message is!
*/

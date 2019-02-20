// Preloaded:
//
// struct MorseDecoder {
//     morse_code: HashMap<String, String>,
// }
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

impl MorseDecoder {

    fn decode_morse(&self, encoded: &str) -> String {
        let words: Vec<&str> = encoded.split(" ").collect();
        let mut answer = String::from("");
        
        for word in words {
            answer.push_str(self.morse_code.get(word).unwrap());
        }
        
        answer
    }
    
}
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for i in 0..words.len(){
            if Self::is_pal(words[i].clone()) {
                return words[i].clone();
            }
        }
        return "".to_string();
    }

    pub fn is_pal(s: String) -> bool {
        let byte_array: &[u8] = s.as_bytes();
        let mut i = 0;
        let mut j = s.len()-1;

        while i < j {
            if byte_array[i]!=byte_array[j]{
                return false;
            }
            i+=1;
            j-=1;
        }

        return true;
    }
}

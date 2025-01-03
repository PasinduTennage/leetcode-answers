func firstPalindrome(words []string) string {
    for i:=0; i < len(words); i++{
        if isPal(words[i]){
            return words[i]
        }
    }
    return ""


}

func isPal(s string)bool{
    i, j := 0, len(s)-1

    for i<j && i < len(s) && j >=0 {
        if s[i]!= s[j] {
            return false
        }
        i++
        j--
    }

    return true
}

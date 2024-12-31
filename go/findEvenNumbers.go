func findEvenNumbers(digits []int) []int {
    frequencies := make([]int, 10)
    for i:=0; i < len(digits); i++{
        frequencies[digits[i]]++
    }

    ans := []int{}

    for i:= 100; i <= 998; i+=2 {
        l_freq := make([]int, 10)
        l_freq[i%10]++
        l_freq[(i/10)%10]++
        l_freq[(i/100)%10]++

        found := true

        for j:=0; j <10; j++{
            if l_freq[j] > frequencies[j] {
                found = false
                break
            }
        }

        if found {
            ans = append(ans, i)
        }

    }

    return ans
}

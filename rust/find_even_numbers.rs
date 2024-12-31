impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut frequencies = [0; 10];

        for i in 0..digits.len(){
            frequencies[digits[i] as usize]+=1;
        }

        let mut ans = Vec::<i32>::new();

        let mut i = 100;

        loop {
            let mut l_freq = [0; 10];
            l_freq[i%10]+=1;
            l_freq[(i/10)%10]+=1;
            l_freq[(i/100)%10]+=1;

            let mut found = true;

            for i in 0..10{
                if frequencies[i] < l_freq[i] {
                    found = false;
                    break;
                }
            }

            if found {
                ans.push(i as i32);
            }

            i = i + 2;

            if i > 998 {
                break;
            }


        }

        return ans;
    }
}

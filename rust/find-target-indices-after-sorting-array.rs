impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_left = 0;
        let mut targets = 0;

        for i in 0..nums.len(){
            if nums[i] < target {
                num_left += 1;
            }else if nums[i] == target {
                targets+=1;
            }
        }

        let mut ans: Vec<i32> =  Vec::new();

        for i in num_left..(num_left+targets){
            ans.push(i);
        }  

        ans
    }
}

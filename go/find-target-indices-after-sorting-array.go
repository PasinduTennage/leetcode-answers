func targetIndices(nums []int, target int) []int {
    to_left := 0
    num_targets:= 0

    for i:=0; i < len(nums); i++{
        if nums[i] == target {
            num_targets++
        }else if nums[i] < target{
            to_left++
        }
    }    

    ans := []int{}
    for i:= to_left; i < num_targets+to_left; i++{
        ans = append(ans, i)
    }

    return ans
}

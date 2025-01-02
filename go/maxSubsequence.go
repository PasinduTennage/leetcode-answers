import "sort"
func maxSubsequence(nums []int, k int) []int {
    arr_cpy := make([]int, len(nums))
    copy(arr_cpy, nums)
    sort.Ints(arr_cpy)

    largest_elems := make(map[int]int)

    for i:=len(arr_cpy)-k; i < len(arr_cpy); i++{
        v, ok := largest_elems[arr_cpy[i]]
        if ok {
            largest_elems[arr_cpy[i]] = v +1
        }else{
            largest_elems[arr_cpy[i]] = 1
        }
    }

    ans := make([]int, 0)

    for i:= 0; i < len(nums); i++{
        v, ok := largest_elems[nums[i]]
        if ok && v > 0 {
            largest_elems[nums[i]] = v - 1
            ans = append(ans, nums[i])
        }
    }

    return ans
       
}

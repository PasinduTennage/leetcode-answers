impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods : Vec<Vec<i32>> = vec![vec![0;3];10];   
        let mut i = 0;
        let bytes = rings.as_bytes();

        loop {
            let color = bytes[i];
            let rod = bytes[i+1];

            if color == b'B'{
                rods[(rod-b'0') as usize][0] = 1 ; 
            }else if color == b'G'{
                rods[(rod-b'0') as usize][1] = 1 ; 
            }else if color == b'R'{
                rods[(rod-b'0') as usize][2] = 1 ; 
            }
            i = i +2 ;

            if i >=rings.len(){
                break;
            }
        } 

        let mut ans = 0;

        for i in 0..10{
            if rods[i][0] == 1 && rods[i][1] == 1 && rods[i][2] == 1 {
                ans+=1;
            }
        }

        return ans;
    }
}

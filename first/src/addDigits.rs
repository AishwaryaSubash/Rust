pub fn function(mut m:i32)->i32 {
    let mut sum:i32=0;
    while m>0 {
        let dig = m%10;
        sum += dig;
        m = m/10;
    }
    return sum;
}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut m:i32 =num;
        while m>=10 {
            m=function(m);
        }
        return m;
    }
}
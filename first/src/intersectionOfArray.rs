use std::collections::HashSet;

impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
         
    let mut ans = HashSet::new();

    for i in &mut nums1 {
        for j in &mut nums2 {
            if (*i==*j) {
                ans.insert(*i);
            }
        }
    }
    let num: Vec<i32> = ans.into_iter().collect();
    return num;
    
    }

    //  for x in &ans {
    //     println!("{x:?}");
    // }
}
use itertools::Itertools;
use std::{collections::{HashSet, BTreeMap}, vec};

const Y_PICK: [[usize; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

fn main() {

//  Initial insight is that the product of a row is equal to 2^15, so we can switch to a normal sum-based square using values that 
//  represent powers of 2

//  Define the square as a central cell x and 4 complementary pairs (y, z) to form the sequences that pass through x.
//  y_i < z_i; y_i != y_j; z_i != z_j
//  If a zero value is allowed, the largest value of the 4 y_i must be at least 3 (0, 1, 2, 3)

//  y_i + z_i = 15 - x
//  therefore 2y_i < 15 - x
//  therefore 2 * 3 < 15 - x, so x < 9

//  Loop over possible values of x and list possible (y, z) pairs for each x
    let yz_vec = (0_u8..9).map(|x| (0..(16 - x) / 2).map(|y| [y, 15 - x - y])
                                        .filter(|[y, z]| y != z && y != &x && z != &x)
                                        .collect_vec()).collect::<Vec<Vec<[u8; 2]>>>();


    let mut squares_dict = BTreeMap::<i32, Vec<Vec<Vec<u8>>>>::new();

//  for each x, find triplets (y_i, y_j, z_k) of complementary pairs such that their sum is 15
//  by ensuring a yyz combination, we avoid reflections in the horizontal axis
    for (x, yz) in yz_vec.into_iter().enumerate() {
        for triplet in yz.iter().combinations(3) {
            let top_row_pick = Y_PICK.iter().map(|idx| 
                                                    idx.iter().zip(triplet.to_owned()).map(|(&ix, &yz)| yz[ix])
                                                                .collect::<Vec<u8>>())
                                            .filter(|t| t.iter().sum::<u8>() == 15)
                                            .collect::<Vec<Vec<u8>>>();
            if top_row_pick.len() == 0 {continue;}

//  Shuffle the members of the top row because they may lead to a different square which is not a reflection in the vertical axis
            let top_rows = top_row_pick.iter().map(|sel| 
                                            (&sel.clone().into_iter().sorted().permutations(3).collect_vec()[..3]).to_owned())
                                            .flatten()
                                            .collect::<Vec<Vec<u8>>>();

//  Each top row has a complementary bottom row formed from the difference between (15 - x) and the corresponding member (but reflected in x)                                            
            let bot_rows = top_rows.iter().map(|candidate| 
                                                            candidate.iter().rev().map(|yz| 15_u8 - x as u8 - yz).collect::<Vec<u8>>())
                                                        .collect::<Vec<Vec<u8>>>();

//  And the middle row is then calculated by subtracting the corner values from 15 in each column (reproducing x in the middle column in passing)
            let mid_rows = top_rows.iter().zip(bot_rows.to_owned()).map(|(top, bot)|
                                                                top.iter().zip(bot).map(|(&t, b)| 15_i8 - (t + b) as i8)
                                                                                    .collect::<Vec<i8>>())
                                                        .collect::<Vec<Vec<i8>>>();
//  Ignore cases where this produces negative numbers or the sum of the middle row is not 15                                                    
            let valid_mid_rows = mid_rows.iter().enumerate()
                                                    .filter(|test| test.1.iter().all(|x| x >= &0) && test.1.iter().sum::<i8>() == 15)
                                                    .collect::<Vec<(usize, &Vec<i8>)>>();

            if valid_mid_rows.len() == 0 {continue;}

//  Build valid squares, check no repeated members, then raise 2 to the corresponding power in each cell and sum all cells
            for (i, _v) in valid_mid_rows {
                let square = vec![top_rows[i].to_owned(), mid_rows[i].iter().map(|&x| x as u8).collect::<Vec<u8>>(), bot_rows[i].to_owned()];
                let members = square.iter().flatten().collect::<HashSet<&u8>>();
                if members.len() < 9 {continue;}
                let total = members.iter().map(|&&p| 2_i32.pow(p as u32)).sum::<i32>();
                squares_dict.entry(total).or_insert(Vec::<Vec<Vec<u8>>>::new()).push(square);
            }
        }
    }
    println!("The totals in ascending order are:");
    for (total, squares) in squares_dict {
        println!("Total {} with squares {:?}", total, squares);
    }
}
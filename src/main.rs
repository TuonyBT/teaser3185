use itertools::Itertools;
use std::{collections::HashSet, vec};

const Y_PICK: [[usize; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

fn main() {

    let mut triplets = Vec::<HashSet<u8>>::new();

    for a in 1u8..5 {
        for b in (a + 1)..(16 - a) / 2 {
            let triplet = HashSet::from([a, b, 15 - a - b]);
            triplets.push(triplet);
        }
    }
//    println!("triplets: {:?}", triplets);

    let mut squares = Vec::<Vec<&HashSet<u8>>>::new();

    for (first_row, second_row) in triplets.iter().cartesian_product(&triplets) {
        let used = first_row.union(&second_row).map(|&z| z).collect::<HashSet<u8>>();
        if used.len() < 6 {continue;}
        let unused = triplets.iter().filter(|z| z.is_disjoint(&used)).collect::<Vec<&HashSet<u8>>>();
        if unused.len() == 0 {continue;}

        for third_row in unused.into_iter() {
            squares.push(vec![first_row, second_row, third_row]);
        }
    }
//    println!("Squares {:?}", squares);

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


//  for each x, find triplets (y_i, z_j, z_k) of complementary pairs such that their sum is 15

    for (x, yz) in yz_vec.into_iter().enumerate() {
        for triplet in yz.iter().combinations(3) {
            let top_rows = Y_PICK.iter().map(|idx| 
                                                    idx.iter().zip(triplet.to_owned()).map(|(&ix, &yz)| yz[ix])
                                                                .collect::<Vec<u8>>())
                                            .filter(|t| t.iter().sum::<u8>() == 15)
                                            .collect::<Vec<Vec<u8>>>();
            if top_rows.len() == 0 {continue;}
            let bot_rows = top_rows.iter().map(|candidate| 
                                                            candidate.iter().rev().map(|yz| 15_u8 - x as u8 - yz).collect::<Vec<u8>>())
                                                        .collect::<Vec<Vec<u8>>>();
            let mid_rows = top_rows.iter().zip(bot_rows.to_owned()).map(|(top, bot)|
                                                                top.iter().zip(bot).map(|(&t, b)| 15_i8 - (t + b) as i8)
                                                                                    .collect::<Vec<i8>>())
                                                        .collect::<Vec<Vec<i8>>>();
            let valid_mid_rows = mid_rows.iter().enumerate()
                                                    .filter(|test| test.1.iter().all(|x| x >= &0))
                                                    .collect::<Vec<(usize, &Vec<i8>)>>();

            if valid_mid_rows.len() == 0 {continue;}

//            println!("X {}", x);
//            println!("Triplet {:?}", triplet);
//            println!("Top Rows {:?}", top_rows);
//            println!("Mid Rows {:?}", mid_rows);
//            println!("Bot Rows {:?}", bot_rows);
//            println!("Valid Mid Rows {:?}", valid_mid_rows);

            for (i, _v) in valid_mid_rows {
                let square = vec![top_rows[i].to_owned(), mid_rows[i].iter().map(|&x| x as u8).collect::<Vec<u8>>(), bot_rows[i].to_owned()];
                if square.iter().flatten().collect::<HashSet<&u8>>().len() < 9 {continue;}
                println!("     Possible Magic Square {:?}", square);

//  Need to swap y with one of the z's to get more squares that are not reflections or rotations
// *********************************************************************************************

            }
            println!();

        }

    }

}
use itertools::Itertools;
use std::collections::HashSet;

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
        println!("X {} YZ pairs {:?}", x, yz);
        for triplet in yz.iter().combinations(3) {
            let rows = Y_PICK.iter().map(|idx| idx.iter().zip(triplet.to_owned()).map(|(&ix, &yz)| yz[ix])
                                                                            .collect::<Vec<u8>>())
                                            .filter(|t| t.iter().sum::<u8>() == 15)
                                            .collect::<Vec<Vec<u8>>>();
            if rows.len() == 0 {continue;}
            println!("Triplet {:?}", triplet);
            println!("Rows {:?}", rows);

        }

    }

}
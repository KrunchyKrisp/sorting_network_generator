fn main() {
    println!("Hello, world!");
    network_generator(10, 10, false);
}

fn network_generator(n: usize, mut depth: usize, pair_wise: bool) -> Vec<Vec<usize>> {
    let (mut network, mut end);
    let (mut data, mut data_end, mut data_temp) = (vec![0;n], vec![0;n], vec![0;n]);
    let (mut counter, mut m_counter): (usize, usize) = (0, 0);

    loop {
        depth += 1;
        network = vec![vec![0; 2]; depth];
        end = vec![vec![0; 2]; depth];
        let mut k: i32 = 0;
        for i in 0..depth {
            k = (k % 2) + 1;
            end[i][0] = 0;
            end[i][1] = k;
        }
        println!("{:?}", end);
        break;
    }


    return network;
}
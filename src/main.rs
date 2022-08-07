use std::time::Instant;

fn main() {
    network_generator(5, 9, true);
}

fn execute_pair(data: &mut Vec<usize>, left: usize, right: usize) {
    if data[left] > data[right] {
        let temp = data[left];
        data[left] = data[right];
        data[right] = temp;
    }
}

fn increase(network: &mut Vec<Vec<usize>>, pos: usize, n: usize) {
    network[pos][1] += 1;
    if network[pos][1] > n - 1 {
        network[pos][0] += 1;
        network[pos][1] = network[pos][0] + 1;
        if network[pos][0] > n - 1 || network[pos][1] > n - 1 {
            network[pos][0] = 0;
            network[pos][1] = 1;
            if pos > 0 {
                increase(network, pos - 1, n);
            }
        }
    }
    if pos > 0 && network[pos] == network[pos - 1] {
        increase(network, pos, n);
    }
}

fn execute_network(network: & Vec<Vec<usize>>, data: &mut Vec<usize>) {
    for pair in network.iter() {
        execute_pair(data, pair[0], pair[1]);
    }
}

fn network_tester(n: usize, network: & Vec<Vec<usize>>) -> bool {
    let (mut data, data_end): (Vec<usize>, Vec<usize>) = (vec![0;n], vec![0;n]);
    let mut data_temp;
    loop {
        data_temp = data.to_vec();
        execute_network(& network, &mut data_temp);
        for i in 0..n-1 {
            if data_temp[i] > data_temp[i+1] {
                return false;
            }
        }
        for i in (0..n).rev() {
            if data[i] == 0 {
                data[i] = 1;
                break;
            }
            data[i] = 0;
        }
        if data == data_end {
            break;
        }
    }
    return true;
}

fn network_generator(n: usize, mut depth: usize, pair_wise: bool) -> Vec<Vec<usize>> {
    let now = Instant::now();
    let (mut network, mut end);
    let (mut counter, mut m_counter): (usize, usize) = (0, 0);

    loop {
        println!("Testing Depth: {}", depth);

        network = vec![vec![0; 2]; depth];
        end = vec![vec![0; 2]; depth];
        let mut k: usize = 0;
        for i in 0..depth {
            k = (k % 2) + 1;
            end[i][0] = 0;
            end[i][1] = k;
        }
        k = 0;
        let (mut i, mut j): (usize, usize) = (0, 1);
        if pair_wise {
            while i < depth && j < n {
                while i < depth && k < n - j {
                    
                    network[i][0] = k;
                    network[i][1] = k + j;

                    i += 1;
                    k += 2 * j;
                }
                j *= 2;
            }
        }
        while i < depth {
            k = (k % 2) + 1;
            network[i][0] = 0;
            network[i][1] = k;
            i += 1
        }
        println!("Initial network at depth {}: {:?}", depth, network);
        loop {
            if counter == 100_000_000 {
                m_counter += 1;
                println!("Testing {}00Mth: {:?}", m_counter, network);
                counter = 0;
            }
            counter += 1;
            
            if network_tester(n, & network) {
                println!("Network Found For n = {}: {:?}", n, network);
                println!("At depth {}", depth);
                println!("At Iteration {}00M + {}", m_counter, counter);
                println!("Total Time: {:?}", now.elapsed());
                return network;
            }
            increase(&mut network, depth - 1, n);

            if network == end {
                break;
            }
        }
        depth += 1;
    }
}
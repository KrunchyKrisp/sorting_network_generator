use std::time::Instant;

fn main() {
    //let starting = Some((Vec::from([[0, 1], [2, 3], [4, 5], [0, 1], [4, 5], [0, 2], [0, 5], [3, 5], [1, 4], [1, 3], [0, 5], [2, 3]]), 1380));
    let starting = None;
    network_generator::<7>(16, true, starting);
}

fn execute_pair<const N: usize>(data: &mut [usize; N], left: usize, right: usize) {
    if data[left] > data[right] {
        let temp = data[left];
        data[left] = data[right];
        data[right] = temp;
    }
}

fn increase<const N: usize>(network: &mut Vec<[usize; 2]>, pos: usize) {
    network[pos][1] += 1;
    if network[pos][1] > N - 1 {
        network[pos][0] += 1;
        network[pos][1] = network[pos][0] + 1;
        if network[pos][0] > N - 1 || network[pos][1] > N - 1 {
            network[pos][0] = 0;
            network[pos][1] = 1;
            if pos > 0 {
                increase::<N>(network, pos - 1);
            }
        }
    }
    if pos > 0 && network[pos] == network[pos - 1] {
        increase::<N>(network, pos);
    }
}

fn execute_network<const N: usize>(network: & Vec<[usize; 2]>, data: &mut [usize; N]) {
    for pair in network.iter() {
        execute_pair(data, pair[0], pair[1]);
    }
}

fn network_tester<const N: usize>(network: & Vec<[usize; 2]>) -> bool {
    let (mut data, data_end, mut data_temp): ([usize; N], [usize; N], [usize; N]) = ([0;N], [0;N], [0;N]);
    loop {
        data_temp.clone_from_slice(&data);
        execute_network(& network, &mut data_temp);
        for i in 0..N-1 {
            if data_temp[i] > data_temp[i+1] {
                return false;
            }
        }
        for i in (0..N).rev() {
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

fn network_generator<const N: usize>(mut depth: usize, pair_wise: bool, starting: Option<(Vec<[usize; 2]>, usize)>) -> Vec<[usize; 2]> {
    let now = Instant::now();
    let (mut network, mut end);
    let (mut counter, mut m_counter): (usize, usize) = (0, 0);

    loop {
        println!("Testing Depth: {}", depth);

        end = vec![[0; 2]; depth];
        let mut k: usize = 0;
        for i in 0..depth {
            k = (k % 2) + 1;
            end[i][0] = 0;
            end[i][1] = k;
        }
        
        if let Some(ref x) = starting {
            network = x.0.to_vec();
            m_counter = x.1;
        } else {
            network = vec![[0; 2]; depth];
            k = 0;
            let (mut i, mut j): (usize, usize) = (0, 1);
            if pair_wise {
                while i < depth && j < N {
                    while i < depth && k < N - j {
                        
                        network[i][0] = k;
                        network[i][1] = k + j;

                        i += 1;
                        k += 2 * j;
                    }
                    j *= 2;
                }
                j = 2;
                while i < depth && j < N {
                    network[i][0] = 0;
                    network[i][1] = j;
                    j *= 2;
                    i += 1;
                }
            }
            while i < depth {
                k = (k % 2) + 1;
                network[i][0] = 0;
                network[i][1] = k;
                i += 1
            }
        }
        
        println!("Initial network at depth {}: {:?}", depth, network);
        loop {
            if counter == 100_000_000 {
                m_counter += 1;
                println!("Testing {}00Mth: {:?}", m_counter, network);
                println!("Total Time: {:?}", now.elapsed());
                counter = 0;
            }
            counter += 1;
            
            if network_tester::<N>(& network) {
                println!("Network Found For N = {}: {:?}", N, network);
                println!("At depth {}", depth);
                println!("At Iteration {}00_{:0>6}", m_counter, counter);
                println!("Total Time: {:?}", now.elapsed());
                return network;
            }
            increase::<N>(&mut network, depth - 1);

            if network == end {
                break;
            }
        }
        depth += 1;
    }
}
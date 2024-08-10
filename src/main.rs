use std::time::Instant;
use num_format::{Locale, ToFormattedString};
use hhmmss::Hhmmss;

fn main() {





    network_generator(6, 12, true, None);
}

fn execute_pair(data: &mut Vec<usize>, left: usize, right: usize) {
    if data[left] > data[right] {
        let temp = data[left];
        data[left] = data[right];
        data[right] = temp;
    }
}

fn increase(network: &mut Vec<[usize; 2]>, n: usize, pos: usize, offset: usize) {
    network[pos][1] += 1;
    if network[pos][1] > n - 1 {
        network[pos][0] += 1;
        network[pos][1] = network[pos][0] + 1;
        if network[pos][0] > n - 1 || network[pos][1] > n - 1 {
            network[pos][0] = offset;
            network[pos][1] = offset + 1;
            if pos > 0 {
                increase(network, n, pos - 1, offset);
            }
        }
    }
    if pos > 0 && network[pos] == network[pos - 1] {
        increase(network, n, pos, offset);
    }
}

fn execute_network(network: & Vec<[usize; 2]>, data: &mut Vec<usize>) {
    for pair in network.iter() {
        execute_pair(data, pair[0], pair[1]);
    }
}

fn network_tester(network: & Vec<[usize; 2]>, n: usize, data: &mut Vec<usize>, data_temp: &mut Vec<usize>) -> bool {
    data.fill(0);
    loop {
        data_temp.copy_from_slice(&data);
        execute_network(& network, data_temp);
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
        if data.iter().sum::<usize>() == 0 {
            break;
        }
    }
    return true;
}

fn network_generator(n: usize, mut depth: usize, pair_wise: bool, starting: Option<(Vec<[usize; 2]>, usize)>) -> Vec<[usize; 2]> {
    let now = Instant::now();
    let mut network = Vec::new();
    let mut end;
    let (mut counter, mut m_counter): (usize, usize) = (0, 0);
    let offset: usize = if pair_wise {1} else {0};
    let mut generate: bool;
    let (mut data, mut data_temp): (Vec<usize>, Vec<usize>) = (vec![0;n], vec![0;n]);

    loop {
        println!("Testing Depth: {}", depth);

        end = vec![[0; 2]; depth];
        let mut k: usize = 0;
        for i in 0..depth {
            k = (k % 2) + 1;
            end[i][0] = offset;
            end[i][1] = offset + k;
        }
        generate = true;
        if let Some(ref x) = starting {
            if x.0.len() == depth {
                network = x.0.to_vec();
                m_counter = x.1;
                generate = false;
            }
        }
        if generate {
            network = vec![[0; 2]; depth];
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
                    k = 0;
                }
            } 
            k = 1;
            while i < depth {
                k = (k % 2) + 1;
                network[i][0] = offset;
                network[i][1] = offset + k;
                i += 1
            }
        }
        
        println!("Initial network at depth {}: {:?}", depth, network);
        loop {
            if counter == 100_000_000 {
                m_counter += 1;
                println!("Testing {} Mth: {:?}", (m_counter*100).to_formatted_string(&Locale::en), network);
                println!("Total Time: {}", now.elapsed().hhmmssxxx());
                counter = 0;
            }
            counter += 1;
            
            if network_tester(& network, n, &mut data, &mut data_temp) {
                println!("Network Found For n = {}: {:?}", n, network);
                println!("At depth {}", depth);
                println!("At Iteration {}", (m_counter*100_000_000 + counter).to_formatted_string(&Locale::en));
                println!("Total Time: {}", now.elapsed().hhmmssxxx());
                return network;
            }
            increase(&mut network, n, depth - 1, offset);

            if network == end {
                break;
            }
        }
        depth += 1;
    }
}
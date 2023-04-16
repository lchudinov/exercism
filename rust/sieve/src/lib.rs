pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut buffer = create_buffer(upper_bound);
    let mut marker_index: u64 = 2;
    while marker_index <= upper_bound && marker_index > 0 {
        cross_out_marker(&mut buffer, upper_bound, marker_index);
        marker_index = find_marker_index(&buffer, upper_bound, marker_index);
    }
    buffer.into_iter().filter(|&i| i > 0u64).collect()
}

fn create_buffer(upper_bound: u64) -> Vec<u64> {
    let v : Vec<u64> = vec![0; (upper_bound + 1).try_into().unwrap()];
    v.iter().enumerate().map(|(index, _)| if index > 1 { index as u64} else { 0 }).collect()
}

fn find_marker_index(buffer: &[u64], limit: u64, last_marker_index: u64) -> u64 {
    let mut i: u64 = last_marker_index + 1;
    while i <= limit {
        if buffer[i as usize] != 0 {
            return i;
        }
        i += 1;
    }
    0
}

fn cross_out_marker(buffer: &mut [u64], limit: u64, marker_index: u64)  {
    let step = buffer[marker_index as usize];
    let mut i: usize = marker_index as usize + step as usize;
    while i <= limit as usize {
        buffer[i] = 0;
        i += step as usize;
    }
}

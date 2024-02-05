/// Solve a system of two linear congruences using the Chinese Remainder Theorem.
/// See https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_(constructive_proof)
/// 
/// A system of two linear congruences is a pair of equations of the form:
/// 
/// x ≡ a1 (mod n1)
/// 
/// x ≡ a2 (mod n2)
/// 
/// where x is the unknown, a1 and a2 are the remainders, and n1 and n2 are the moduli.
/// 
/// # Arguments
/// 
/// * `a1` - The remainder of the first congruence.
/// * `n1` - The modulus of the first congruence.
/// * `a2` - The remainder of the second congruence.
/// * `n2` - The modulus of the second congruence.
/// 
/// # Returns
/// 
/// The solution to the system of congruences.
fn crt(a1: &i64, n1: &i64, a2: &i64, n2: &i64) -> i64 {
    let (m1, m2) = bézout(n1, &2);
    (a1 * m2 * n2 + a2 * m1 * n1) % (n1 * n2)
}


/// Compute the Bézout coefficients for a pair of integers.
/// See https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn bézout(a: &i64, b: &i64) -> (i64, i64) {
    let (mut r, mut r_prev) = (*a, *b);
    let (mut s, mut s_prev) = (1, 0);
    let (mut t, mut t_prev) = (0, 1);
    while r != 0 {
        let quotient = r_prev / r;  // integer division
        (r, r_prev) = (r_prev - quotient * r, r);
        (s, s_prev) = (s_prev - quotient * s, s);
        (t, t_prev) = (t_prev - quotient * t, t);
    }
    (s_prev, t_prev)
}

fn solve_congruences(remainders: &[i64], moduli: &[i64]) -> i64 {
    let mod_prod: i64 = moduli.iter().product();
    let solution: i128 = remainders.iter().zip(moduli.iter()).map( |(&rem_i, mod_i)| {
        let prod_without_i = mod_prod / mod_i;
        let (coeff, _) = bézout(mod_i, &prod_without_i);
        rem_i as i128 * prod_without_i as i128 * coeff as i128
    }).sum();
    (solution % mod_prod as i128) as i64
}



fn z_locs_within_cycle(input: &PuzzleInput, start_node: &usize, cycle_len: &usize) -> Vec<usize> {
    let mut map = input.map.clone();
    map.sort_by_key(|(s1, _, _)| s1.to_string());
    let nodes: Vec<Node> = map.iter().map(|(_, l, r)| {
        Node {
            left: map.binary_search_by_key(l, |(s, _, _)| s.to_string()).unwrap(),
            right: map.binary_search_by_key(r, |(s, _, _)| s.to_string()).unwrap(),
        }
    }).collect();
    let mut z_locs: Vec<usize> = vec![];
    let mut current_node = *start_node;
    for dir in &input.instructions {
        current_node = nodes[current_node].child(dir);
    }
    for (cycle, (i, dir)) in iproduct!(0..*cycle_len, input.instructions.iter().enumerate()) {
        if map[current_node].0.ends_with('Z') {
            let loc = cycle * input.instructions.len() + i;
            z_locs.push(loc);
        }
        current_node = nodes[current_node].child(dir);
    }
    z_locs
}

use std::io::stdin;

fn permutation(num: u32) -> Vec<Vec<u32>> {
    fn helper(nums: Vec<u32>, current: Vec<u32>, result: &mut Vec<Vec<u32>>) {
        if nums.is_empty() {
            result.push(current);
            return;
        }

        for i in 0..nums.len() {
            let mut next_nums = nums.clone();
            let mut next_current = current.clone();
            next_current.push(next_nums.remove(i));
            helper(next_nums, next_current, result);
        }
    }

    let nums: Vec<u32> = (1..=num).collect();
    let mut result = Vec::new();
    helper(nums, Vec::new(), &mut result);
    result
}

fn haruhi(num: u32) -> Vec<u32> {
    let mut perms = permutation(num); // Generate all permutations
    let mut res = Vec::new(); // Result vector
    let mut aux: u32 = 1; // Auxiliary counter for fallback digits
    let mut aux_404: u32 = 1; // Auxiliary counter for

    // Initialize `res` with the first `num` digits
    for i in 1..=num {
        res.push(i);
    }

    // Remove the initial permutation from `perms`
    perms.retain(|perm| perm != &res);

    while !perms.is_empty() {
        let mut found_match = false;
        let mut failed_match = false;

        // Add the next digit to `res`
        res.push(aux);

        // Evaluate only the last `num` digits of `res`
        let last_n_digits = &res[res.len().saturating_sub(num as usize)..];

        // Check if the last `num` digits match any permutation
        if perms.iter().any(|perm| perm == last_n_digits) {
            // If a match is found, remove the matching permutation
            perms.retain(|perm| perm != last_n_digits);
            found_match = true;
        } else {
            // If no match is found, replace the last digit with the next value
            res.pop(); // Remove the last digit
            aux += 1;
            if aux > num {
                aux = 1;
                failed_match = true 
            }
        }

        if found_match {
            aux = 1; // Reset aux to 1 when a match is found
        }

        if failed_match {
            if num>3{
            if &res[res.len().saturating_sub((num+3) as usize)..] == vec![num..3].append(&mut vec![1,2,1]).as_slice() {
                let mut aux_vec = res.clone();
                aux_vec.pop();
                aux_vec.pop();
                aux_vec.pop();
                aux_vec = aux_vec.iter().rev().map(|&x| x).collect();
                for i in aux_vec {
                    res.push(i);
                }
                break;
            }}
            res.push(aux_404);
            aux_404 += 1;
            if aux_404 > (num/2)+1 {
                aux_404 = 1; // Reset aux_404 to 1 when it exceeds num/2
            }
        } 

        println!("Temp Result: {:?}", res);
        println!("Remaining Permutations: {:?}", perms.len());
    }

    res
}

fn main() {
    let mut inp = String::new();

    println!("Enter a number: ");
    stdin().read_line(&mut inp).unwrap();

    let num = inp.trim().parse::<u32>().expect("Please enter a valid number");


    let res = haruhi(num);
    println!("Final: {:?}", res);
    println!("Final Lenght: {}", res.len());
}

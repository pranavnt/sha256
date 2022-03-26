// I was bored at a coffee shop and didn't want to do homework, so I read into how sha256 works and am trying to implement it from scratch
// This should go without saying, but please don't use this anywhere
// massive credit to https://en.wikipedia.org/wiki/SHA-2

fn main() {
    sha256("Pranav")
}

fn sha256(input: &str) {
    // step 1: convert to binary
    let mut binary_string = String::new();

    for char in input.to_string().clone().into_bytes() {
        binary_string += &format!("0{:b} ", char);
    }

    // step 2: add one "1" to the end of the string
    let mut padded_binary_string = binary_string.clone() + "1";

    // step 3: append zeros to the end of the string until it is a multiple of 512 minus 64
    while padded_binary_string.len() % 512 != 448 {
        padded_binary_string += "0";
    }

    // step 4: for the final 64 bits, append the length of the original string
    let mut length_of_string_in_binary = String::new();

    for char in binary_string.len().to_string().clone().into_bytes() {
        length_of_string_in_binary += &format!("0{:b} ", char);
    }

    padded_binary_string += &length_of_string_in_binary;

    // step 5: Initialize the 8 hash values
    // how to get these:
    // 1. Get the first 8 primes (2, 3, 5, 7, 11, 13, 17, 19)
    // 2. take the decimal part of their square roots
    // 3. multiple their square roots by 2^32
    // 4. convert to hex
    let mut hash_values: Vec<u32> = vec![
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    // step 6: initialize these constants (will use later and go into more detail then)
    // how to get these constants:
    // 1. Get the first 64 primes (all primes between 2 to 311)
    // 2. take the decimal part of their cube roots
    // 3. multiple their cube roots by 2^32
    // 4. convert to hex
    let constants: Vec<u32> = vec![
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    // step 7: break up the string into 512-bit chunks and iterate through them
    for i in 0..padded_binary_string.len() / 512 {
        let chunk = padded_binary_string[i * 512..(i + 1) * 512].to_string();

        // step 9 create the message schedule (64 32 bit elements)
        let mut message_schedule: Vec<u32> = vec![0; 64];

        // step 10: break up the chunk into 16 32 bit words
        for j in 0..16 {
            let mut temp = chunk[j * 32..(j + 1) * 32].to_string();
            temp.truncate(32);
            message_schedule[j] = u32::from_str_radix(&temp, 2).unwrap();
        }

        for j in 16..64 {
            let s0 = (message_schedule[j - 15].rotate_right(7))
                ^ (message_schedule[j - 15].rotate_right(18))
                ^ (message_schedule[j - 15] >> 3);
            let s1 = (message_schedule[j - 2].rotate_right(17))
                ^ (message_schedule[j - 2].rotate_right(19))
                ^ (message_schedule[j - 15] >> 10);
            message_schedule[j] = message_schedule[j - 16] + s0 + message_schedule[j - 7] + s1;
        }

        let mut a = hash_values[0];
        let mut b = hash_values[1];
        let mut c = hash_values[2];
        let mut d = hash_values[3];
        let mut e = hash_values[4];
        let mut f = hash_values[5];
        let mut g = hash_values[6];
        let mut h = hash_values[7];

        for j in 0..64 {
            let S1 = (e.rotate_right(6)) ^ (e.rotate_right(11)) ^ (e.rotate_right(25));
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h + S1 + ch + constants[i] + message_schedule[i];

            let S0 = (a.rotate_right(2)) ^ (a.rotate_right(13)) ^ (a.rotate_right(22));
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = S0 + maj;

            h = g;
            g = f;
            f = e;
            e = d + temp1;
            d = c;
            c = b;
            b = a;
            a = temp1 + temp2;
        }

        hash_values = vec![
            hash_values[0] + a,
            hash_values[1] + b,
            hash_values[2] + c,
            hash_values[3] + d,
            hash_values[4] + e,
            hash_values[5] + f,
            hash_values[6] + g,
            hash_values[7] + h,
        ];

    }
    
    let mut final_hash = String::new();

    for hash in hash_values {
        final_hash.push_str(&format!("{:x}", hash));
    }

    println!("{}", final_hash);
}

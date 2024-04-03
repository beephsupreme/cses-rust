/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::chessboard_and_queens::solve;
use cses::utils::io::get_vector_vector_bool;

fn main() {
    let b = get_vector_vector_bool();
    let r: usize = solve(0, &mut [0usize; 8], &b);
    eprintln!("{}", r);
}

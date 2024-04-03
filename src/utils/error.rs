/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */


use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CsesError {
    details: String
}

impl CsesError {
    pub fn new(msg: &str) -> CsesError {
        CsesError {details: msg.to_string()}
    }
}

impl fmt::Display for CsesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CsesError {
    fn description(&self) -> &str {
        &self.details
    }
}

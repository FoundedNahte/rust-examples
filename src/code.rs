#![allow(overflowing_literals)]

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut output = "";
        for i in shifts {
            for j in s {
                output += (j as i32 + i) as string;
            }
        }

        return let slice = &output[shifts.size()
    }
}


impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut output = "";
        for i in shifts {
            for j in s.chars() {
                output += (j as i32 + i) as char;
            }
        }

        return output[shifts.len()..]
    }
}

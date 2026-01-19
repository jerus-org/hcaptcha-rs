// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use hcaptcha::Hcaptcha;

#[derive(Hcaptcha)]
struct Test {
    Hcaptcha: String,
}

fn main() {
    println!("Super!");
}
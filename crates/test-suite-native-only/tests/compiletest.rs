// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compilefail/*.rs");
    t.pass("tests/ui/pass/test_*.rs");
}

// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

type item_ty_yes0 = {
    x: &uint //~ ERROR anonymous region types are not permitted here
};

type item_ty_yes1 = {
    x: &'self uint
};

type item_ty_yes2 = {
    x: &'foo uint //~ ERROR named regions other than `self` are not allowed as part of a type declaration
};

fn main() {}

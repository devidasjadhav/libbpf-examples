// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Copyright (c) 2021 BMC Software, Inc.
// Author Devasia Thomas <https://www.linkedin.com/in/devasiathomas/>
//
// Based on capable(8) by Brendan Gregg

use anyhow::{bail, Result};

#[path = "../.output/hello.skel.rs"]
mod hello;
use hello::*;


fn bump_memlock_rlimit() -> Result<()> {
    let rlimit = libc::rlimit {
        rlim_cur: 128 << 20,
        rlim_max: 128 << 20,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
        bail!("Failed to increase rlimit");
    }

    Ok(())
}


fn main() -> Result<()> {
    let skel_builder = HelloSkelBuilder::default();

    bump_memlock_rlimit()?;

    let  open_skel = skel_builder.open()?;
    //Pass configuration to BPF

    let mut skel = open_skel.load()?;
    skel.attach()?;
    println!("Waiting for ^c..");
    loop {
    }
}


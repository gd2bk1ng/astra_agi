// =============================================================================
//  Astra AGI
//  File: main.rs
//
//  Description: Optional binary entry point (CLI or runtime)
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2025-12-23
//
//  //  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use astra_agi::runtime::executor::Executor;

fn main() {
    let mut executor = Executor::new();
    executor.execute("print('Hello, Astra AGI!')").unwrap();
}

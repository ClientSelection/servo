/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Configuration options for a single run of the servo application. Created
//! from command line arguments.

use azure::azure_hl::{BackendType, CairoBackend, CoreGraphicsBackend};
use azure::azure_hl::{CoreGraphicsAcceleratedBackend, Direct2DBackend, SkiaBackend};

use std::result;

#[deriving(Clone)]
pub struct Opts {
    urls: ~[~str],
    render_backend: BackendType,
    n_render_threads: uint,
    tile_size: uint,
    profiler_period: Option<f64>,
    exit_after_load: bool,
    output_file: Option<~str>,
    headless: bool,
}

pub fn from_cmdline_args(args: &[~str]) -> Opts {
    use extra::getopts;

    let args = args.tail();

    let opts = ~[
        getopts::optopt("o"),  // output file
        getopts::optopt("r"),  // rendering backend
        getopts::optopt("s"),  // size of tiles
        getopts::optopt("t"),  // threads to render with
        getopts::optflagopt("p"),  // profiler flag and output interval
        getopts::optflag("x"), // exit after load flag
        getopts::optflag("z"), // headless mode
    ];

    let opt_match = match getopts::getopts(args, opts) {
      result::Ok(m) => m,
      result::Err(f) => fail!(f.to_err_msg()),
    };

    let urls = if opt_match.free.is_empty() {
        fail!(~"servo asks that you provide 1 or more URLs")
    } else {
        opt_match.free.clone()
    };

    let render_backend = match opt_match.opt_str("r") {
        Some(backend_str) => {
            if backend_str == ~"direct2d" {
                Direct2DBackend
            } else if backend_str == ~"core-graphics" {
                CoreGraphicsBackend
            } else if backend_str == ~"core-graphics-accelerated" {
                CoreGraphicsAcceleratedBackend
            } else if backend_str == ~"cairo" {
                CairoBackend
            } else if backend_str == ~"skia" {
                SkiaBackend
            } else {
                fail!(~"unknown backend type")
            }
        }
        None => SkiaBackend
    };

    let tile_size: uint = match opt_match.opt_str("s") {
        Some(tile_size_str) => from_str(tile_size_str).unwrap(),
        None => 512,
    };

    let n_render_threads: uint = match opt_match.opt_str("t") {
        Some(n_render_threads_str) => from_str(n_render_threads_str).unwrap(),
        None => 1,      // FIXME: Number of cores.
    };

    // if only flag is present, default to 5 second period
    let profiler_period = do opt_match.opt_default("p", "5").map |period| {
        from_str(period).unwrap()
    };

    Opts {
        urls: urls,
        render_backend: render_backend,
        n_render_threads: n_render_threads,
        tile_size: tile_size,
        profiler_period: profiler_period,
        exit_after_load: opt_match.opt_present("x"),
        output_file: opt_match.opt_str("o"),
        headless: opt_match.opt_present("z"),
    }
}

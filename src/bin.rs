// Copyright 2016 The Perceptia Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Wayland protocol generator for testing purposes.

#![warn(missing_docs)]

extern crate skylane_scanner;

use std::env;
use std::path::Path;

fn main() {
    let args = env::args();
    if args.len() > 1 {
        for arg in args.skip(1) {
            let path = Path::new(&arg);
            let mut scanner =
                skylane_scanner::Scanner::new(&path)
                    .expect(format!("Initialize scanner for file {:?}", &path).as_str());

            println!("pub mod {} {{\n\n    pub mod server {{",
                     scanner.get_protocol_name().expect("Extracting protocol name"));
            println!("{}", scanner.generate_server_interface(2));
            println!("    }}\n\n    pub mod client {{");
            println!("{}", scanner.generate_client_interface(2));
            println!("    }}\n}}");

        }
    } else {
        println!("No args ware given!");
    }
}

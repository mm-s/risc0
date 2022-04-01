// Copyright 2022 Risc0, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::{Arg, Command};
use lib::{sign, SignatureWithReceipt};

fn main() {
    let matches = Command::new("sign")
        .version("0.1.0")
        .author("Risc0, Inc.")
        .about("Digital signatures with Risc0")
        .arg(
            Arg::new("msg")
                .long("msg")
                .takes_value(true)
                .help("Message"),
        )
        .arg(
            Arg::new("pass")
                .long("pass")
                .takes_value(true)
                .help("Passphrase"),
        )
        .get_matches();

    let msg_str = matches.value_of("msg").unwrap();
    let pass_str = matches.value_of("pass").unwrap();

    // Generate the signature
    let signing_receipt = sign(&pass_str, &msg_str).unwrap();
    let raw = signing_receipt.to_raw().unwrap();
    let deserialized_receipt = SignatureWithReceipt::from_raw(&raw).unwrap();

    println!("{:?}", &raw);

    log::info!(
        "msg_raw.len() = {:?}; core_raw.len() = {:?}",
        raw.journal.len(),
        raw.seal.len()
    );
    log::info!("commit: {:?}", &deserialized_receipt.verify().unwrap());
}

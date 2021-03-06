//! https://github.com/lumen/otp/tree/lumen/lib/parsetools/src

use super::*;

test_compiles_lumen_otp!(leex);
test_compiles_lumen_otp!(yecc);
test_compiles_lumen_otp!(yeccparser);
test_compiles_lumen_otp!(yeccscan imports "lib/stdlib/src/erl_scan", "lib/stdlib/src/io");

fn relative_directory_path() -> PathBuf {
    super::relative_directory_path().join("parsetools/src")
}

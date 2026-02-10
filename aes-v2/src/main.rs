use aes_v2::{AES128, State};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    // Read 16-byte key from stdin
    let mut key = [0u8; 16];
    stdin.read_exact(&mut key)?;

    let aes = AES128::new(key);
    let mut block = [0u8; 16];

    loop {
        let n = stdin.read(&mut block)?;
        if n == 0 {
            break;
        }
        if n < 16 {
            break;
        }

        let mut state = State::new(block);
        aes.cipher(&mut state);
        stdout.write_all(state.as_bytes())?;
    }

    Ok(())
}

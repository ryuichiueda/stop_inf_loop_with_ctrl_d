use nix::{unistd, fcntl, Error};

fn main() -> Result<(), Box<Error>> {
    let mut ch = [0;4];
    loop {
        fcntl::fcntl(2, nix::fcntl::F_SETFL(nix::fcntl::OFlag::O_NONBLOCK))?;
        if let Ok(n) = unistd::read(2, &mut ch) { //ノンブロックでread呼び出し
            let s = String::from_utf8(ch[..n].to_vec()).unwrap();
            if s == "" { //break with CTRL+D
                 eprintln!("^D");
                 break;
            }
        }
        //出力するときはO_SYNCにしておかないと止まる（上田の理解不足によりだれか理由おしえて）
        fcntl::fcntl(2, nix::fcntl::F_SETFL(nix::fcntl::OFlag::O_SYNC))?;
        println!("わはははは");
    }

    println!("ループから出た"); //Ctrl+Dだとこれが表示される。（Ctrl+Cだと途中で終わるから出ない。）
    Ok(())
}

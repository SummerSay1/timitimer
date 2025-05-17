use clap::*;
use std::io::Write;
use std::time::Duration;
use std::{thread, usize};

fn main() {
    let mut dsconst: usize = 0;
    let (mut dsh, mut dsm, mut dss): (usize, usize, usize) = (0, 0, 0);
    let mut dsmi: usize;
    let loading = vec!['/', '-', '\\', '|'];

    let mut direction: usize = 0;

    loop {
        dsmi = dsconst % 1000;

        dsh = dsconst / 3600000;
        dsm = dsconst / 60000;
        dss = dsconst / 1000;

        //基础运算
        if dsm >= 60 {
            dsm = dsm - dsm / 60 * 60
        }

        if dss >= 60 {
            dss = dss - dss / 60 * 60
        }

        //print
        if dsmi < 10 {
            print!(
                "\r {}:{}:{}.00{} {}",
                dsh, dsm, dss, dsmi, loading[direction]
            );
        } else if dsmi >= 10 && dsmi < 100 {
            print!(
                "\r {}:{}:{}.0{} {}",
                dsh, dsm, dss, dsmi, loading[direction]
            );
        } else {
            print!("\r {}:{}:{}.{} {}", dsh, dsm, dss, dsmi, loading[direction]);
        }
        std::io::stdout().flush().unwrap();

        direction = direction + 1;
        if direction == 4 {
            direction = 0;
        }
        dsconst = dsconst + 1;
        thread::sleep(Duration::from_millis(1));
    }
}

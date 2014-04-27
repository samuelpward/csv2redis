extern crate csv;

use std::path::Path;
use std::os;
use std::io::Process;
use std::io::pipe::PipeStream;


fn main() {
  let args = os::args();

  //  Opens and decodes csv file (with header)
  let fp = &Path::new(args[1]);
  let mut rdr = csv::Decoder::from_file(fp);
  rdr.has_headers(true);

  //  Start redis-cli --pipe
  let mut child = match Process::new("redis-cli", ["--pipe".to_owned()]) {
    Ok(child) => child,
    Err(e) => fail!("Failed. Do you have redis installed?"),
  };

  //  stdin pipe for redis-cli
  let pipe = child.stdin.get_mut_ref();

  for (date, open, high, low, close, volume, adj) in rdr.decode_iter::<(~str, f32, f32, f32, f32, uint, f32)>() {
      pipe.write_line(format!("SET {} {}", date, close));
    }

}

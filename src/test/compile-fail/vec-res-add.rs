// xfail-test #2587
// error-pattern: copying a noncopyable value

struct r {
  let i:int;
  new(i:int) {self.i = i;}
}

impl r : Drop {
    fn finalize() {}
}

fn main() {
    // This can't make sense as it would copy the classes
    let i = move ~[r(0)];
    let j = move ~[r(1)];
    let k = i + j;
    log(debug, j);
}

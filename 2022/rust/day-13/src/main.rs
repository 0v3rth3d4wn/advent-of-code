use serde_json::Value;
use std::cmp::Ordering;
use std::fs;
// def cmp(a, b)
//   case [a, b]
//   in Integer, Integer then a <=> b
//   in Array, Array then a.take(b.size).zip(b).lazy.map { cmp _1, _2 }.detect { _1 != 0 } || a.size <=> b.size
//   else cmp Array(a), Array(b)
//   end
// end

fn cmp(lr: (&Value, Value)) -> Ordering {
    // let (left, right) = lr;

    match (lr.0, &lr.1) {
        (Value::Number(l), Value::Number(r)) => l.as_u64().cmp(&r.as_u64()),
        (Value::Array(l), Value::Array(r)) => if l.to_vec().iter().zip(r.to_vec()).map(|(l, r)| cmp((l,r))).find(|e| *e == Ordering::Less) == None {
            lr.0.as_u64().cmp(&lr.1.as_u64())
        } else {
            Ordering::Greater
        }, 
        _ => cmp(lr)
        // lr if lr.0.is_u64() && lr.1.is_u64() => println!("{:?}", lr),
        // _ => cmp(),
    }
}
fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let pairs: Vec<&str> = file.trim_end().split("\n\n").collect();

    for pair in pairs {
        let (pair_left, pair_right): (&str, &str) = pair.split_once("\n").unwrap();
        let pair_left = serde_json::from_str::<Vec<Value>>(pair_left).unwrap();
        let pair_right = serde_json::from_str::<Vec<Value>>(pair_right).unwrap();

        for lr in pair_left.iter().zip(pair_right) {
            cmp(lr);
        }
        // for mut e in pair_left {
        //     println!("{}", e.take());
        // }
        //
    }
}

use std::{concat, str::FromStr};

use criterion::{BenchmarkId, Criterion};
use radix_engine_interface::math::integer::{NthRoot,I256,I512};
use num_traits::Pow;
use num_bigint::BigInt;
use rug::{Integer, ops::Pow as RugPow};
use radix_engine_interface::math::bnum_integer::{NthRoot as BnumNthRoot,BnumI256,BnumI512};

use crate::{ops_fn,bench_ops,process_op};
use crate::macros::QUICK;

const ADD_OPERANDS: [(&str, &str); 4] = [
    ("278960446186580977117854925043439539266349000000000000000000000000000000000", "278960446186580977117854925043439539266349000000000000000000000000000000000"),
    ("-278960446186580977117854925043439539266349000000000000000000000000000000000", "278960446186580977117854925043439539266349000000000000000000000000000000000"),
    ("1", "-1"),
    ("-278960446186580977117854925043439539266349000000000000000000000000000000000", "-278960446186580977117854925043439539266349000000000000000000000000000000000"),
];

const SUB_OPERANDS: [(&str, &str); 4] = [
    ("278960446186580977117854925043439539266349000000000000000000000000000000000", "278960446186580977117854925043439539266349000000000000000000000000000000000"),
    ("-278960446186580977117854925043439539266349000000000000000000000000000000000", "278960446186580977117854925043439539266349000000000000000000000000000000000"),
    ("1", "-1"),
    ("-278960446186580977117854925043439539266349000000000000000000000000000000000", "-278960446186580977117854925043439539266349000000000000000000000000000000000"),
];

const MUL_OPERANDS: [(&str, &str); 4] = [
    ("278960446186580977117854925043439539", "2789604461865809771178549250434395392"),
    ("-278960446186580977117854925043439539", "2789604461865809771178549250434395392"),
    ("634992332820282019728", "131231233"),
    ("-123123123123", "-1"),
];

const DIV_OPERANDS: [(&str, &str); 4] = [
    ("278960446186580977117854925043439539", "2789604461865809771178549250434395392"),
    ("-278960446186580977117854925043439539", "2789604461865809771178549250434395392"),
    ("634992332820282019728", "131231233"),
    ("-123123123123", "-1"),
];

const ROOT_OPERANDS: [(&str, &str); 4] = [
    ("57896044618658097711785492504343953926634992332820282019728","17"),
    ("12379879872423987", "13"),
    ("12379879872423987", "5"),
    ("9", "2"),
];

const POW_OPERANDS: [(&str, &str); 4] = [
    ("12", "13"),
    ("1123123123", "5"),
    ("4", "5"),
    ("9", "2"),
];

const TO_STRING_OPERANDS: [&str; 4] = [
    "578960446186580977117854925043439539266349923328202820197792003956564819967",
    "-112379878901230908903281928379813",
    "12379879872423987123123123",
    "9",
];

const FROM_STRING_OPERANDS: [&str; 4] = [
    "578960446186580977117854925043439539266349923328202820197792003956564819967",
    "-112379878901230908903281928379813",
    "12379879872423987123123123",
    "9",
];

ops_fn!(I256, nth_root, pow, u32);
bench_ops!(I256, "add");
bench_ops!(I256, "sub");
bench_ops!(I256, "mul");
bench_ops!(I256, "div");
bench_ops!(I256, "root", u32);
bench_ops!(I256, "pow", u32);
bench_ops!(I256, "to_string");
bench_ops!(I256, "from_string");

ops_fn!(I512, nth_root, pow, u32);
bench_ops!(I512, "add");
bench_ops!(I512, "sub");
bench_ops!(I512, "mul");
bench_ops!(I512, "div");
bench_ops!(I512, "root", u32);
bench_ops!(I512, "pow", u32);
bench_ops!(I512, "to_string");
bench_ops!(I512, "from_string");

ops_fn!(BigInt, nth_root, pow, u32);
bench_ops!(BigInt, "add");
bench_ops!(BigInt, "sub");
bench_ops!(BigInt, "mul");
bench_ops!(BigInt, "div");
bench_ops!(BigInt, "root", u32);
bench_ops!(BigInt, "pow", u32);
bench_ops!(BigInt, "to_string");
bench_ops!(BigInt, "from_string");

ops_fn!(Integer, root, pow, u32, "clone");
bench_ops!(Integer, "add");
bench_ops!(Integer, "sub");
bench_ops!(Integer, "mul");
bench_ops!(Integer, "div");
bench_ops!(Integer, "root", u32);
bench_ops!(Integer, "pow", u32);
bench_ops!(Integer, "to_string");
bench_ops!(Integer, "from_string");

ops_fn!(BnumI256, nth_root, pow, u32);
bench_ops!(BnumI256, "add");
bench_ops!(BnumI256, "sub");
bench_ops!(BnumI256, "mul");
bench_ops!(BnumI256, "div");
bench_ops!(BnumI256, "root", u32);
bench_ops!(BnumI256, "pow", u32);
bench_ops!(BnumI256, "to_string");
bench_ops!(BnumI256, "from_string");

ops_fn!(BnumI512, nth_root, pow, u32);
bench_ops!(BnumI512, "add");
bench_ops!(BnumI512, "sub");
bench_ops!(BnumI512, "mul");
bench_ops!(BnumI512, "div");
bench_ops!(BnumI512, "root", u32);
bench_ops!(BnumI512, "pow", u32);
bench_ops!(BnumI512, "to_string");
bench_ops!(BnumI512, "from_string");

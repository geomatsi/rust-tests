fn main() {

	// bug: vars immutable by default
	let x = 1;
	println!("value: {}", x);
	//x = 2;
	println!("value: {}", x);

	// ok: declare mutable var
	let mut x = 3;
	println!("value: {}", x);
	x = 4;
	println!("value: {}", x);

	// ok: shadowing vars
	let x = 5;
	println!("value: {}", x);
	let x = x + 1;
	println!("value: {}", x);

	// no way to infer int type here: need to specify one
	let guess: u32 = "42".parse().expect("not a number...");
	println!("int: {}", guess);

	// number literals
	let a1 = 10_100;	// visual separator
	let a2 = 0xff;		// hex
	let a3 = 0b1100;	// bin
	let a4 = 44u8;		// type suffix
	println!("{} {} {} {}", a1, a2, a3, a4);

	// floating point: IEEE-754
	let f1 = 2.64;		// default: f64
	let f2: f32 = 2.32;	// f32
	println!("{} {}", f1, f2);

	// chars: Unicode Scalar Values
	let c1 = 'z';
	let c2: char = 'Z';
	let c3 = '\u{556}';
	println!("{} {} {}", c1, c2, c3);

	// compound type: tuples
	let tup: (i32, f32, char) = (500, 3.3, '\u{444}');
	println!("{} {}", tup.0, tup.2);

	// compound type: array
	let a = [1, 2, 4, 7, 11];	// fixed length storage on stack
	println!("{} {}", a[1], a[3]);
	//println!("{}", a[100]);	// will panic at runtime
}

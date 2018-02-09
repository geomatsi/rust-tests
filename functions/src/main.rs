// functions and simple control flow

fn main() {

	println!("{}", f_min(1,2));
	println!("{}", f_min(2,1));
	println!("{}", f_min(1,1));

	println!("{}", f_fib_rec(1));
	println!("{}", f_fib_rec(3));
	println!("{}", f_fib_rec(10));

	println!("{}", f_fib_norec(1));
	println!("{}", f_fib_norec(3));
	println!("{}", f_fib_norec(10));

	println!("pi_s ~ {}", f_pi_s(1));
	println!("pi_s ~ {}", f_pi_s(10));
	println!("pi_s ~ {}", f_pi_s(1000));
	println!("pi_s ~ {}", f_pi_s(10000));
	println!("pi_s ~ {}", f_pi_s(100000));

	println!("pi_x ~ {}", f_pi_x(1));
	println!("pi_x ~ {}", f_pi_x(10));
	println!("pi_x ~ {}", f_pi_x(1000));
	println!("pi_x ~ {}", f_pi_x(10000));
	println!("pi_x ~ {}", f_pi_x(100000));

}

// min
fn f_min(x1: i32, x2: i32) -> i32 {

	if x1 <= x2 {
		x1
	} else {
		x2
	}
}

#[test]
fn f_min_test() {
	assert_eq!(f_min(1,1), 1);
	assert_eq!(f_min(1,2), 1);
	assert_eq!(f_min(2,1), 1);
}

// fib recursion
fn f_fib_rec(n: u32) -> u32 {

	if n == 0 {
		1
	} else if n == 1 {
		1
	} else {
		f_fib_rec(n-1) + f_fib_rec(n-2)
	}
}

#[test]
fn f_fib_rec_test() {
	assert_eq!(f_fib_rec(0), 1);
	assert_eq!(f_fib_rec(1), 1);
	assert_eq!(f_fib_rec(2), 2);
	assert_eq!(f_fib_rec(3), 3);
	assert_eq!(f_fib_rec(4), 5);
	assert_eq!(f_fib_rec(5), 8);
}

// fib w/o recursion
fn f_fib_norec(n: u32) -> u32 {
	let mut n1 = 0;
	let mut n2 = 1;

	for _ in 0..n {
		n2 = n1 + n2;
		n1 = n2 - n1;
	}

	n2
}

#[test]
fn f_fib_norec_test() {
	assert_eq!(f_fib_norec(0), 1);
	assert_eq!(f_fib_norec(1), 1);
	assert_eq!(f_fib_norec(2), 2);
	assert_eq!(f_fib_norec(3), 3);
	assert_eq!(f_fib_norec(4), 5);
	assert_eq!(f_fib_norec(5), 8);
}

// Pi approximation: Gregory-Leibniz series
fn f_pi_s(n: u32) -> f64 {
	let mut f: f64 = 0.0;
	let mut s: f64 = 1.0;
	let mut k: f64 = 0.0;

	for _ in 0 .. n {
		f = f + s/(2.0*k + 1.0);
		s = -s;
		k = k + 1.0;
	}

	4.0*f
}

// Pi approx: BBP formula (Bailey, Borwein, Plouffe)
fn f_pi_x(n: u32) -> f64 {
	let mut f: f64 = 0.0;
	let mut m: f64 = 1.0;
	let mut k: f64 = 0.0;

	for _ in 0 .. n {
		f = f + m*(
			4.0/(8.0*k + 1.0) -
			2.0/(8.0*k + 4.0) -
			1.0/(8.0*k + 5.0) -
			1.0/(8.0*k + 6.0));
		m = m/16.0;
		k = k + 1.0;
	}

	f
}

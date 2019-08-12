trait Printable {
    fn stringify(&self) -> String;
}

struct Foo(u32);

impl Printable for Foo {
    fn stringify(&self) -> String {
        self.0.to_string()
    }
}

struct Bar(f32);

impl Printable for Bar {
    fn stringify(&self) -> String {
        self.0.to_string()
    }
}

// intentionally, for test purposes, one field is mutable and another one is not
struct FooBar<'a> {
    a: &'a mut dyn Printable,
    b: &'a dyn Printable,
}

impl<'a> FooBar<'a> {
    pub fn init(a: &'a mut dyn Printable, b: &'a dyn Printable) -> FooBar<'a> {
        FooBar { a, b }
    }
}

impl<'a> Printable for FooBar<'a> {
    fn stringify(&self) -> String {
        format!("{}_{}", self.a.stringify(), self.b.stringify())
    }
}

fn print(a: &dyn Printable) {
    println!("{}", a.stringify());
}

fn main() {
    let mut foo = Foo(10);
    let bar = Bar(5.0);

    let foobar = FooBar::init(&mut foo, &bar);

    print(&foobar);

    println!("run tests: cargo test --bin test-traits");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn f_test_n1() {
        assert_eq!(Foo(10).stringify(), "10");
    }

    #[test]
    fn f_test_n2() {
        assert_eq!(Bar(10.0).stringify(), "10");
    }

    #[test]
    fn f_test_n3() {
        assert_eq!(FooBar::init(&mut Foo(10), &Bar(5.0)).stringify(), "10_5");
    }

    #[test]
    fn f_test_n4() {
        let mut foo = Foo(100);
        let bar = Bar(5.0);
        let foobar = FooBar::init(&mut foo, &bar);

        assert_eq!(foobar.stringify(), "100_5");
        assert_eq!(foo.stringify(), "100");
        assert_eq!(bar.stringify(), "5");
    }
}

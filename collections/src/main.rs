use std::collections::HashMap;

#[derive(Debug)]
enum MixedType {
    IVar(i32),
    FVar(f32),
}

#[derive(Debug)]
enum TestResult {
    Skip,
    Pass,
    Fail,
}

fn main() {
    // simple vectors

    let mut v1 = vec![1, 2, 3];
    let mut v2: Vec<i32> = Vec::new();

    v1.push(0);
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);

    v2.push(30);
    v2.push(20);
    v2.push(10);

    dump("v1", &v1);
    dump("v2", &v2);

    println!("{:?}", v1.get(3));
    println!("{:?}", v1.get(5));

    v1[0] = 10;
    dump("v1", &v1);

    update(&mut v1);
    dump("v1", &v1);

    // more complicated vector type
    let mut v3: Vec<MixedType> = Vec::new();

    v3.push(MixedType::IVar(1));
    v3.push(MixedType::IVar(2));
    v3.push(MixedType::FVar(3.0));

    for e in &v3 {
        println!("v3: {:?}", e);
    }

    for e in vec![0, 1, 2, 3, 4] {
        let v = v3.get(e);
        match v {
            None => {
                println!("v3: {} -> None", e);
            }
            Some(t) => match t {
                &MixedType::IVar(s) => {
                    println!("v3: {} -> int {}", e, s);
                }
                &MixedType::FVar(s) => {
                    println!("v3: {} -> float {}", e, s);
                }
            },
        }
    }

    // hash maps
    let mut tests = HashMap::new();

    tests.insert(String::from("Test01"), TestResult::Pass);
    tests.insert(String::from("Test02"), TestResult::Fail);

    let t = String::from("Test03");
    tests.insert(t, TestResult::Pass);
    // t is no more valid here, its ownership now belongs to 'tests'

    let query = vec!["Test01", "Test02", "Test03", "Test04"];

    for tc in query {
        let res = tests.get(tc);
        match res {
            None => {
                println!("Test {} not run", tc);
            }
            Some(s) => {
                println!("Test {} -> {:?}", tc, s);
            }
        }
    }

    test_report("Report #1", &tests);

    // overwrite prev
    tests.insert(String::from("Test01"), TestResult::Fail);
    test_report("Report #2", &tests);

    // insert new if no such key
    tests
        .entry(String::from("Test01"))
        .or_insert(TestResult::Skip);
    tests
        .entry(String::from("Test02"))
        .or_insert(TestResult::Skip);
    tests
        .entry(String::from("Test03"))
        .or_insert(TestResult::Skip);
    tests
        .entry(String::from("Test04"))
        .or_insert(TestResult::Skip);
    tests
        .entry(String::from("Test05"))
        .or_insert(TestResult::Skip);
    test_report("Report #3", &tests);

    // update
    let data = vec!["Test01", "Test02", "Test03", "Test04", "Tests05", "Tests06"];
    for tc in data {
        let res = tests.entry(String::from(tc)).or_insert(TestResult::Skip);
        match *res {
            TestResult::Fail => *res = TestResult::Skip,
            TestResult::Pass => *res = TestResult::Skip,
            _ => {}
        }
    }

    test_report("Report #4", &tests);

    // update all
    test_set_all(&mut tests, &TestResult::Pass);
    test_report("Report #5", &tests);

    // update all
    test_set_all(&mut tests, &TestResult::Skip);
    test_report("Report #6", &tests);
}

fn test_report(n: &str, m: &HashMap<String, TestResult>) {
    println!("------ {} ------", n);
    for (k, v) in m {
        println!("Test {}: {:?}", k, v);
    }
}

fn test_set_all(m: &mut HashMap<String, TestResult>, r: &TestResult) {
    for (_, v) in m.iter_mut() {
        match r {
            &TestResult::Pass => *v = TestResult::Pass,
            &TestResult::Fail => *v = TestResult::Fail,
            &TestResult::Skip => *v = TestResult::Skip,
        }
    }
}

fn dump(s: &str, v: &Vec<i32>) {
    for e in v {
        println!("{}: {}", s, e);
    }
}

fn update(v: &mut Vec<i32>) {
    for e in v {
        *e += 1;
    }
}

#[test]
fn f_vec_test1() {
    let mut v: Vec<i32> = Vec::new();

    v.push(0);
    assert_eq!(v, vec![0]);

    v.push(1);
    assert_eq!(v, vec![0, 1]);

    v.push(2);
    assert_eq!(v, vec![0, 1, 2]);

    v.push(2);
    assert_eq!(v, vec![0, 1, 2, 2]);

    v[3] = 3;
    assert_eq!(v, vec![0, 1, 2, 3]);

    update(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4]);
}

#[test]
fn f_vec_test2() {
    let v = vec![0, 1, 2, 3, 4, 5];

    assert_eq!(v.get(1), Some(&1i32));
    assert_eq!(v.get(10), None);
}

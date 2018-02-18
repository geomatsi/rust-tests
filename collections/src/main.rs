#[derive(Debug)]
enum MixedType {
    IVar(i32),
    FVar(f32),
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
            }
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

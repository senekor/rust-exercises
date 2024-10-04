use apple_counter::*;

#[test]
fn no_apples_dropped() {
    let expected = 0;
    let mut truck = ClumsyTruck::new(expected);
    let dropped_apples = count_dropped_apples(&mut truck);
    assert_eq!(dropped_apples, expected);
}

#[test]
fn one_apple_dropped() {
    let expected = 1;
    let mut truck = ClumsyTruck::new(expected);
    let dropped_apples = count_dropped_apples(&mut truck);
    assert_eq!(dropped_apples, expected);
}

#[test]
fn two_apples_dropped() {
    let expected = 2;
    let mut truck = ClumsyTruck::new(expected);
    let dropped_apples = count_dropped_apples(&mut truck);
    assert_eq!(dropped_apples, expected);
}

#[test]
fn seven_apples_dropped() {
    let expected = 7;
    let mut truck = ClumsyTruck::new(expected);
    let dropped_apples = count_dropped_apples(&mut truck);
    assert_eq!(dropped_apples, expected);
}

#[test]
fn ten_thousand_apples_dropped() {
    let expected = 10_000;
    let mut truck = ClumsyTruck::new(expected);
    let dropped_apples = count_dropped_apples(&mut truck);
    assert_eq!(dropped_apples, expected);
}

struct ClumsyTruck<Apple> {
    apples_to_drop: usize,
    apples: Vec<Apple>,
}
impl<Apple> ClumsyTruck<Apple> {
    fn new(apples_to_drop: usize) -> Self {
        Self {
            apples_to_drop,
            apples: Vec::new(),
        }
    }
}
impl<Apple> Truck<Apple> for ClumsyTruck<Apple> {
    fn load(&mut self, apple: Apple) {
        self.apples.push(apple);
    }

    fn deliver(&mut self) {
        if self.apples.len() != 10_000 {
            panic!("The truck should be loaded with 10_000 apples!");
        }
        let apples_to_keep = self.apples.len() - self.apples_to_drop;
        self.apples.truncate(apples_to_keep);
    }
}

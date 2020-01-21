struct Single<T> {
    next: Option<T>,
}

impl<T> Iterator for Single<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        std::mem::swap(&mut self.next, &mut result);
        result
    }
}

fn single<T>(t: T) -> Single<T> {
    Single { next: Some(t) }
}

fn main() {
    let actual = single(42).collect::<Vec<u32>>();
    assert_eq!(vec![42], actual);
}

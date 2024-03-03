pub trait Print<T> {
    fn print(&self) -> String;
}

impl<T: ToString> Print<T> for [T] {
    fn print(&self) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<T: ToString> Print<T> for [(T, T)] {
    fn print(&self) -> String {
        self.iter()
            .map(|(x, y)| format!("{} {}", x.to_string(), y.to_string()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

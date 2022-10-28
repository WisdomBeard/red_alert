pub trait Hidable {
    fn hide(&mut self) -> ();
    fn reveal(&mut self) -> ();
    fn is_hidden(&self) -> bool;
}
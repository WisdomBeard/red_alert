pub trait Hittable {
    fn hit(&mut self) -> ();
    fn repair(&mut self) -> ();
    fn is_hit(&self) -> bool;
}
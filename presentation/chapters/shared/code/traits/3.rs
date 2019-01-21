trait Foo {}

trait Bar {
    fn fooify(&self) -> impl Foo;
}
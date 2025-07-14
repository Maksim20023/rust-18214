use do_something::do_something;
struct Foo;
struct Bar;
struct Cas;
struct Baz;

do_something! {
    foo: Foo,
    bar: Bar,
}

fn foo() {
    let _d = Data::new(Foo, Bar);
}

fn main() {foo()}
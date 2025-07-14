use do_something::do_something;
struct Foo;
struct Bar;
struct Cas;
struct Baz;

do_something! {
    foo: Foo,
    bar: Bar,
    cas: Cas,
    baz: Baz,
}

fn foo() {
    let _d = Data::new(Foo, Bar, Cas, Baz);
}

fn main() {foo()}
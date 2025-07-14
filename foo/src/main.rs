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
    let d = Data::new(Foo, Bar, Cas, Baz);
    d.foo;
    d.bar;
    d.cas;
    d.baz;
}

fn main() {foo()}
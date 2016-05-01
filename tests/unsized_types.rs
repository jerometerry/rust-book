// every T implicity as T: Sized, and the ? undoes this default
struct Foo<T: ?Sized> {
    f: T,
}

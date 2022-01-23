# unsafe-ref-rust
Allow raw pointer access without those pesky unsafe blocks everywhere!

```rs
let mut a = A(0);
let mut x: UnsafeMutRef<dyn Foo> = UnsafeMutRef::new(&mut a);
let mut y: UnsafeMutRef<dyn Foo> = UnsafeMutRef::new(&mut a);
let z: UnsafeRef<dyn Foo> = UnsafeRef::new(&a);
x.set(42);
y.set(z.get() + 10);
```

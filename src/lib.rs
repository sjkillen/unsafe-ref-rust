use std::ops::{Deref, DerefMut};

pub struct UnsafeRef<T: ?Sized> {
    x: *const T,
}

impl<T: ?Sized> Deref for UnsafeRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            return &*self.x;
        }
    }
}

impl<T: ?Sized> UnsafeRef<T> {
    pub fn new<'a>(x: &'a T) -> Self {
        return UnsafeRef { x };
    }
}

pub struct UnsafeMutRef<T: ?Sized> {
    x: *mut T,
}

impl<T: ?Sized> Deref for UnsafeMutRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            return &*self.x;
        }
    }
}

impl<T: ?Sized> DerefMut for UnsafeMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            return &mut *self.x;
        }
    }
}

impl<T: ?Sized> UnsafeMutRef<T> {
    pub fn new<'a>(x: &'a mut T) -> Self {
        return UnsafeMutRef { x };
    }
}

#[cfg(test)]
mod tests {
    use crate::{UnsafeMutRef, UnsafeRef};

    trait Foo {
        fn set(&mut self, v: i32);
        fn get(&self) -> i32;
    }
    struct A(i32);
    impl Foo for A {
        fn set(&mut self, v: i32) {
            self.0 = v;
        }
        fn get(&self) -> i32 {
            self.0
        }
    }

    #[test]
    fn it_works() {
        let mut a = A(0);
        let mut x: UnsafeMutRef<dyn Foo> = UnsafeMutRef::new(&mut a);
        let mut y: UnsafeMutRef<dyn Foo> = UnsafeMutRef::new(&mut a);
        let z: UnsafeRef<dyn Foo> = UnsafeRef::new(&a);
        x.set(42);
        y.set(z.get() + 10);
        if z.get() != 52 {
            panic!();
        }
    }
}

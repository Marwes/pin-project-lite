use pin_project_lite::pin_project;

pin_project! {
    pub struct S {
        #[pin]
        field: u8,
    }
    impl PinnedDrop for S {
        fn drop(self: Pin<&mut Self>) {
            self.__drop_inner();
        }
    }
}

fn main() {
    let _x = S { field: 0 };
}

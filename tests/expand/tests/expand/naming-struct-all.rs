use pin_project_lite::pin_project;

pin_project! {
    #[project = StructProj]
    #[project_ref = StructProjRef]
    struct Struct<T, U> {
        #[pin]
        pinned: T,
        unpinned: U,
    }
}

fn main() {}

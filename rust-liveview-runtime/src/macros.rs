macro_rules! cfg_async_std_runtime {
   ($($item:item)*) => {
        $(
            #[cfg(any(feature = "async-std-runtime"))]
            $item
        )*
    }
}

macro_rules! cfg_tokio_runtime {
   ($($item:item)*) => {
        $(
            #[cfg(feature = "tokio-runtime")]
            $item
        )*
    }
}

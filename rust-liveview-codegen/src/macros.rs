macro_rules! cfg_runtime {
   ($($item:item)*) => {
        $(
            #[cfg(feature = "runtime")]
            $item
        )*
    }
}

macro_rules! cfg_view {
   ($($item:item)*) => {
        $(
            #[cfg(feature = "view")]
            $item
        )*
    }
}

macro_rules! cfg_common {
   ($($item:item)*) => {
        $(
            #[cfg(feature = "common")]
            $item
        )*
    }
}

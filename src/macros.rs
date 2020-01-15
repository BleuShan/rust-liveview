#[macro_export]
macro_rules! cfg_wasm_arch {
   ($($item:item)*) => {
        $(
            #[cfg(target_arch = "wasm32")]
            $item
        )*
    }
}

#[macro_export]
macro_rules! cfg_native_arch {
   ($($item:item)*) => {
        $(
            #[cfg(not(target_arch = "wasm32"))]
            $item
        )*
    }
}

#[macro_export]
macro_rules! cfg_test {
   ($($item:item)*) => {
        $(
            #[cfg(test)]
            $item
        )*
    }
}

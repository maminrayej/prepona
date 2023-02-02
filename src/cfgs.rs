macro_rules! cfg_parallel {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "parallel")]
            $item
        )*
    }
}

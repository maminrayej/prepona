macro_rules! cfg_parallel {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "parallel")]
            #[cfg_attr(docsrs, doc(cfg(feature = "parallel")))]
            $item
        )*
    }
}

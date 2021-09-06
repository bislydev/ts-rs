// macro_rules! impl_via {
//     ( ($($t:tt)* ) via $via:ty) => {
//         $($t)* {
//             fn decl() -> String {
//                 <$via as $crate::TS>::decl()
//             }
//             fn name() -> String {
//                 <$via as $crate::TS>::name()
//             }
//             fn name_with_type_args(args: Vec<String>) -> String {
//                 <$via as $crate::TS>::name_with_type_args(args)
//             }
//             fn inline(indent: usize) -> String {
//                 <$via as $crate::TS>::inline(indent)
//             }
//             fn inline_flattened(indent: usize) -> String {
//                 <$via as $crate::TS>::inline_flattened(indent)
//             }
//             fn dependencies() -> Vec<($crate::TypeId, String)> {
//                 <$via as $crate::TS>::dependencies()
//             }
//             fn transparent() -> bool {
//                 <$via as $crate::TS>::transparent()
//             }
//             fn dump(out: impl AsRef<std::path::Path>) -> std::io::Result<()> {
//                 <$via as $crate::TS>::dump(out)
//             }
//         }
//     };
// }

#[cfg(feature = "bytes")]
mod bytes {
    use crate::TS;
    use bytes::Bytes;
    use std::any::TypeId;

    // impl_via!((impl TS for Bytes) via Vec<u8>);

    impl TS for Bytes {
        fn name() -> String {
            "Array<number>".to_owned()
        }

        fn inline(indent: usize) -> String {
            format!("Array<{}>", u8::inline(indent))
        }

        fn dependencies() -> Vec<(TypeId, String)> {
            vec![(TypeId::of::<u8>(), u8::name())]
        }

        fn transparent() -> bool {
            true
        }
    }
}


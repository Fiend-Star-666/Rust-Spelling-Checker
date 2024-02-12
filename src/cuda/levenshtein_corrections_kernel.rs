// use accel_derive::kernel;
// use accel::*;
//
// #[kernel]
// pub fn levenshtein_corrections_kernels(unknown_words: DeviceBufferRef<String>, dictionary: DeviceBufferRef<String>, corrections: DeviceBufferRef<Option<String>>) {
//     let idx = accel_core::index();
//     if idx < unknown_words.len() {
//         let word = &unknown_words[idx];
//         let correction = levenshtein_checker.suggest_correction(word.as_str());
//         corrections[idx] = correction;
//     }
// }
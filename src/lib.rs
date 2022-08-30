//! # profile_time
//! This lightweight and simples crate provides 2 macros
//! to help you separate debug and release code.
//! This crate provides:
//! * [`debug_time!`]
//! * [`release_time!`]

/// Code inside the macro will only run
/// if the current build is on debug mode.
/// This will not run if on release.
/// 
/// # Examples
/// ```no_run
/// use profile_time::debug_time;
/// 
/// debug_time! {
///     // Code inside here will only be run in debug mode
///     println!("Current mode: debug");
/// }
/// ```
#[macro_export]
macro_rules! debug_time {
    ($($body:tt)*) => {
        #[cfg(debug_assertions)]
        $($body)*
    };
}

/// Code inside the macro will only run
/// if the current build is on release mode.
/// This will not run if on debug.
/// 
/// # Examples
/// ```no_run
/// use profile_time::release_time;
/// 
/// release_time! {
///     // Code inside here will only be run in debug mode
///     println!("Current mode: release");
/// }
/// ```
#[macro_export]
macro_rules! release_time {
    ($($body:tt)*) => {
        #[cfg(not(debug_assertions))]
        $($body)*
    };
}
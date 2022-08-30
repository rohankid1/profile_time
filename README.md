# profile_time
This crates provides 2 macros to help you separate debug and release code.

# Usage

```rs
#[macro_use]
extern crate profile_time;

release_time! {
    struct Release {}
}

fn print_profile() {
    release_time! {
        println!("On release");
    }

    debug_time! {
        println!("On debug");
    }
}

debug_time! {
    struct Debug {}
}

fn main() {
    release_time! {
        let profile = Release {};
    }

    print_profile();

    debug_time! {
        let profile = Debug {}; 
    }
}
```
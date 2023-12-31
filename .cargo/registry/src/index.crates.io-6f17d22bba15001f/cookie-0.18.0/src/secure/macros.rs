#[cfg(test)]
macro_rules! assert_simple_behaviour {
    ($clear:expr, $secure:expr) => ({
        assert_eq!($clear.iter().count(), 0);

        $secure.add(Cookie::new("name", "val"));
        assert_eq!($clear.iter().count(), 1);
        assert_eq!($secure.get("name").unwrap().value(), "val");
        assert_ne!($clear.get("name").unwrap().value(), "val");

        $secure.add(Cookie::new("another", "two"));
        assert_eq!($clear.iter().count(), 2);

        $clear.remove("another");
        assert_eq!($clear.iter().count(), 1);

        $secure.remove("name");
        assert_eq!($clear.iter().count(), 0);
    })
}

#[cfg(test)]
macro_rules! assert_secure_behaviour {
    ($clear:expr, $secure:expr) => ({
        $secure.add(Cookie::new("secure", "secure"));
        assert!($clear.get("secure").unwrap().value() != "secure");
        assert!($secure.get("secure").unwrap().value() == "secure");

        let mut cookie = $clear.get("secure").unwrap().clone();
        let new_val = format!("{}l", cookie.value());
        cookie.set_value(new_val);
        $clear.add(cookie);
        assert!($secure.get("secure").is_none());

        let mut cookie = $clear.get("secure").unwrap().clone();
        cookie.set_value("foobar");
        $clear.add(cookie);
        assert!($secure.get("secure").is_none());
    })
}

// This is courtesty of `static_assertions`. That library is Copyright (c) 2017
// Nikolai Vazquez. See https://github.com/nvzqz/static-assertions-rs for more.
macro_rules! const_assert {
    ($x:expr $(,)?) => {
        #[allow(unknown_lints, clippy::eq_op)]
        const _: [(); 0 - !{ const ASSERT: bool = $x; ASSERT } as usize] = [];
    };
}

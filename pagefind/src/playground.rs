pub const PLAYGROUND_HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/playground/index.html"
));

pub const PLAYGROUND_CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/playground/pagefind-playground.css"
));

pub const PLAYGROUND_JS: &str = concat!(
    "const pagefind_version = \"",
    env!("CARGO_PKG_VERSION"),
    "\";",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/vendor/playground/pagefind-playground.js"
    ))
);

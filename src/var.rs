pub mod var{
    /**
     * Creating JavaScript-style variables
     * ```
     * likejs::var!(var str_text = "Hello, World!".to_string());
     * ```
     */
    
    #[macro_export]
    macro_rules! var {
        (let $name:ident = $contents:expr) => {
            #[allow(unused)]
            let mut $name = ($contents);
        };
        (var $name:ident = $contents:expr) => {
            #[allow(unused)]
            let mut $name = ($contents);
        };
        (const $name:ident = $contents:expr) => {
            let $name = $contents;
        };
    }
}
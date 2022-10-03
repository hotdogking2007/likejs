pub mod function{
    /**
     * JavaScript arrow function implementation
     * ```
     * let f = likejs::arrow!(() => {"Hello, World!".to_string()});
     * ```
     * or
     * ```
     * let f = likejs::arrow!(() => "Hello, World!".to_string());
     * ```
     */
    #[macro_export]
    macro_rules! arrow {
        (($($vars:ident),*) => $stat:block) => {
            |$($vars),*| {$stat}
        };
        (($($vars:ident),*) => $oneline:expr) => {
            |$($vars),*| {$oneline}
        };
    }
    /**
     * JavaScript function implementation
     * ```
     * likejs::normal_function!(function add(a, b){
     *     a+b
     * });
     * let int = add(12,13);
     * ```
     */
    #[macro_export]
    macro_rules! normal_function {
        (function $name:ident($($vars:ident),*) $stat:block) => {
            let $name = |$($vars: _),*| {$stat};
        }
    }
}
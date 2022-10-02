pub mod function{
    /**
     * 자바스크립트 arrow 함수 구현
     * ```
     * let f = arrow!(() => "Hello, World!".to_string())
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
     * 자바스크립트 함수 구현
     * ```
     * normal!(function add(a, b){
     *     a+b
     * })
     * ```
     */
    #[macro_export]
    macro_rules! normal {
        (function $name:ident($($vars:ident),*) $stat:block) => {
            let $name = |$($vars),*| {$stat};
        }
    }
}
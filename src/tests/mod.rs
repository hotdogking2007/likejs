#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn funtion_test1(){
        let i = 3;
        let f1 = arrow!((a) => {
            a
        });
        assert_eq!(f1(i), 3)
    }
    #[test]
    fn funtion_test2(){
        let i1 = 3;
        let i2 = 7;
        let f1 = arrow!((a, b) => {
            a+b
        });
        assert_eq!(f1(i1,i2), 10)
    }
    #[test]
    fn funtion_test3(){
        let i = 10;
        let f1 = arrow!((a) => a);
        assert_eq!(f1(i), 10)
    }
    #[test]
    fn funtion_test4(){
        let i = 3;
        normal!(function f1(a) {
            a
        });
        assert_eq!(f1(i), 3)
    }
    #[test]
    fn funtion_test5(){
        let i1 = 3;
        let i2 = 7;
        normal!(function f1(a, b) {
            a+b
        });
        assert_eq!(f1(i1,i2), 10)
    }
    #[test]
    fn funtion_test6(){
        normal!(function f1() {
            return 1;
        });
        let i = f1();
        assert_eq!(1,i)
    }
}
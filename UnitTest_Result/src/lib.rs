#[cfg(test)]
mod test{
    /* 
    不能对这些使用 Result<T, E> 的测试使用 #[should_panic] 注解。
    为了断言一个操作返回 Err 成员，不要使用对 Result<T, E> 值使用问号表达式（?）。
    而是使用 assert!(value.is_err())。
     */
    #[test]
    fn  resultTest() -> Result<(),String>{
        if 2+2==4 {
            Ok(())
        }else {
            Err(String::from("Error"))
        }
    }
}
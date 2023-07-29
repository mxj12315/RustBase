use UnitTest_adder;
/* 
    使用cargo test --test integration_test 命令来测试tests文件夹下的单元测试
 */
#[test]
fn it_adds_two() {
    assert_eq!(4, UnitTest_adder::add_two(2));
    
}

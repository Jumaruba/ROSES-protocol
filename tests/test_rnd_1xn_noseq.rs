mod tester;
use tester::Tester; 

#[test]
fn test(){
    let mut tester = Tester::new();
    tester.init(2,2, 2);
    for _ in 0..100 {
        tester.apply_operation();
        tester.disseminate();
        assert_eq!(true, tester.verify());
    }

    assert_eq!(tester.aw_clis[1].fetch(), tester.clis[1].fetch());

}
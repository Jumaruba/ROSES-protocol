mod tester;
use tester::Tester; 

#[test]
fn test_rnd_1xn_noseq(){
    C2T!(BEGIN);
    let mut tester = Tester::new();
    tester.init(1,1, 2);
    for _ in 0..7{
        tester.apply_operation();
        tester.disseminate();
        assert_eq!(true, tester.verify());
    }
    C2T!(END);

}
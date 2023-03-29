mod tester;
use tester::Tester; 

#[test]
fn test_rnd_1xn_noseq(){
    C2T!(BEGIN);
    let mut tester = Tester::new();
    let n_tests = 1; 
    tester.init(2, 2, 2);
    for _ in 0..n_tests {
        for _ in 0..20 {
            tester.apply_operation();
            tester.disseminate();
        }
        println!("{:?}", tester);
        assert_eq!(tester.verify(), true);
    }

    C2T!(END);

}
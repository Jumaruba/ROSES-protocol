mod tester;
use tester::Tester; 

#[test]
fn test_rnd_nxn_noseq(){
    C2T!(BEGIN);
    let n_tests = 1000; 
    for i in 0..n_tests {
        let mut tester = Tester::new();
        tester.init(2, 2, 2);
        println!("TEST {} ====================", i);
        for _ in 0..20 {
            tester.apply_operation();
            tester.disseminate();
        }
        println!("{:?}", tester);
        assert_eq!(tester.verify(), true);
    }

    C2T!(END);

}
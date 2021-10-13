mod goldberg;

fn main() {
    let side = 3;
    let gold = goldberg::Goldberg::new(side);

    // print unique tags to generate NFTs
    let l = gold.path_list();
    for i in l {
        println!("{}", i);
    }
}

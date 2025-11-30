 use std::io::Write;
 fn main(){
    let mut file = std::fs::File::create("Nigerian Brewery Limited.txt").expect("create failed");

    let lager = vec!["Lager Drinks","33 Export" , "Desparados" , "Goldberg" , "Gulder" , "Heineken" , "Star"];
    let stout = vec!["Stout drinks","Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Non alcoholic drinks","Maltina","Amstel Malta","Malta Gold","Fayrouz"];

    for i in 0..lager.len(){
        let b = lager[i];
        let k = stout.len();
        let t = non_alcoholic.len();

        file.write_all(b.as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        println!("\nLager drinks updated to file");

        if k>=i+1{
            let c = stout[i];
            println!("this is k {}",k);
            file.write_all(c.as_bytes()).expect("write failed");
                file.write_all("\t".as_bytes()).expect("write failed");
                println!("\nStout drinks updated to file");
            }
            else{
                continue;
            }

        if t>=i+1{
            let d = non_alcoholic[i];
            println!("this is t {}",t);
            file.write_all(d.as_bytes()).expect("write failed");
                file.write_all("\t".as_bytes()).expect("write failed");
                println!("\nNon alcoholic drinks updated to file");
            }
        else{
            continue;
        }

        file.write_all("\n".as_bytes()).expect("write failed");
    }


}       

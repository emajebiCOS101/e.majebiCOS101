use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Information Service Department-Abuja.txt").expect("create failed");

    file.write_all("\t\t\tEFCC MINISTERS MERGED FILE\n\n".as_bytes()).unwrap();
    let sn = vec!["S/N", "1", "2", "3", "4", "5"];
    let name_of_commisioner = vec!["NAME OF COMMISSIONER","Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbonna","Adewale Jimoh Akanbi","Osazuwa Faith Etiyeye",];
    let ministry = vec!["MINISTRY","Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geopolitical_zone = vec!["GEOPOLITICAL ZONE","South West","North East","South South","South West","South East"];
    let max_len = sn.len().max(name_of_commisioner.len()).max(ministry.len()).max(geopolitical_zone.len());

    for i in 0..max_len {
        if i < sn.len() {
            file.write_all(sn[i].as_bytes()).unwrap();
        }
        file.write_all("\t\t".as_bytes()).unwrap();

        if i < name_of_commisioner.len() {
            file.write_all(name_of_commisioner[i].as_bytes()).unwrap();
        }
        file.write_all("\t\t".as_bytes()).unwrap();

        if i < ministry.len() {
            file.write_all(ministry[i].as_bytes()).unwrap();
        }
        file.write_all("\t\t".as_bytes()).unwrap();

        if i < geopolitical_zone.len() {
            file.write_all(geopolitical_zone[i].as_bytes()).unwrap();
        }

        file.write_all("\n".as_bytes()).unwrap();
    }
    println!("Datasets successfully merged");
}

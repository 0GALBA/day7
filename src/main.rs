use std::{fs::File, io::Read};
use std::io::Result;
use std::str::FromStr;

struct Calibration{
    potentiel_résultat: i32,
    nombre_non_traité : Vec<i32>,
}
impl Calibration {
    fn new_from_string(line:&str) -> Calibration{
        // line: "109101993: 3 8 2 471 2 5 5 2 1 1 517"
        let tmp1 :Vec<&str> = line.split(':').collect();
        let tmp1_5 = tmp1.get(0).unwrap();
        let tmp2 = tmp1.get(1).unwrap();
        let potentiel_résultat = i32::from_str(tmp1_5).unwrap();
        let mut nb : Vec<i32> = Vec::new();
        let tmp2_5 : Vec<&str> = tmp2.split(' ').collect();
        for nombre_non_traité in tmp2_5 {
            nb.push(i32::from_str(nombre_non_traité).unwrap());
        }

        Calibration{
            potentiel_résultat: potentiel_résultat,
            nombre_non_traité: nb,
        }
    }
}

fn algo(cal: Calibration) {
    //faire en sorte que la fonction elle traite qu'une opération entre 2 chifrre               fonction récursive
    //elle se rapelle elle meme avec le nouvelle argument et le chiffre d'apres (ne pas se rappler si il n'y a pas de chiffre apres)
    //copîer le vec et modifier le vec copier en retirzant les chiffre copier en soit les additionant soir en les multilpiant
    let mut vec_copie1 = Calibration::tmp2_5.clone();
    let mut vec_copie2 = Calibration::tmp2_5.clone();
    let mut nmbr1 = vec_copie1[0];
    let mut nmbr2 = vec_copie1[1];


    let mut encrs1 = nmbr1 + nmbr2;
    let mut encrs2 = nmbr1 * nmbr2;

    vec_copie1.remove(0);
    vec_copie1.remove(0);
    vec_copie1.insert(0, encrs1);

    vec_copie2.remove(0);
    vec_copie2.remove(0);
    vec_copie2.insert(0, encrs2);
}


fn main() -> Result<()> {
    //tableau
    let mut buffer = String::new();
    let mut f = File::open("data.csv")?;
    f.read_to_string(&mut buffer)?;
    let lignes = buffer.split('\n');
    for ligne in lignes {
        let cal = Calibration::new_from_string(ligne);
    }


    Ok(())
}
use std::{fs::File, io::Read};
use std::io::Result;
use std::str::FromStr;

struct Calibration{
    potentiel_résultat: i64,
    nombre_non_traité : Vec<i64>,
}
impl Calibration {
    fn new_from_string(line:&str) -> Calibration{
        // line: "109101993: 3 8 2 471 2 5 5 2 1 1 517"
        let tmp1 :Vec<&str> = line.split(':').collect();
        let tmp1_5 = tmp1.get(0).unwrap();
        let tmp2 = tmp1.get(1).unwrap();
        let potentiel_résultat = i64::from_str(tmp1_5).unwrap();
        let mut nb : Vec<i64> = Vec::new();
        let tmp2_5 : Vec<&str> = tmp2.split(' ').collect();
        for nombre_non_traité in tmp2_5 {
            if nombre_non_traité.len()>0 {
                nb.push(i64::from_str(nombre_non_traité).unwrap());   
            }
        }

        Calibration{
            potentiel_résultat: potentiel_résultat,
            nombre_non_traité: nb,
        }
    }
}

fn algo(potentiel_résultat: i64, nombre_non_traité:Vec<i64> ) -> bool {
    //faire en sorte que la fonction elle traite qu'une opération entre 2 chifrre               fonction récursive
    //elle se rapelle elle meme avec le nouvelle argument et le chiffre d'apres (ne pas se rappler si il n'y a pas de chiffre apres)
    //copîer le vec et modifier le vec copier en retirzant les chiffre copier en soit les additionant soir en les multilpiant
    
    let nmbr1 = nombre_non_traité[0];
    let nmbr2 = nombre_non_traité[1];
    let mut encrs1 = 0;
    let mut encrs2 = 0;

    encrs1 = nmbr1 + nmbr2;
    encrs2 = nmbr1 * nmbr2;

    if nombre_non_traité.len() <= 2 {

        if potentiel_résultat == encrs1 {
            return true
        }

        if potentiel_résultat == encrs2 {
            return true
        }
    } else {
        let mut vec_copie1 =nombre_non_traité.clone();
        let mut vec_copie2 =nombre_non_traité.clone();
        vec_copie1.remove(0);
        vec_copie1.remove(0);
        vec_copie1.insert(0, encrs1);
    
        vec_copie2.remove(0);
        vec_copie2.remove(0);
        vec_copie2.insert(0, encrs2);
    
        if encrs1 < potentiel_résultat {
            if algo(potentiel_résultat, vec_copie1) {
                return true;
            }
        }
    
        if encrs2 < potentiel_résultat {
            if algo(potentiel_résultat, vec_copie2) {
                return true;
            }
        }
    }

    return false;
    //appler algo avec comme argument cal et vec copie 1 et 2
    //que algo retourne un bool
}

fn main() -> Result<()> {
    //tableau
    let mut reponceeeeeeeeeee = 0;
    let mut buffer = String::new();
    let mut f = File::open("data.csv")?;
    f.read_to_string(&mut buffer)?;
    let lignes = buffer.split('\n');
    for ligne in lignes {
        let cal = Calibration::new_from_string(ligne);
        if algo(cal.potentiel_résultat, cal.nombre_non_traité) == true {
            reponceeeeeeeeeee += cal.potentiel_résultat;
        }
    }
    println!("{}", reponceeeeeeeeeee);
    Ok(())
}
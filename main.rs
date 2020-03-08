use std::io;
use rand::Rng;
use std::convert::TryFrom;


fn saisie_nombre(texte:&str) -> i64 {
    let mut saisie:String = String::new();
    let valeur:i64;
    println!("{}", texte);
    io::stdin().read_line(&mut saisie).expect("Erreur de saisie, une erreur interne est survenue.");
    match saisie.trim().parse() {
        Ok(v) => {
            valeur = v
        }
        Err(_e) => {
            println!("Ceci n'est pas un nombre entier");
            valeur = saisie_nombre(texte);
        }
    }

    valeur
}

fn gen_bornes(mini:i64, maxi:i64) -> i64 {
    let mut rng = rand::thread_rng();
    let aleat:i64 = rng.gen_range(mini, maxi);

    aleat
}

fn difficulte(niveau:u8) -> (i64, i64, i64) {
    let borne_mini:i64;
    let mut borne_maxi:i64 = 100;

    if niveau < 5 {
        borne_mini = 0;
    }
    else {
        borne_mini = 1 - (num::pow(niveau as i64, 2_usize) * niveau as i64);
    }
    if niveau > 1 {
        borne_maxi = num::pow(borne_maxi, 2_usize) * niveau as i64;
    }

    (gen_bornes(borne_mini, borne_maxi), borne_mini, borne_maxi)
}

fn plus_ou_moins(valeur:i64, secret:i64) -> i8 {
    if valeur < secret {
        println!("C'est plus que {}", valeur);
        return 1i8;
    }
    else if valeur > secret {
        println!("C'est moins que {}", valeur);
        return 2i8;
    }
    else {
        println!("\n*** Félicitation !!! Tu as trouvé {} ***", secret);
        return 0i8;
    }
}

fn partie(niveau:u8) {
    let (secret, borne_mini, borne_maxi) = difficulte(niveau);
    let mut maxi:i64 = borne_maxi;
    let mut mini:i64 = borne_mini;
    let mut nbr_coups:i64 = 0;
    loop {
        let saisie:i64 = saisie_nombre(&String::from(format!("Saisissez un nombre entier entre {} et {} :", mini, maxi)));
        nbr_coups += 1;
        println!("Coup n°{}", nbr_coups);
        match plus_ou_moins(saisie, secret) {
            2i8 => {
                if saisie < maxi {
                    maxi = saisie;
                }
            }
            1i8 => {
                if saisie > mini {
                    mini = saisie;
                }
            }
            0i8 => {
                println!("************* En {} coups ! *************", nbr_coups);
                break;
            }
            _ => {()}
        }
    }
}

fn main() {
    println!("##############################################");
    println!("### Bienvenue dans le Jeu du Plus ou Moins ###");
    println!("############ Valbou 2020 - v0.0.1 ############");
    println!("\n");

    let mut niveau:u8 = match u8::try_from(saisie_nombre("Choisissez votre niveau de jeu (minimum 1) :")) {
        Ok(v) => {v},
        Err(_e) => {println!("Erreur de saisie ! Le niveau a 1 a été choisi ! \n"); 1u8},
    };
    if niveau < 1 {
        niveau = 1;
    }
    partie(niveau);

    println!("\n");
    println!("############# Fin de la Partie ! #############");
    println!("##############################################");
}

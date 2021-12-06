/* Rust cheat sheet */

/* NOTES
*   1. Unused variables must be begin by underscore
*   2. Essential chapter Memory security : https://jimskapt.github.io/rust-book-fr/ch04-01-what-is-ownership.html
*   3. Print debuging infos : 
        => println!("{:#?}", object);
        => dbg!(&object);
*   4. The function with "!" at end is functional macros 
*/

/****** DERIVE MACRO ********
*
*   #[function(parameter)] => apply in next line (function, module, structure ...)
*   #![function(parameter)] => apply in entire crate
*
****  DERIVE MACRO LIST  ****
*   #![allow(unused_variables)] // Désactive les warnings concernant les variables non utilisés
*   #[derive(debug)]            // Permet de dériver un type non primitif afin de stocker des 
*                               // informations de déboguage (ex: println!("structure : {:?}", ma_structure);)
*                               // Elle doit être placé au dessus de la définition d'un type non primitif
*   #![allow(dead_code)]        // Autorise les fonctions non utilisés
*/
#![allow(dead_code)]
#![allow(unused_variables)]

// TYPES & CONST
const BINARY_WORD: u8 = 0b1111_0000;
const HEX_WORD: u8 = 0xf0;
const DEC_WORD: u8 = 240;
const CHAR_WORD: u8 = b'A'; // only u8

const ARCHITECTURE_TYPE: usize = 54_789; // u32 in 32 bits and u64 in 64 bits system

const MY_BOOLEAN: bool = true;
const MY_TUPLE: (i32, f64, u8) = (500, 54.12345, 1);

fn main() {
    println!("Hello, world!");
    // TUPLES
    let (_x, _y, _z) = MY_TUPLE;
    println!("La valeur de y est : {}", _y);
    println!("La valeur de z est : {}", MY_TUPLE.2);

    // ARRAYS
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _mois = ["Janvier", "Février", "Mars", "Avril", "Mai", "Juin", "Juillet", "Août", "Septembre", "Octobre", "Novembre", "Décembre"];
    let _a = [3; 5]; // make 5 elements with value 3

    // FUNCTIONS
    une_autre_fonction(MY_TUPLE.0);
    afficher_mesure_avec_unite(5, 'h');
    let x = plus_un(5);
    println!("La valeur de x est : {}", x);

    // STRUCTURE DE CONTROLE
    let nombre = 6;
    if nombre % 4 == 0 {
        println!("Le nombre est divisible par 4");
    } else if nombre % 3 == 0 {
        println!("Le nombre est divisible par 3");
    } else if nombre % 2 == 0 {
        println!("Le nombre est divisible par 2");
    } else {
        println!("Le nombre n'est pas divisible par 4, 3 ou 2");
    }

    // INLINE IF STATEMENT
    let condition = true;
    let nombre = if condition { 5 } else { 6 };
    println!("La valeur du nombre est : {}", nombre);

    // LOOP STICKER
    let mut compteur = 0;
    'increment: loop {
        println!("compteur = {}", compteur);
        let mut restant = 10;
        loop {
            println!("restant = {}", restant);
            if restant == 9 {
                break;
            }
            if compteur == 2 {
                break 'increment;
            }
            restant -= 1;
        }
        compteur += 1;
    }
    println!("Fin du compteur = {}", compteur);

    // LOOP ASSIGNMENT
    let mut compteur = 0;
    let resultat = loop {
        compteur += 1;

        if compteur == 10 {
            break compteur * 2;
        }
    };
    println!("Le résultat est {}", resultat);

    // WHILE CONDITION
    let mut nombre = 3;
    while nombre != 0 {
        println!("{} !", nombre);

        nombre -= 1;
    }
    println!("DÉCOLLAGE !!!");

    // FOR CONDITION WITH ARRAY
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("La valeur est : {}", element);
    }

    // FOR CONDITION
    for nombre in (1..4).rev() {
        println!("{} !", nombre);
    }
    println!("DÉCOLLAGE !!!");

    // STRUCTURE INITIALISATION
    let utilisateur1 = Utilisateur {
        email: String::from("quelquun@example.com"),
        pseudo: String::from("pseudoquelconque123"),
        actif: true,
        nombre_de_connexions: 1,
    };

    // STRUCTURE INITIALISATION WITH OTHER INSTANCE
    let utilisateur2 = Utilisateur {
        email: String::from("quelquundautre@example.com"),
        ..utilisateur1 // signifie que le reste de la structure sera complété avec l'instance de l'autre structure Utilisateur
    };
}

// FUNCTIONS
fn une_autre_fonction(x: i32) {
    println!("La valeur de x est : {}", x);
}

fn afficher_mesure_avec_unite(valeur: i32, unite: char) {
    println!("La mesure est : {}{}", valeur, unite);
}

fn plus_un(x: i32) -> i32 {
    x + 1
}

// STRUCTURE DEFINITION
struct Utilisateur {
    actif: bool,
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
}
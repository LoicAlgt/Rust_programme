static mut GLOBAL_VAR: Option<&'static str> = None;

fn genvar() {
    let val = "loic"; // Valeur initiale de la variable locale
    unsafe {
        GLOBAL_VAR = Some(val);
    }
    println!("Variable locale initialisée : {}", val);
}

fn test() {
    let global_val = unsafe { GLOBAL_VAR };
    match global_val {
        Some(val) => println!("Valeur de la variable globale : {}", val),
        None => println!("Aucune valeur n'a été stockée dans la variable globale."),
    }
}

fn main() {
    genvar();
    test();
}

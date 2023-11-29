use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::prelude::*;


mod utiles;

#[derive(Clone)]
struct Pokemon {
    numero: String,
    nombre: String,
    type1: String,
    type2: String,
    total: String,
    hp: String,
    attak: String,
    defense: String,
    sp_attak: String,
    sp_defense: String,
    speed: String,
    generation: String,
    legendary: String,
}


fn elimina_fila() {

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("pokemon.txt")
    .unwrap();

    let mut new_pokemon = inicializar_struct();


    let mut control_pikashu: bool = false;

    loop {
        let nombre_pokemon = utiles::ingreso_texto("mete el nombre de tu pinshi pokemon".to_string());
        if let Ok(lines) = read_lines("./pokemon.txt") {
            for line in lines {




                if let Ok(ip) = line {
                    let ip_copy = ip.clone();
                    let split = ip_copy.split(",");

                    let mut contador_columnas = 0;

                    for elemento in split {
                        match contador_columnas {
                            0 => new_pokemon.numero = elemento.to_string().to_lowercase(),
                            1 => new_pokemon.nombre = elemento.to_string().to_lowercase(),
                            2 => new_pokemon.type1 = elemento.to_string().to_lowercase(),
                            3 => new_pokemon.type2 = elemento.to_string().to_lowercase(),
                            4 => new_pokemon.total = elemento.to_string().to_lowercase(),
                            5 => new_pokemon.hp = elemento.to_string().to_lowercase(),
                            6 => new_pokemon.attak = elemento.to_string().to_lowercase(),
                            7 => new_pokemon.defense = elemento.to_string().to_lowercase(),
                            8 => new_pokemon.sp_attak = elemento.to_string().to_lowercase(),
                            9 => new_pokemon.sp_defense = elemento.to_string().to_lowercase(),
                            10 => new_pokemon.speed = elemento.to_string().to_lowercase(),
                            11 => new_pokemon.generation = elemento.to_string().to_lowercase(),
                            12 => new_pokemon.legendary = elemento.to_string().to_lowercase(),
                            _ => (),

                        }
                        contador_columnas += 1;
                    }

                        if nombre_pokemon.to_lowercase() == new_pokemon.nombre {
                            println!("\n~~Se encontro a su Pokemon~~\n");
                            println!(
                                "#{}\nNombresss~ {}\nType1~ {}\nType2~ {}\nTotal~ {}\nHp~ {}\nAttak~ {}\nDefense~ {}\nSp. Attak~ {}\nSp. Defense~ {}\nSpeed~ {}\nGeneration~ {}\nLegendary~ {}",
                                new_pokemon.numero,
                                new_pokemon.nombre,
                                new_pokemon.type1,
                                new_pokemon.type2,
                                new_pokemon.total,
                                new_pokemon.hp,
                                new_pokemon.attak,
                                new_pokemon.defense,
                                new_pokemon.sp_attak,
                                new_pokemon.sp_defense,
                                new_pokemon.speed,
                                new_pokemon.generation,
                                new_pokemon.legendary
                            );
                            control_pikashu = true;
                        }
                    }
                }
                if control_pikashu == false {

                    let string_print = format!("{},{},{},{},{},{},{},{},{},{},{},{},{}", new_pokemon.numero,new_pokemon.nombre,  
                    new_pokemon.type1,new_pokemon.type2,new_pokemon.total,new_pokemon.hp,new_pokemon.attak,
                    new_pokemon.defense,new_pokemon.sp_attak,new_pokemon.sp_defense,new_pokemon.speed,new_pokemon.generation,
                    new_pokemon.legendary);
                    file.write_all(string_print.as_bytes()).expect("PERDON NO FUNCIONO");                
                    
                } else {
                    println!("no se agrega  ");

                }
            }

            if control_pikashu {
                break;
            } else {
                println!("No se encontro el pokemon en el dataset, Por favor ingrese otro denuevo");
            }
        }
}





fn cambia_nombre(){

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("pokemon.txt")
    .unwrap();
    let mut control_pikashu = false;
    let mut new_pokemon = inicializar_struct();
    let mut struct_aux = inicializar_struct();

    let mut mote_nuevo = String::new();

    loop {
        let nombre_pokemon = utiles::ingreso_texto("mete el nombre de tu pinshi pokemon".to_string());
        if let Ok(lines) = read_lines("./pokemon.txt") {
            control_pikashu = false;
            for line in lines {



                if let Ok(ip) = line {
                    let ip_copy = ip.clone();
                    let split = ip_copy.split(",");

                    let mut contador_columnas = 0;

                    for elemento in split {
                        match contador_columnas {
                            0 => new_pokemon.numero = elemento.to_string().to_lowercase(),
                            1 => new_pokemon.nombre = elemento.to_string().to_lowercase(),
                            2 => new_pokemon.type1 = elemento.to_string().to_lowercase(),
                            3 => new_pokemon.type2 = elemento.to_string().to_lowercase(),
                            4 => new_pokemon.total = elemento.to_string().to_lowercase(),
                            5 => new_pokemon.hp = elemento.to_string().to_lowercase(),
                            6 => new_pokemon.attak = elemento.to_string().to_lowercase(),
                            7 => new_pokemon.defense = elemento.to_string().to_lowercase(),
                            8 => new_pokemon.sp_attak = elemento.to_string().to_lowercase(),
                            9 => new_pokemon.sp_defense = elemento.to_string().to_lowercase(),
                            10 => new_pokemon.speed = elemento.to_string().to_lowercase(),
                            11 => new_pokemon.generation = elemento.to_string().to_lowercase(),
                            12 => new_pokemon.legendary = elemento.to_string().to_lowercase(),
                            _ => (),

                        }
                        contador_columnas += 1;
                    }

                        if nombre_pokemon.to_lowercase() == new_pokemon.nombre {
                            println!("\n~~Se encontro a su Pokemon~~\n");
                            println!(
                                "#{}\nNombre~ {}\nType1~ {}\nType2~ {}\nTotal~ {}\nHp~ {}\nAttak~ {}\nDefense~ {}\nSp. Attak~ {}\nSp. Defense~ {}\nSpeed~ {}\nGeneration~ {}\nLegendary~ {}",
                                new_pokemon.numero,
                                new_pokemon.nombre,
                                new_pokemon.type1,
                                new_pokemon.type2,
                                new_pokemon.total,
                                new_pokemon.hp,
                                new_pokemon.attak,
                                new_pokemon.defense,
                                new_pokemon.sp_attak,
                                new_pokemon.sp_defense,
                                new_pokemon.speed,
                                new_pokemon.generation,
                                new_pokemon.legendary
                            );
                            control_pikashu = true;
                        }
                    }
                }


                if control_pikashu == true {
                    struct_aux = new_pokemon.clone();

                    mote_nuevo = utiles::ingreso_texto("nuevo mote".to_string());
                    struct_aux.nombre = mote_nuevo;
                    println!("SE CAMBIO EL NOMBRE");
                
                }

                let string_print = format!("\n{},{},{},{},{},{},{},{},{},{},{},{},{}", new_pokemon.numero,new_pokemon.nombre,  
                new_pokemon.type1,new_pokemon.type2,new_pokemon.total,new_pokemon.hp,new_pokemon.attak,
                new_pokemon.defense,new_pokemon.sp_attak,new_pokemon.sp_defense,new_pokemon.speed,new_pokemon.generation,
                new_pokemon.legendary); //y continuar
                //hacer el write_all
                file.write_all(string_print.as_bytes()).expect("PERDON NO FUNCIONO");                
            }

            if control_pikashu {
                let string_print = format!("\n{},{},{},{},{},{},{},{},{},{},{},{},{}", struct_aux.numero,struct_aux.nombre,  
                struct_aux.type1,struct_aux.type2,struct_aux.total,struct_aux.hp,struct_aux.attak,
                struct_aux.defense,struct_aux.sp_attak,struct_aux.sp_defense,struct_aux.speed,struct_aux.generation,
                struct_aux.legendary); //y continuar
                //hacer el write_all
                file.write_all(string_print.as_bytes()).expect("PERDON NO FUNCIONO");

                break;
            } else {
                println!("No se encontro el pokemon en el dataset, Por favor ingrese otro denuevo");
            }
        }

}


fn eliminar_pokemon(){

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("pokemon.txt")
    .unwrap();

    let mut new_pokemon = inicializar_struct();

    let mut control_pikashu: bool = false;

    let mut file_pokemon = File::create("pokemon1.txt").unwrap();

    loop {
        let nombre_pokemon = utiles::ingreso_texto("mete el nombre de tu pinshi pokemon".to_string());

        if let Ok(lines) = read_lines("./pokemon.txt") {
            let mut contenido = String::new();
    
            for line in lines {
                if let Ok(ref ip) = line {
                    let ip_copy = ip.clone();
                    let split = ip_copy.split(",");
    
                    let mut contador_columnas = 0;
                    let mut skip_line = false;
    
                    for elemento in split {
                        println!("{}", elemento);
                        if contador_columnas == 1 && elemento.to_string().to_lowercase() == nombre_pokemon.to_string().to_lowercase() {
                            control_pikashu = true;
                            skip_line = true;
                            break;
                        }
                        contador_columnas += 1;
                    }
    
                    if skip_line {
                        continue;
                    }
                }
                contenido.push_str(&line.unwrap());
                contenido.push('\n');
            }
    
            file_pokemon.write_all(contenido.as_bytes()).unwrap();

            if control_pikashu {
                let mut file_pokemon = File::create("pokemon1.txt").unwrap();
                file_pokemon.write_all(contenido.as_bytes()).unwrap();
            } else {
                println!("No se encontro el pokemon en el dataset, Por favor ingrese otro denuevo");
            }
        }

        break;
        }
    println!("Listo! revisa el nuevo archivo llamado pokemon1.txt");
}

fn inicializar_struct() -> Pokemon{
    //se crea una variable de tipo Pokemon para almacenar los datos
    let new_pokemon: Pokemon = Pokemon {
        numero: String::new(),
        nombre: String::new(),
        type1: String::new(),
        type2: String::new(),
        total: String::new(),
        hp: String::new(),
        attak: String::new(),
        defense: String::new(),
        sp_attak: String::new(),
        sp_defense: String::new(),
        speed: String::new(),
        generation: String::new(),
        legendary: String::new(),
    };
    return new_pokemon;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn open_file_to_append(p: &Path) -> File{
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}




fn main() {

    println!("digite: \n1 para eliminar una fila \n2 para ponerle un mote al pokemon seleccionado y agregarlo al final de la lista \n ");
    match utiles::texto_numero("desicion".to_string()){
        1 => eliminar_pokemon(),
        2 => cambia_nombre(),
        3 => elimina_fila(),
        _ => ()

    }
}

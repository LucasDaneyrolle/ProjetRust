mod graph;
use graph::Graph;
use std::io::{self, Write, stdout};


fn main() {
  print!("{}[2J", 27 as char);
  let mut finished = false;
  let mut input_string;

  let mut main_graph = Graph::new();

  while !finished {  
    println!("1/ Ajouter un point au graphe");
    println!("2/ Ajouter un chemin d'un point vers un autre");
    println!("3/ Afficher les informations du graphe");
    println!("4/ Calcul du plus court chemin entre 2 points");
    println!("5/ Fin");
    print!("==> Choix: ");
    stdout().flush();
    input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    print!("\n");
    match input_string.as_str().trim().parse::<i32>() {
<<<<<<< HEAD
      Ok(1) => addPoint(&mut mainGraph),
      Ok(2) => println!("{:?}", selectPointIndex(&mut mainGraph)),
      Ok(3) => println!("QSDSD!"),
      Ok(4) => finished = true,
      Ok(_) => println!("Mauvaise entrée !"),
      Err(..) => println!("Mauvaise entrée !"),
=======
      Ok(1) => add_point(&mut main_graph),
      Ok(2) => add_link(&mut main_graph),
      Ok(3) => main_graph.show(),
      Ok(4) => process_dijkstra(&mut main_graph),
      Ok(5) => finished = true,
      Ok(_) => println!("Mauvais entrée !"),
      Err(..) => println!("Mauvais entrée !"),
>>>>>>> 054455aff9f759161014ffcc8ef6d33d4993a058
    }
    print!("\n");
  }
}


fn add_point(main_graph: &mut Graph) {
  let mut nom = String::new();

  print!("Donner un nom: ");
  stdout().flush();
  io::stdin().read_line(&mut nom).expect("Failed to read line");
  main_graph.addPoint(String::from(nom.as_str().trim()));
}


fn add_link(main_graph: &mut Graph) {
  print!("Selectionner un premier point (numero)\n");
  let id_point1 = select_point_id(main_graph);

  print!("==> Selectionner un second point (numero)\n");
  let id_point2 = select_point_id(main_graph);

  print!("==> Veuillez donner un poids au chemin: ");
  stdout().flush();
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  let weight = input.trim().parse::<i32>().unwrap();
  print!("\n");

  main_graph.linkPoints(id_point1, id_point2, weight);
}


fn select_point_id(main_graph: &Graph) -> String {
  let mut input_string;

  loop {
    for i in 0..main_graph.nodes.len() {
      println!("{:?}/ Noeud {:?}", i+1, main_graph.nodes[i].name_id);
    }
    print!("==> Choix: ");
    stdout().flush();
    input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    print!("\n\n");
    let choice = input_string.trim().parse::<usize>().unwrap();
    if choice > 0 && choice <= main_graph.nodes.len() {
      return main_graph.nodes[choice-1].name_id.clone();
    }
  }
}

fn process_dijkstra(main_graph: &mut Graph) {
  print!("Selectionner un point de départ (numero)\n");
  let id_point1 = select_point_id(main_graph);

  print!("==> Selectionner un point d'arrive (numero)\n");
  let id_point2 = select_point_id(main_graph);

  let weight = main_graph.dijkstra(id_point1, id_point2);
  print!("Le poids du plus court chemin est {:?}\n", weight);
  print!("Vous pouvez déterminer le chemin en affichant les informations du graphe\n");
}
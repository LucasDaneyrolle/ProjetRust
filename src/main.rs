mod graph;
use graph::Graph;
use std::io::{self, Write};


fn main() {
  let mut finished = false;
  let mut input_string;

  let mut mainGraph = Graph::new();

  while !finished {  
    println!("1/ Ajouter un point au graphe");
    println!("2/ Ajouter un chemin d'un point vers un autre");
    println!("3/ Calcul du plus court chemin entre 2 points");
    println!("4/ Fin");
    print!("Choix: ");
    io::stdout().flush();
    input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    print!("\n");
    match input_string.as_str().trim().parse::<i32>() {
      Ok(1) => addPoint(&mut mainGraph),
      Ok(2) => println!("{:?}", selectPointIndex(&mut mainGraph)),
      Ok(3) => println!("QSDSD!"),
      Ok(4) => finished = true,
      Ok(_) => println!("Mauvaise entrée !"),
      Err(..) => println!("Mauvaise entrée !"),
    }
    print!("\n");
  }
}

fn addPoint( mainGraph: &mut Graph) {
  let mut nom = String::new();

  print!("Donner un nom: ");
  io::stdout().flush();
  io::stdin().read_line(&mut nom).expect("Failed to read line");
  mainGraph.addPoint(String::from(nom.as_str().trim()));
}

fn selectPointIndex(mainGraph: &Graph) -> i32 {
  let mut input_string;
  let mut i = 0;

  while true {
    for i in 0..mainGraph.nodes.len() {
      println!("{:?}/ Noeud {:?}", i+1, mainGraph.nodes[i].name_id);
    }
    print!("Choix: ");
    io::stdout().flush();
    input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    print!("\n");
    let choice = input_string.parse::<i32>().unwrap();
    if(choice > 0 && choice < (mainGraph.nodes.len() as i32)){
      return choice;
    }
    print!("\n");
  }
  return 0;
}



  /*

  let mut graphTest = Graph::new();

  graphTest.addPoint(String::from("A"));
  graphTest.addPoint(String::from("B"));
  graphTest.addPoint(String::from("C"));
  graphTest.addPoint(String::from("D"));
  graphTest.addPoint(String::from("E"));
  graphTest.addPoint(String::from("F"));

  graphTest.linkPoints(String::from("A"), String::from("B"), 4);
  graphTest.linkPoints(String::from("A"), String::from("C"), 4);
  graphTest.linkPoints(String::from("B"), String::from("C"), 2);
  graphTest.linkPoints(String::from("C"), String::from("D"), 3);
  graphTest.linkPoints(String::from("C"), String::from("E"), 1);
  graphTest.linkPoints(String::from("D"), String::from("F"), 2);
  graphTest.linkPoints(String::from("E"), String::from("F"), 3);

  let test = graphTest.dijkstra(String::from("A"), String::from("F"));
  graphTest.show();
  println!("Poids du chemin optimisé: {:?}", test);
  */


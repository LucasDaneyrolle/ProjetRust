mod graph;
use graph::Graph;

fn main() {
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
}


pub struct NodeLink { // CHEMIN
  toNode: String,
  weigth: i32
}

pub struct Node { // NOEUD
  pub name_id: String,
  links: Vec<NodeLink>,
  calculWeight: i32,
}

pub struct Graph {
  pub nodes: Vec<Node>,
}

impl Graph {

  pub fn new() -> Graph {
    return Graph { nodes: Vec::new() }
  }

  pub fn show(&self) {
    print!("########### AFFICHAGE GRAPH #############\n");
    for i in 0..self.nodes.len() {
      print!("\nNoeud {:?}: (poids calculé: {:?})\n", self.nodes[i].name_id, self.nodes[i].calculWeight);
      for j in 0..self.nodes[i].links.len() {
        print!("    - Vers noeud {:?} -> Poids: {:?}\n", self.nodes[i].links[j].toNode, self.nodes[i].links[j].weigth);
      }
    }
  }

  pub fn addPoint(&mut self, name_id: String) {
    self.nodes.push(Node { name_id, links: Vec::new(), calculWeight: i32::MAX })
  }

  pub fn linkPoints(&mut self, name_id: String, to_name_id: String, weight: i32) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut i_found = false;
    let mut j_found = false;

    while i < self.nodes.len() && !i_found { //ON VERIF SI LE PREMIER POINT EXISTE
      if self.nodes[i].name_id == name_id {
        while j < self.nodes.len() && !j_found { //SI OUI, ON VERIF SI LE DEUXIEME POINT EXISTE
          if self.nodes[j].name_id == name_id {
            self.nodes[j].links.push(NodeLink { toNode: to_name_id.clone(), weigth: weight });
            j_found = true;
          }  
          j += 1;
        }
        i_found = true;
      }
      i += 1;
    }

    return !i_found || !j_found;
  }

  pub fn dijkstra(&mut self, name_id: String, to_name_id: String) -> i32 {

    // REMETTRE LES POIDS A MAX OU 0
    for i in 0..self.nodes.len() {
      if self.nodes[i].name_id == name_id {
        self.nodes[i].calculWeight = 0;
      } else {
        self.nodes[i].calculWeight = i32::MAX;
      }
    }

    for j in 0..self.nodes.len() {
      for k in 0..self.nodes[j].links.len() {
        
        // ON RECUPERE L'INDEX DU NOEUD DE DESTINATION
        let mut x = 0;
        while x < self.nodes.len() && self.nodes[x].name_id != self.nodes[j].links[k].toNode {
          x += 1;
        }

        // Si le " POIDS DU NOEUD TRAITE + LE POIDS DU CHEMIN " est inférieur au " NOEUD DE DESTINATION "
        // On update le POIDS DU NOEUD SUIVANT avec la nouvelle valeur inférieur
        if self.nodes[j].calculWeight + self.nodes[j].links[k].weigth < self.nodes[x].calculWeight {
          self.nodes[x].calculWeight = self.nodes[j].calculWeight + self.nodes[j].links[k].weigth;
        }
      }
    }

    // ON RECUPERE L'INDEX DU NOEUD DE FIN
    let mut x = 0;
    while x < self.nodes.len() && self.nodes[x].name_id != to_name_id {
      x += 1;
    }

    return self.nodes[x].calculWeight;
  }
}


#[cfg(test)]
mod test {
use crate::Graph;
    //test si on peut bien relier les points A et B
    #[test]
    fn test_linkPoints_AB() {
        assert_eq!(
          Graph::linkPoints(&mut Graph { nodes: Vec::new() }, String::from("A"), String::from("B"), 4),
          true,
        )
    }

    //test si on peut bien relier les points B et C
    #[test]
    fn test_linkPoints_BC() {
      assert_eq!(
        Graph::linkPoints(&mut Graph { nodes: Vec::new() }, String::from("B"), String::from("C"), 3),
        true,
      )
    }

    //test si on peut bien relier les points C et F
    #[test]
    fn test_linkPoints_CF() {
      assert_eq!(
        Graph::linkPoints(&mut Graph { nodes: Vec::new() }, String::from("C"), String::from("F"), 1),
        true,
      )
    }

    //test si un poids négatif est possible (c'est possible)
    #[test]
    fn test_linkPoints_val() {
      assert_eq!(
        Graph::linkPoints(&mut Graph { nodes: Vec::new() }, String::from("C"), String::from("D"), -1),
        true,
      )
    }

    #[test]
    fn test_dijkstra() {
      assert_eq!(
        Graph::dijkstra(&mut Graph { nodes: Vec::new() }, String::from("C"), String::from("F")),
        0,
      )
    }
}
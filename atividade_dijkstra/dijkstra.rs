//pseudocódigo (wikipedia)
//  
//function Dijkstra(Graph, source):
//
//      for each vertex v in Graph.Vertices:
//
//          dist[v] ← INFINITY
//          prev[v] ← UNDEFINED
//          add v to Q
//      dist[source] ← 0
//      
//      while Q is not empty:
//          u ← vertex in Q with min dist[u]
//          remove u from Q
//          
//          for each neighbor v of u still in Q:
//              alt ← dist[u] + Graph.Edges(u, v)
//              if alt < dist[v]:
//                  dist[v] ← alt
//                  prev[v] ← u
//
//      return dist[], prev[]
//
//dist -> array com distância de source para outros vértices
//
//prev -> ponteiros para "previous-hop nodes" no caminho mais curto de source para o vértice dado
//
//u procura o vértice em Q com o menor dist
//
//Graph.Edges retorna a distância entre os dois vértices "edge joining"
//
//alt recebe o valor da distância do nó source para esse nó vizinho v se for passor por u. Se
//o caminho for menor do que o menor caminho já salvo para v, então isso será sobrescrito pelo
//caminho alt

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;


//estrutura de um vértice
#[derive(Debug, PartialEq, Eq)]
struct Vertex {
        id: usize,
        distance: i32,
}

//implementation necessário para fazer com que o BinaryHeap compare de acordo com o distance
impl Ord for Vertex {
        fn cmp(&self, other: &Self) -> Ordering {
                other.distance.cmp(&self.distance)
        }
}

//implementation necessário para fazer com que o BinaryHeap compare de acordo com o distance
impl PartialOrd for Vertex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}

//função declarada de acordo com o pseudocódigo
fn dijkstra(graph: HashMap<usize, Vec<(usize, i32)>>, source: usize) -> (HashMap<usize, i32>, HashMap<usize, Option<usize>>) {

        //dist é um HashMap iniciado com todos os pares sendo (vertex, i32::MAX)
        let mut dist: HashMap<usize, i32> = graph.keys().map(|&v| (v, i32::MAX)).collect();

        //prev é um HashMap de vertex e Option(Some or None) iniciados com None para os
        //predecessores, já que isso é o que vamos usar para representar os caminhos
        let mut prev: HashMap<usize, Option<usize>> = graph.keys().map(|&v| (v, None)).collect();

        //criamos um binary heap para armazenar a fila de prioridade (mais eficiente)
        let mut q = BinaryHeap::new();

        //source adicionado a q
        q.push(Vertex { id: source, distance: 0 });

        //dist do source é zero
        dist.insert(source, 0);

        //começamos a repetição retirando o vértice da lista de prioridade com o menor dist_u
        while let Some(Vertex { id: u, distance: dist_u }) = q.pop() {
                if dist_u > *dist.get(&u).unwrap() {
                        continue;
                }

                //aqui obtemos uma lista de vizinhos de u enquanto houverem
                if let Some(neighbors) = graph.get(&u) {

                        //para cada vizinho de u...
                        for &(v, weight) in neighbors {

                                //alt é a distância até o nó v percorrendo pelo u
                                let alt = dist_u + weight;

                                //se esse caminho for menor que o atual para v, então vamos utilizar esse
                                //ao invés do antigo.
                                if alt < *dist.get(&v).unwrap() {

                                        //inserimos o vértice com a nova distância no dist
                                        dist.insert(v, alt);

                                        //atualizamos o caminho contido no prev
                                        prev.insert(v, Some(u));

                                        //adicionamos o v atualizado no q.
                                        q.push(Vertex { id: v, distance: alt });
                                }
                        }
                }
        }
        //retorno
        (dist, prev)
}


//main para testes
fn main() {
        let mut graph: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();
        graph.insert(0, vec![(1, 4), (2, 2)]);
        graph.insert(1, vec![(3, 5)]);
        graph.insert(2, vec![(1, 1), (3, 8)]);
        graph.insert(3, vec![(4, 3)]);
        graph.insert(4, vec![]);

        let source = 0;

        let (distances, prev) = dijkstra(graph, source);

        println!("Distances from the source vertex:");
        for (vertex, distance) in &distances {
                println!("Vertex {}: {}", vertex, distance);
        }

        println!("Predecessors:");
        for (vertex, predecessor) in &prev {
                match predecessor {
                        Some(prev_vertex) => println!("Vertex {}: {}", vertex, prev_vertex),
                        None => println!("Vertex {}: Not reachable", vertex),
                }
        }
}

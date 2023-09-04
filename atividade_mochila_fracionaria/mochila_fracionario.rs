//para resolver o problema da mochila fracionaria, vamos precisar de um algoritmo guloso que
//coloque primeiro os itens na mochila que posuem o melhor valor/peso até chegar no último item que
//possui um peso maior do que a capacidade restante da mochila. No caso desse último item, vamos
//dividi-lo para ocupar o espaço restante na mochila.

use std::vec;

//comecamos definindo o que é um item
struct Item {
        weight: f64,
        value: f64,
        ratio: f64,
}

//declaração função da mochila fracionaria
fn fractional_knapsack(items: &mut Vec<Item>, capacity: f64) -> f64 {

        //calculando ratio de cada item
        for item in items.iter_mut() {
                item.ratio = item.weight / item.value;
        }

        //sorteia os items de maneira decrescente de acordo com o ratio
        items.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap());

        //variáveis para guardar o valor total dos itens na mochila e a capacidade restante
        let mut total_value = 0.0;
        let mut remaining_capacity = capacity;

        //vamos calcular para cada item até que o último seja fracionado e colocado na mochila
        for item in items {
                
                //se tivermos espaço suficiente para o item, então só precisamos colocá-lo na
                //mochila sem fracionar
                if remaining_capacity > item.weight {
                        total_value = total_value + item.value;
                        remaining_capacity = remaining_capacity - item.weight;

                //caso seja necessário fracionar, dividimos o espaço restante pelo ratio
                } else {
                        total_value += item.ratio * remaining_capacity;
                        break;
                }
        }

        //retornamos o valor total guardado na mochila
        total_value
}

//main para testes
fn main() {
        let mut items = vec![
                Item { weight: 2.0, value: 3.5, ratio: 0.0 },
                Item { weight: 2.7, value: 9.9, ratio: 0.0 },
                Item { weight: 8.0, value: 1.0, ratio: 0.0 },
                Item { weight: 7.0, value: 5.0, ratio: 0.0 },
                Item { weight: 3.0, value: 3.0, ratio: 0.0 },
                Item { weight: 1.0, value: 2.0, ratio: 0.0 },
        ];

        let capacity = 27.0;

        let max_value = fractional_knapsack(&mut items, capacity);
        println!("valor máximo na mochila com os items atuais: {}", max_value);
}

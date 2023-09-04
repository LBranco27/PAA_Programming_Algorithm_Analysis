//comentando o algoritmo de help-karp em C

#include <stdio.h>
#include <limits.h>

//número máximo de cidades
#define MAX_N 8 

//número de cidades
int n;

//matriz de distâncias
int dist[MAX_N][MAX_N]; 

//matriz de memoização
int memo[MAX_N][1 << MAX_N]; 

//funcao recursiva do problema do caixeiro viajante
int tsp(int current, int mask) {

	//mask é uma mascara que, onde o bit estiver 1, significa que a cidade foi visitada.
	//Esse laço de interrupção verifica se todas as cidades foram visitadas
	//(todos os bits são 1 ate n-1). Se sim, precisamos voltar para a cidade inicial para
	//finalizar o tour
	if (mask == (1 << n) - 1) {
		return dist[current][0]; // Retornar à cidade inicial
	}

	//verifica se esse valor já foi calculado, se sim, então retorna ele
	if (memo[current][mask] != -1) {

		//retornar valor memoizado
		return memo[current][mask]; 
	}

	int minDistance = INT_MAX;

	//itera sobre as possíveis cidades ainda não visitadas
	for (int next = 0; next < n; next++) {
		//se a próxima cidade não foi visitada
		if (!(mask & (1 << next))) { 
			//calcula a distância total para visitar a cidade 'next' a partir de 'current'
			//e soma com o resultado de tsp que encontra a distância mínima do percurso
			//subsequente a partir de 'next'
			int newDistance = dist[current][next] + tsp(next, mask | (1 << next));

			//se a nova distância for menor...
			if (newDistance < minDistance) {
				minDistance = newDistance;
			}
		}
	}

	//adicionamos a distância mínima para current no memo
	//tabela de memorização
	memo[current][mask] = minDistance;

	//minDistance é o retorno de tsp
	return minDistance;
}

//Exemplo de iteração
//Digite o número de cidades: 3
//Digite as distâncias entre as cidades:
//Digite a distância da cidade 1 para a cidade 2: 4
//Digite a distância da cidade 1 para a cidade 3: 6
//Digite a distância da cidade 2 para a cidade 1: 3
//Digite a distância da cidade 2 para a cidade 3: 7
//Digite a distância da cidade 3 para a cidade 1: 4
//Digite a distância da cidade 3 para a cidade 2: 6
//Distância mínima do percurso: 15
//

int main() {
	printf("Digite o número de cidades: ");
	scanf("%d", &n);

	printf("Digite as distâncias entre as cidades:\n");
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < n; j++) {
			if(j != i){
				printf("Digite a distância da cidade %d para a cidade %d: ", i+1, j+1);
				scanf("%d", &dist[i][j]);
			}
		}
	}

	//inicializar a matriz de memo com -1
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < (1 << n); j++) {
			memo[i][j] = -1;
		}
	}

	//começar da cidade 0 com a primeira cidade visitada
	int minTour = tsp(0, 1); 
	printf("Distância mínima do percurso: %d\n", minTour);

	return 0;
}

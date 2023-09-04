//codigo feito por Lucas Branco para o ultimo projeto da disciplina AED 2. Que é um jogo em que o jogador precisa chegar na saída de uma caverna cheia de salas e caminhos.
//é um código antigo que fiz sem muito interesse em deixar conciso e otimizado, então há vários problemas criados por minha pressa em fazer isso o mais rápido possível.
//também há algumas decisões duvidáveis feitas nesse código
//um algortimo A* que busca o caminho mais curto para um vertice em um grafo. A* utiliza heurísticas.

//o algoritmo recebe um "Chamber" que é um vértice do grafo. Perceba que não recebe um vértice de destino pois no caverna há somente uma saída que é o nosso vértice de destino.
public int findEscape2(Chamber start){

	//inicializamos o vértice atual com os seguintes valores para F, G e H.
	//F representa a distância mínima que é possível ter com um dado vértice. Calculado por G + H. 0 por padrão para o start
	start.setF(0);

	//G representa a distância percorrida pelo caminho mais curto do início até o vértice. start inicia com 0 pois é o início
	start.setG(0);

	//H representa a heurística. O custo estimado (mínimo) do vértice atual para o vértice de destino. 0 por padrão para o start
	start.setH(0);

	//openSet é uma lista com os vértices que podemos selecionar
	ArrayList<Chamber> openSet = new ArrayList<Chamber>();

	//closedSet é uma lista com os vértices já selecionados
	ArrayList<Chamber> closedSet = new ArrayList<Chamber>();

	//start é adicionado ao openSet pois vamos começar com ele
	openSet.add(start);

	//neighbor será utilizado no código
	Chamber neighbor = new Chamber();

	//exit é o vértice de destino
	Chamber exit = new Chamber();

	//esse laço de repetição procura o vértice de destino na lista com todos os vértices do grafo
	//na época que criei esse código não percebi que um continue depois de achar a saída otimizaria um pouco mais o código
	for (Chamber chamber : chambers) {
		if (chamber.isExit() == true) {
			exit = chamber;
		}
	}

	//enquanto existirem nós que podemos percorrer
	while(!openSet.isEmpty()){

		//current armazenará o vértice de menor F do openSet
		Chamber current = new Chamber();

		//i servirá para checarmos se está é a primeira vez rodando essa repetição
		int i = 0;

		//para todos os vértices em openSet
		for (Chamber chamber : openSet) {

			//no primeiro caso, queremos que o primeiro (e único) vértice do openSet (start) seja selecionado
			if(i == 0){
				current = chamber;
				
				//i é incrementado para essa condição não ser executada novamente
				i++;
			}
			
			//queremos sempre escolher o vértice de menor F do openSet. Pois é o possivelmente mais próximo do destino
			if(chamber.getF() < current.getF()){
				current = chamber;
			}
		}

		//o vértice atual é removido do openSet pois já foi escolhido, e adicionado ao closedSet que formará a solução ótima global
		openSet.remove(current);
		closedSet.add(current);

		//precisamos verificar todos os vértices adjacentes ao vértice escolhido para sabermos qual é o mais próximo do destino
		for (Tunnel tunnel : current.getConnections()) {
			
			//precisamos fazer isso porque o current é um dessas duas variáveis e queremos o nó adjacente, não o atual
			if (tunnel.getDestiny() != current) {
				neighbor = tunnel.getDestiny();
			} else {
				neighbor = tunnel.getOrigin();
			}

			//caso seja um nó adjacente que já foi escolhido previamente, então pulamos esse nó em específico
			if (closedSet.contains(neighbor)){
				continue;
			}

			//depois de algumas execuções o algoritmo encontrará a saída e precisamos retornar o que foi obtido
			if (neighbor.isExit()){
				
				//TunnelNumber representa a distância que será percorrida caso o caminho mais curto foi tomado
				neighbor.setTunnelNumber(current.getG() + Math.abs(neighbor.getDistance() - current.getDistance()));

				//no nosso caso, precisamos retornar apenas a distância mínima e não o caminho de custo mínimo
				//caso seja desejado o caminho de custo mínimo, podemos retornar o closedSet com o nó de saída adicionado a ele
				return neighbor.getTunnelNumber();
			}

			//a Distance é parte de uma estrutura interna do jogo...
			//Com essa subtração obtemos a distância entre o começo e o nó vizinho que estamos analisando
			neighbor.setG(Math.abs(neighbor.getDistance() - start.getDistance()));

			//Aqui obtemos o quão mínimo (barato) poderia ser o caminho adjacente até o nó final
			neighbor.setH(Math.abs(exit.getDistance() - neighbor.getDistance()));

			//E finalmente F que representa a menor distância que possivelmente esse nó vizinho pode ter até o nó final
			neighbor.setF(neighbor.getH() + neighbor.getG());

			//movementCost representa a distância entre o nó atual e esse nó vizinho
			int movementCost = current.getG() + Math.abs(neighbor.getDistance() - current.getDistance()) ;

			//caso mevementCost seja menor que o G do nó vizinho e esse nó ainda não estiver no OpenSet, precisamos colocá-lo no OpenSet
			if (movementCost < neighbor.getG() || !openSet.contains(neighbor)) {

				//trocamos o G do nó atual para o movementCost, assim preparando para a próxima iteração
				neighbor.setG(movementCost);

				//TunnelNumber é atualizado
				neighbor.setTunnelNumber(movementCost);

				//o nó vizinho é adicionado ao openSet para que seus nós vizinhos seja analisados em futuras iterações
				if (!openSet.contains(neighbor)) {
					openSet.add(neighbor);
				}
			}
		}
	}
	//-1 é retornado caso o nó final não seja encontrado. Algo que não deve acontecer nesse código se a geração de grafos estiver correta
	return -1;
}

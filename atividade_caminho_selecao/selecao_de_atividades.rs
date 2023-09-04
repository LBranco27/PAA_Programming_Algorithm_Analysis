//optei por fazer o código em Rust pois é uma linguagem diferente que quero aprender
//pseudocódigo com recursão(slide)
//
//recursivef(s,f,i,n)
//m = i + 1
//while m <= n and sm < fi
//        do m = m + 1
//if m <= n
//        then return {am} U recursivef(s,f,m,n)
//        else return 0
//
//s -> lista com o horário de início das atividades
//f -> lista com o horário de fim das atividades
//i -> indice atual
//n -> tamanho do numero de atividades

//podemos começar com uma struct para definir atividade como um horário inicial e final

//usize é um integer que é 32 bits se a arquitetura do seu computador for 32 bits, 64 bits se for
//64 bits, etc. u -> unsigned
#[derive(Clone)]
struct Activity {
        start: usize,
        finish: usize,
}

// s e f representado por activities.
fn recursive_activity_selector(activities: &Vec<Activity>, i: usize) -> Vec<Activity> {

        //m irá olhar as próximas atividades após a que já escolhemos (0 no início)
        let mut m = i + 0;

        //procura a próxima atividade que começa após o final da atividade atual (0 no início)
        while m < activities.len() && activities[m].start < activities[i].finish {
                m += 1
        }

        //evitar que m esteja fora da lista de atividades
        if m <= activities.len() - 1{

                //chamada recursiva com a posição do próximo indice que queremos ao invés de i
                let mut selected_activities = recursive_activity_selector(activities, m);

                //inserimos um clone da atividade que queremos no vetor de atividades selecionadas

                //após todas as recursões sejam executadas, a última irá colocar a atividade
                //selecionada nessa lista, depois a penúltima... Até que sejam colocadas todas as
                //atividades.
                selected_activities.insert(0, activities[m].clone());
                //retorno
                return selected_activities
        }

        //retorna um vetor vazio caso m > activities.len() (significa que o trabalho foi terminado)
        else {
                return Vec::new()
        }
}

//main para teste
fn main() {
        
        //construcao do activities com o macro !
        let activities = vec![
                Activity { start: 1, finish: 4 },
                Activity { start: 3, finish: 5 },
                Activity { start: 0, finish: 6 },
                Activity { start: 5, finish: 7 },
                Activity { start: 3, finish: 9 },
                Activity { start: 5, finish: 9 },
                Activity { start: 6, finish: 10 },
                Activity { start: 8, finish: 11 },
                Activity { start: 8, finish: 12 },
                Activity { start: 2, finish: 14 },
                Activity { start: 12, finish: 16 },
        ];

        let mut selected_activities = recursive_activity_selector(&activities, 0);

        //não consegui pensar em uma maneira eficiente de colocar a atividade 0 na lista. E das
        //maneiras que tentei, ocasionavam em stack overflow. Algo que eu não entendi é que isso
        //parace ser um problema presente até no pseudocódigo.
        selected_activities.insert(0, activities[0].clone());

        println!("Selected Activities:");
        for activity in selected_activities {
                println!("Start: {}, Finish: {}", activity.start, activity.finish);
        }
}

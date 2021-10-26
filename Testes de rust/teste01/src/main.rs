fn f1(n: usize){
    println!("{}", n);

}

fn main() {

    let v= String::from("777");
    let c1 = |n|{
        println!("{}", n);
    };
    c1(v);
    println!("{}", v);
}
/*
#[derive(Debug)]
struct Jogo{
    nome:String,
    ano:isize,
    num_de_jogadores:isize,
}

#[derive(Debug)]
struct Retangulo{
    largura:isize,
    altura:isize,
}
impl Retangulo{
    fn area(&self) -> isize{
        self.altura * self.largura
    }
}

fn f1(x:isize) -> isize {
    println!("print aqui: {}\nTchau Tchau", x);
    0
}

fn main() {
    let variavel = f1(2);
    println!("print aqui: {}", variavel);

    let jogo1 = Jogo{
        nome: String::from("Hollow Knight"), 
        ano: 2017, 
        num_de_jogadores: 1
    };

    let jogo2 = Jogo{
        nome: String::from("The Legend of Zelda: Breath of the Wild"), 
        ..jogo1
    };

    let ret = Retangulo{
        altura:2,
        largura:2
    };

    println!("nome: {:?}", jogo2);
    println!("{:?}", ret.area());
}*/


    /*loop{
        println!("{}", variavel);
        variavel += 1;
        if variavel == 10 {
            break;
        }
    }*/

    /*
    let mut variavel = [1, 2, 3, 4, 5, 6];

    for elem in variavel.iter() {
        println!("{}", elem);
    }*/

    /*let mut variavel = String::from("oi vitinho");

    println!("{}", variavel);

    variavel.push_str(" aggg");

    println!("{}", variavel);*/
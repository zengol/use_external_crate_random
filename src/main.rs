
use rand::thread_rng; 
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        //usamos thread_rng() para obtener rapidamente un generador
        //lo guardamos en la variable mut "rng" porque su valor cambiara en cada inicialización
        let mut rng = thread_rng();
        //enviamos como argumento "rng" por lo tanto enviamos el generador 
        //a la función shuffle() del
        //trait SliceRandom que es un sudmodulo de el crate rand. 
        //suffle() se encarga de ordenar de manera aleatoria el 
        //vector "cards" usando un algoritmo de encriptación.
        self.cards.shuffle(&mut rng);
    }
    
}
fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    println!("Heres your deck: {:#?}", deck);
}

use yew::prelude::*;

struct Product {
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64,
}

struct State {
    products: Vec<Product>,
}

pub struct Home {
    state: State,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let products: Vec<Product> = vec![
            Product {
                id: 1,
                name: "Fortnite".to_string(),
                description: "No more fortnite for you titouan".to_string(),
                image: "./assets/fortnite.png".to_string(),
                price: 75.0,
            },
            Product {
                id: 2,
                name: "Fortnite".to_string(),
                description: "No more fortnite for you titouan".to_string(),
                image: "./assets/fortnite.png".to_string(),
                price: 75.3,
            },
        ];
        Self {
            state: State { products },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                html! {
                    <div>
                        <img src = {&product.image}/>
                        <div>{&product.name}</div>
                        <div>{"$"}{&product.price.to_string()}</div>
                    </div>
                }
            })
            .collect();
        html! { <span>{products}</span> }
    }
}

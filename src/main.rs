extern crate reqwest_wasm;

use std::borrow::Borrow;
use yew::html::Scope;
use yew::prelude::*;

mod data;

enum Msg {
    FetchNewUser,
    UpdatePerson(data::Result),
}

struct App {
    user: data::Result,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user: data::Result::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::FetchNewUser => {
                link.send_future(App::get_person());

                false
            }
            Msg::UpdatePerson(person) => {
                log::info!("Update Person: {:?}", { &person });
                self.user = person;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::FetchNewUser);

        html! {
            <div class="h-screen bg-gray-600 w-full flex flex-col items-center justify-center gap-y-4">
                <div class="w-96 h-80 bg-gray-800 shadow-md border-indigo-400 h-auto w-auto p-4 pl-8 pr-8 rounded-md font-medium text-xl flex flex-col items-center">
                    <img class="rounded-full w-24 h-24" src={self.user.picture.large.to_string()} />
                    <div class="mt-4 mb-4 flex flex-col gap-y-1">
                        <div class="flex flex-row">
                            <span class="text-gray-100 font-medium">{"Gender: "}</span>
                            <span class="text-gray-300 font-light ml-2">{&self.user.gender}</span>
                        </div>
                        <div class="flex flex-row">
                            <span class="text-gray-100 font-medium">{"Registered: "}</span>
                            <span class="text-gray-300 font-light ml-2">{&self.user.registered.date}</span>
                        </div>
                        <div class="flex flex-row">
                            <span class="text-gray-100 font-medium">{"Phone number: "}</span>
                            <span class="text-gray-300 font-light ml-2">{&self.user.phone}</span>
                        </div>
                    </div>
                    <span class="font-bold text-xl text-center text-indigo-400"> {&self.user.name.first} {" "} {&self.user.name.last} </span>
                    <span class="font-light text-lg text-center text-gray-400"> {"Password: "} {&self.user.login.password} </span>
                </div>

                <button {onclick} class="bg-indigo-400 shadow-md h-auto w-auto pl-4 pr-4 pb-2 pt-2 rounded-md font-medium text-xl text-white hover:bg-indigo-300">{"Find date"}</button>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(App::get_person());
        }
    }
}

impl App {
    async fn get_person() -> Msg {
        let res = reqwest_wasm::get("https://randomuser.me/api/1.2")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let parsed_json = serde_json::from_str::<data::Root>(res.as_str()).unwrap();
        return Msg::UpdatePerson((*parsed_json.results.first().unwrap()).clone());
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

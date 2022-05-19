use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "yew.todomvc.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let entries = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                Vec::new()
            }
        };
        let state = State {
            entries,
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
        };
        App {
            link,
            storage,
            state,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let entry = Entry {
                    description: self.state.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.state.entries.push(entry);
                self.state.value = "".to_string();
            }
            Msg::Edit(idx) => {
                let edit_value = self.state.edit_value.clone();
                self.state.complete_edit(idx, edit_value);
                self.state.edit_value = "".to_string();
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.state.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
            }
            Msg::Nope => {}
        }
        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        let mut items_left_string = "".to_string();
        let total = self.state.total();
        if total == 1 {
            items_left_string.push_str(" item left")
        } else {
            items_left_string.push_str(" items left")
        }
        html! {
            <div class="w-2/3 mx-auto">
                <section>
                    <header class="text-center my-4">
                        <h1 class="text-6xl text-red-600">{ "todos" }</h1>
                        { self.view_input() }
                    </header>
                    <section class="my-4">
                        <label for="toggle_all" class="block w-full rounded bg-slate-300 mb-4 p-4">
                            <input id="toggle_all" type="checkbox" checked=self.state.is_all_completed() onclick=self.link.callback(|_| Msg::ToggleAll) />
                        </label>
                        <ul>
                            { for self.state.entries.iter().filter(|e| self.state.filter.fit(e))
                                .enumerate()
                                .map(|val| self.view_entry(val)) }
                        </ul>
                    </section>
                    <footer class="flex gap-3 justify-around my-4">
                        <span class="border-2 rounded p-4">
                            <strong>{ self.state.total() }</strong>
                            { items_left_string }
                        </span>
                        <ul class="flex-grow gap-3 flex justify-center">
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                        <button class="bg-red-300 hover:bg-red-500 hover:text-white transition-colors rounded p-4" onclick=self.link.callback(|_| Msg::ClearCompleted)>
                            { format!("Clear completed ({})", self.state.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="flex flex-col gap-3 items-center text-sm text-slate-500 my-4 mt-8">
                    <p>{ "Double-click to edit a todo." }</p>
                    <p>{ "Originally written by " }<a class="underline" href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a>{"."}</p>
                    <p>{ "Edited to facilitate tailwindCSS by " }<a class="underline" href="https://blog.vomkonstant.in/" target="_blank">{ "Konstantin Kovar" }</a>{"."}</p>
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_filter(&self, filter: Filter) -> Html {
        let flt = filter.clone();

        html! {
            <li class="p-4 border-2 rounded">
                <a class=if self.state.filter == flt { "selected" } else { "not-selected" }
                   href=&flt
                   onclick=self.link.callback(move |_| Msg::SetFilter(flt.clone()))>
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html {
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input class="p-4 w-full mt-4 border-0 border-b-2 border-slate-500 focus:border-slate-800 focus:outline-none"
                   placeholder="What needs to be done?"
                   value=&self.state.value
                   oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                   onkeypress=self.link.callback(|e: KeyboardEvent| {
                       if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                   }) />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        let mut label_class = "".to_string();
        if entry.editing {
            label_class.push_str(" hidden");
        }
        if entry.completed {
            label_class.push_str(" inline");
        }

        html! {
            <li class="p-4 pr-0 my-2 border-b-2 border-slate-200 last:border-0">
                <div class="flex items-center justify-between">
                <div class="flex gap-6">
                    <input type="checkbox" checked=entry.completed onclick=self.link.callback(move |_| Msg::Toggle(idx)) />
                    <label class=label_class ondblclick=self.link.callback(move |_| Msg::ToggleEdit(idx))>{ &entry.description }</label>
                    { self.view_entry_edit_input((&idx, &entry)) }
                </div>
                    <button class="bg-red-300 hover:bg-red-500 hover:text-white transition-colors rounded p-4 ml-auto" onclick=self.link.callback(move |_| Msg::Remove(idx))>{"Remove"}</button>
                </div>
            </li>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (&usize, &Entry)) -> Html {
        let idx = *idx;
        if entry.editing {
            html! {
                <input
                    type="text"
                    value=self.state.edit_value
                    oninput=self.link.callback(move |e: InputData| Msg::UpdateEdit(e.value))
                    onblur=self.link.callback(move |_| Msg::Edit(idx))
                    onkeypress=self.link.callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                }) />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
}

impl State {
    fn total(&self) -> usize {
        self.entries.len()
    }

    fn total_completed(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| Filter::Completed.fit(e))
            .count()
    }

    fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self
            .entries
            .iter()
            .filter(|e| self.filter.fit(e))
            .peekable();

        if filtered_iter.peek().is_none() {
            return false;
        }

        filtered_iter.all(|e| e.completed)
    }

    fn toggle_all(&mut self, value: bool) {
        for entry in self.entries.iter_mut() {
            if self.filter.fit(entry) {
                entry.completed = value;
            }
        }
    }

    fn clear_completed(&mut self) {
        let entries = self
            .entries
            .drain(..)
            .filter(|e| Filter::Active.fit(e))
            .collect();
        self.entries = entries;
    }

    fn toggle(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.completed = !entry.completed;
    }

    fn toggle_edit(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.editing = !entry.editing;
    }

    fn complete_edit(&mut self, idx: usize, val: String) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.description = val;
        entry.editing = !entry.editing;
    }

    fn remove(&mut self, idx: usize) {
        let idx = {
            let filter = self.filter.clone();
            let entries = self
                .entries
                .iter()
                .enumerate()
                .filter(|&(_, e)| filter.fit(e))
                .collect::<Vec<_>>();
            let &(idx, _) = entries.get(idx).unwrap();
            idx
        };
        self.entries.remove(idx);
    }
}

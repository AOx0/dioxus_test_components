#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
struct AppProps {
    logged_in: bool,
}

fn App(cx: Scope<AppProps>) -> Element {
    let a = cx.props.logged_in.then(|| "DashboardScreen");

    cx.render(rsx! {a})
}

fn Listas(cx: Scope) -> Element {
    let nombres = ["Daniel", "Pedro", "Karla", "Sofia", "Pedro"];
    let nombres_list = nombres
        .iter()
        .filter(|nombre| nombre.starts_with('P'))
        .map(|nombre| rsx! { li { key: "{nombre}", "{nombre}" } });

    cx.render(rsx! {
        ul { nombres_list } // Lista con puntos
    })
}

fn ValorControlado(cx: Scope) -> Element {
    let valor = use_state(&cx, || "".to_string());
    let valor2 = use_ref(&cx, || "".to_string());

    let val = valor2.read().to_string();
    cx.render(rsx! {
        input {
            oninput: move |evt| valor.set(evt.value.clone()),
            value: "{valor}"
        }
        br {}
        input {
            oninput: move |evt| {
                *valor2.write_silent() = evt.value.clone();
                cx.needs_update()
            },
        }
        br {}
        p { "Valor 1: {valor}" }
        br {}
        p { "Valor 2: {val}" }
    })
}

#[derive(PartialEq, Props)]
struct VoteButtonProps {
    score: i32,
}

fn VoteButton(cx: Scope<VoteButtonProps>) -> Element {
    cx.render(rsx! {
        div {
            p { "+" }
            p { "{cx.props.score}" }
            p { "-" }
        }

    })
}

#[inline_props]
fn Title2<'a>(cx: Scope, title: &'a str, otro: Option<&'a str>) -> Element {
    let valor = otro.unwrap_or("Default");

    cx.render(rsx! {
        div {
            p { "{title}" },
            p { "{valor}" }
        }
    })
}

#[derive(Props)]
struct TitleProps<'a> {
    title: &'a str,
}

fn Title<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    cx.render(rsx! {
        "{cx.props.title}"
    })
}

#[inline_props]
fn Division<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        children
    })
}

fn App2(cx: Scope) -> Element {
    let titulo = "Contador";
    cx.render(rsx! {
        Division {
            Title { title: titulo}
            br {}
            Title2 { title: titulo }
            VoteButton { score: 50 }
        }
    })
}

fn main() {
    // dioxus::web::launch_with_props(App, AppProps { logged_in: true }, |config| config);
    dioxus::web::launch(ValorControlado);
}

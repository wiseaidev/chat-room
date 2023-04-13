use web_sys::{HtmlInputElement, InputEvent};
use crate::services::PubnubService;
use serde::{ Deserialize, Serialize };
use std::collections::HashSet;
use log::info;
use std::env;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub text: String,
    pub from: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let mut pubnub = PubnubService::new(env!("PUB_KEY_0"), env!("SUB_KEY_0"));
    let messages = vec![Message {text: "".to_string(), from: "".to_string()}];
    let users = HashSet::new();


    let input_alias_ref = use_node_ref();
    let input_alias_handle = use_state(String::default);
    let input_alias = (*input_alias_handle).clone();

    let on_alias_input_change = {
        let input_alias_ref = input_alias_ref.clone();

        Callback::from(move |_| {
            let input = input_alias_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                input_alias_handle.set(input.value());
            }
        })
    };


    let input_pending_text_ref = use_node_ref();
    let input_pending_text_handle = use_state(String::default);
    let input_pending_text = (*input_pending_text_handle).clone();

    let on_input_pending_text = {
        let input_pending_text_ref = input_pending_text_ref.clone();

        Callback::from(move |_| {
            let input = input_pending_text_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                input_pending_text_handle.set(input.value());
            }
        })
    };
    // let on_message = Callback::from(move |message: Message| {
    //     messages.push(message);
    // });

    // let onoffline = Callback::from(move |nickname: String| {
    //     info!("Removing user {:?}", nickname);
    //     users.remove(&nickname);
    // });

    // let ononline = Callback::from(move |nickname: String| {
    //     info!("Adding user {:?}", nickname);
    //     users.insert(nickname);
    // });

    // let on_connect_click = {
    //     let input_alias_ref = input_alias_ref.clone();

    //     Callback::from(move |_| {
    //         let input = input_alias_ref.cast::<HtmlInputElement>();

    //         if let Some(input) = input {
    //             info!("Called send chat!");
    //             let value = input.value();
    //             pubnub.connect("chat-room", &value, on_message, onoffline, ononline);
    //         }
    //     })
    // };

    // let on_send_msg_click = {
    //     let input_pending_text_ref = input_pending_text_ref.clone();

    //     Callback::from(move |event: MouseEvent| {
    //         event.prevent_default();
    //         let input = input_pending_text_ref.cast::<HtmlInputElement>();

    //         if let Some(input) = input {
    //             info!("Called send chat!");
    //             let value = input.value();
    //             pubnub.send_message("chat-room", &value);
    //             input_pending_text_handle.set("".into());
    //         }
    //     })
    // };

    // let onkeypress = {
    //     let input_pending_text_ref = input_pending_text_ref.clone();

    //     move |event: KeyboardEvent| {
    //         if event.key() == "Enter" {
    //             info!("Called send chat!");
    //             let input = input_pending_text_ref.cast::<HtmlInputElement>();
    //             if let Some(input) = input {
    //                 let value = input.value();
    //                 pubnub.send_message("chat-room", &value);
    //                 input_pending_text_handle.set("".into());
    //                 input.set_value("");
    //             }
    //         }
    //     }
    // };

    html! {
        <div class="container">
          <div class="chat-container">
          <div class="chat-messages">
            <h2>{ "Messages" }</h2>
            <ul>
              { for messages.iter().enumerate().map(view_message) }
            </ul>
            <div class="chat-inputs">
              <input
                placeholder="Type your message.."
                type="text"
                class="pending-text"
                ref={input_pending_text_ref}
                oninput={on_input_pending_text}
                value={input_pending_text}
                //onkeypress={onkeypress}
              />
              <button
               type="submit"
               //onclick={on_send_msg_click}
              >
                {"Send"}
              </button>
            </div>
          </div>
          <div class="chat-users">
            <h2>{ "Users" }</h2>
            <ul>
              { for users.iter().enumerate().map(view_user::<dyn AsMut<PubnubService> + 'static>) }
            </ul>
            <div class="username-input">
              <input
                placeholder="Enter username..."
                type="text"
                ref={input_alias_ref}
                oninput={on_alias_input_change}
                value={input_alias}
              />
              <button
               type="submit"
               //onclick={on_connect_click}
              >
                { "Join" }
              </button>
            </div>
          </div>
        </div>
      </div>
    }
}

fn view_message((_idx, message): (usize, &Message)) -> Html {
    html! {
      <li>
        <div class="message sent">
          <span class="sender">{"["}{&message.from}{"]"}</span>
          <span class="text">{&message.text}</span>
        </div>
      </li>
    }
}

fn view_user<C: ?Sized>((_idx, user): (usize, &String)) -> Html {
    html! {
      <li>
        <span>{ user }</span>
      </li>
    }
}

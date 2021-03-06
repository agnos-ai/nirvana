// Copyright (c) 2020 agnos.ai UI Limited.
#![recursion_limit = "1024"]

mod app;
mod header;
mod item;
mod list;

use std::{cell::RefCell, fmt, ops::Deref, rc::Rc};

use wasm_bindgen::prelude::*;
use yew::html::{Component, ComponentLink};

#[cfg(feature = "use_wee_alloc")]
extern crate wee_alloc;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "use_wee_alloc")]
#[global_allocator]

static ALLOC:wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct WeakComponentLink<COMP:Component,>(Rc<RefCell<Option<ComponentLink<COMP,>,>,>,>,);

impl<COMP:Component,> Clone for WeakComponentLink<COMP,> {
    fn clone(&self,) -> Self { Self(Rc::clone(&self.0,),) }
}

impl<COMP:Component,> Default for WeakComponentLink<COMP,> {
    fn default() -> Self { Self(Rc::default(),) }
}

impl<COMP:Component,> Deref for WeakComponentLink<COMP,> {
    type Target = Rc<RefCell<Option<ComponentLink<COMP,>,>,>,>;

    fn deref(&self,) -> &Self::Target { &self.0 }
}

impl<COMP:Component,> PartialEq for WeakComponentLink<COMP,> {
    fn eq(&self, other:&Self,) -> bool { Rc::ptr_eq(&self.0, &other.0,) }
}

#[derive(Debug,)]

pub enum Hovered {
    Header,
    Item(String,),
    List,
    None,
}

impl fmt::Display for Hovered {
    fn fmt(&self, f:&mut fmt::Formatter<'_,>,) -> fmt::Result {

        write!(f, "{}", match self {
            Hovered::Header => "Header",
            Hovered::Item(name,) => name,
            Hovered::List => "List container",
            Hovered::None => "Nothing",
        })
    }
}

// This is the entry point for the web app
#[wasm_bindgen]

pub fn run_app() -> Result<(), JsValue,> {

    wasm_logger::init(wasm_logger::Config::default(),);

    yew::start_app::<app::App,>();

    Ok((),)
}

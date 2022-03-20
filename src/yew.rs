use std::{marker::PhantomData};

use yew::{html, Callback, Html, Component, Context};

use crate::tree::{
    Directory as DirectoryCore, Gallery as GalleryCore, Picture as PictureCore, PictureTree,
};

pub use crate::tree::{dir,picture};

pub type Directory = DirectoryCore<Html>;
pub type Pictures = PictureTree<Html>;
pub type Picture = PictureCore<Html>;
pub type GalleryModel = GalleryCore<Html>;

pub trait GalleryConfig {
    fn model() -> GalleryModel;
}

pub struct Gallery<C: GalleryConfig + 'static> {
    __marker: PhantomData<C>,
    current: PicturePath,
    model: GalleryModel,
}

impl <C: GalleryConfig + 'static>Component for Gallery<C> {
    type Message = PicturePath;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            __marker: PhantomData,
            current: Vec::new(),
            model: C::model(),
        }
    }

    fn update(&mut self,_ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.current = msg;
        true
    }
    
    fn view(&self,ctx: &Context<Self>) -> Html {
        let current = self.model.get(self.current.iter().cloned());
        html! {
            <div class="cafeteria-root">
                <section class="cafeteria-nav">
                    <h2>{if self.current.len() == 0 { "Cafeteria".to_owned() } else { self.current.join("/") }}</h2>
                    {self.render_tree(ctx.link().callback(|v| v))}
                </section>
                {current.map(|current| { html! { <section>{current}</section> } }).unwrap_or_default()}
            </div>
        }
    }
}

impl <C: GalleryConfig + 'static>Gallery<C> {
    fn render_tree(&self,callback: Callback<PicturePath>) -> Html {
        self.tree_dir(&self.model.dir,callback,Vec::new())
    }
    fn tree_dir_with_name(&self,name: &str, dir: &Directory, callback: Callback<PicturePath>,path: PicturePath) -> Html {
        html! {
            <>
                <p>{name}</p>
                {self.tree_dir(dir,callback,path)}
            </>
        }
    }
    fn tree_dir(&self,dir: &Directory, callback: Callback<PicturePath>,path: PicturePath) -> Html {
        let list = dir.iter().map(move |(key, val)| {
            let mut path = path.clone();
            path.push(key.clone());
            let content = match val {
                PictureTree::Picture(_) => {
                    let path_cloned = path.clone();
                    let callback = callback.reform(move |_| path_cloned.clone());
                    if path.eq(&self.current) { html! {<strong>{key}</strong>} } else { html! {<a onclick={callback}>{key}</a>} }
                }
                PictureTree::Dir(dir) => self.tree_dir_with_name(key.as_str(), dir, callback.clone(),path.clone()),
            };
            html! {<li>{content}</li>}
        });
        html! {
            <ul>
                {for list}
            </ul>
        }
    }
    
}

pub type PicturePath = Vec<String>;
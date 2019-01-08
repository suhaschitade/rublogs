#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;
#[macro_use] extern crate r2d2;
#[macro_use] extern crate r2d2_diesel;

extern crate rocket_contrib;
extern crate chrono;

use rocket::Request;
//use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};

use handlebars::{Helper,Handlebars,Context,RenderContext,Output,HelperResult,JsonRender};
use rocket_contrib::serve::StaticFiles;
use rocket::http::Status;

use rocket_contrib::json::*;
use diesel::result::Error;

mod db;
mod schema;

mod post;

use post::Post;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    parent: &'static str,
}

#[derive(Serialize)]
struct PostContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<Post>,
    parent: &'static str,
}
#[derive(Serialize)]
struct PostDetailsContext {
    title: &'static str,
    name: Option<String>,
    items: Post,
    parent: &'static str,
}

#[get("/index")]
fn index() -> Template {
    //Redirect::to("/hello/unknown")
    Template::render("index", &TemplateContext{
        title:"Hello",
        name:None,
        items: vec!["Four","Five","Six"],
        parent: "layout",
    })
}
#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    Template::render("index", &TemplateContext{
        title:"Hello",
        name:Some(name),
        items: vec!["Four","Five","Six"],
        parent: "layout",
    })
}
#[get("/about")]
fn about() -> Template {
    Template::render("about", &TemplateContext{
        title:"About",
        name:None,
        items: vec!["One","Two","Three"],
        parent: "layout",
    })
}

#[get("/post/<key>")]
fn post(key: i32, connection: db::DbConn) -> Template {

    println!("Getting information of Post for key {}", key);

    let post_details = (Post::get_details(key,&connection));

    println!("title: {}", post_details[0].title);

    Template::render("post", &PostContext{
        title:"Post details",
        name:None,
        items:post_details,
        parent:"layout",

    })


}


#[get("/")]
fn posts(connection: db::DbConn) -> Template {
    println!("getting posts");
    let my_post =  (Post::read(&connection));

    for i in 0..my_post.len(){
        println!("{}", my_post[i].title.to_string());
    }

    Template::render("posts", &PostContext{
        title:"My posts",
        name:None,
        items: my_post,
        parent: "layout",
    })


    //Json(my_post)

}


#[catch(404)]
fn not_found(req: &Request) -> Template{
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn wow_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output
) -> HelperResult {
    if let Some(param) = h.param(0){
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }
    Ok(())
}
fn rocket() -> rocket::Rocket{
    rocket::ignite()
            .manage(db::init_pool())
            .mount("/", StaticFiles::from("static/"))
            .mount("/", routes![index, hello,about])
            .mount("/", routes![posts])
            .mount("/", routes![post])
           .register(catchers![not_found])
           .attach(Template::custom(|engines|{
               engines.handlebars.register_helper("wow", Box::new(wow_helper));

           }))
       }



fn main() {
    //rocket::ignite().mount("/", routes![index]).launch();
    rocket().launch();
}

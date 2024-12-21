use actix::Addr;
use actix_session::Session;
use actix_web::web::{self, Data};
use actix_web::{get,HttpResponse, Responder};
use log::info;
use super::{builder::{Body, HtmlPage, Synthesizable}, getters::{get_lo_index_body_individual, get_lo_index_head}};

use crate::db::messages::{GetBelongingGroupsName, GetJoinCode, GetMyGroupName, IsUserGroupOwner};
use crate::db::utils::{AppState, DbActor};
use crate::html_synthesis::getters::{get_li_basic_head, get_li_belonging_groups_head, get_li_create_group_individual, get_li_create_group_scripts, get_li_crud_expenses_controll, get_li_groups_head, get_li_groups_scripts, get_li_header, get_li_home_individual, get_li_home_scripts, get_li_join_group_head, get_li_join_group_individual, get_li_join_group_scripts, get_li_manage_groups_head, get_li_manage_groups_individual, get_li_manage_groups_scripts, get_li_support_individual, get_lo_login_body_individual, get_lo_login_script, get_lo_register_body_individual, get_lo_register_head, get_lo_register_script, get_lo_support_body_individual};
use crate::sessions::session;

use super::getters::{get_footer, get_header};
use regex::Regex;

#[get("logged_out/home")]
pub async fn get_logged_out_index(state: Data<AppState>) -> impl Responder{
    let header = match get_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let individual = match get_lo_index_body_individual(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_body_individual".to_string()),
    };
   
    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };
    
    let body = Body{
        header,
        individual,
        footer,
        scripts: "".to_string(),
    };

    let head = match get_lo_index_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}

#[get("logged_out/support")]
pub async fn get_logged_out_support(state: Data<AppState>) -> impl Responder{
    let header = match get_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let individual = match get_lo_support_body_individual(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_body_individual".to_string()),
    };
   
    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };
    
    let body = Body{
        header,
        individual,
        footer,
        scripts: "".to_string(),
    };

    let head = match get_lo_index_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}


#[get("logged_out/register")]
pub async fn get_logged_out_register(state: Data<AppState>) -> impl Responder{
    let header = match get_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let individual = match get_lo_register_body_individual(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_body_individual".to_string()),
    };
   
    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };
    

    let scripts = match get_lo_register_script(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_lo_register_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}

#[get("logged_out/log_in")]
pub async fn get_logged_out_login(state: Data<AppState>) -> impl Responder{
    let header = match get_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let individual = match get_lo_login_body_individual(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_body_individual".to_string()),
    };

    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let scripts = match get_lo_login_script(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_lo_register_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}

#[get("logged_in/home")]
pub async fn get_logged_in_home(state: Data<AppState>, session: Session) -> impl Responder{
    if session::is_logged_in(&session) == false{
        return HttpResponse::Unauthorized().json("you must be logged in to get this resource".to_string())
    }


    let header = match get_li_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let individual = match get_li_home_individual(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_body_individual".to_string()),
    };

    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let scripts = match get_li_home_scripts(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_li_basic_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}


#[get("logged_in/support")]
pub async fn get_logged_in_support(state: Data<AppState>, session: Session) -> impl Responder{
    if session::is_logged_in(&session) == false{
        return HttpResponse::Unauthorized().json("you must be logged in to get this resource".to_string())
    }


    let header = match get_li_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let individual = match get_li_support_individual(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_body_individual".to_string()),
    };

    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let scripts = match get_li_home_scripts(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_li_basic_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}


#[get("logged_in/my_groups")]
pub async fn get_logged_in_my_groups(state: Data<AppState>, session: Session) -> impl Responder{
    if session::is_logged_in(&session) == false{
        return HttpResponse::Unauthorized().json("you must be logged in to get this resource".to_string())
    }

    let mut individual = "<main>".to_string();

    let groups_buttons = match build_my_groups_buttons(state.clone(), session,"my_groups").await{
        Ok(result) => result,
        Err(error) => return error
    };

    individual.push_str(&groups_buttons);
    individual.push_str("</main>");

    let header = match get_li_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let scripts = match get_li_home_scripts(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_li_groups_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack groups_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}


#[get("logged_in/my_groups/{group_name}")]
pub async fn get_logged_in_my_groups_specyfic(state: Data<AppState>, session: Session, group_name: web::Path<String>) -> impl Responder{
    if session::is_logged_in(&session) == false{
        return HttpResponse::Unauthorized().json("you must be logged in to get this resource".to_string())
    }

    let mut individual = "<main>".to_string();

    let groups_buttons = match build_my_groups_buttons_picked(state.clone(), session, "my_groups", group_name.to_string()).await{
        Ok(result) => result,
        Err(error) => return error
    };

    let crud_expenses_buttons = match get_li_crud_expenses_controll(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack CRUD expenses controll".to_string()),
    };

    individual.push_str(&groups_buttons);
    individual.push_str("</div>\n");
    individual.push_str(&format!("<div id=\"charts\">
                                    <div id=\"group_name\">{}</div>\n",group_name));
    individual.push_str("<div id=\"barChart\"></div>\n
                        <div id=\"doughnutChart\"></div>\n
                        </div>\n
                        <div id=\"table\"></div>\n
                        </div>\n");

    
    individual.push_str(&crud_expenses_buttons);    
    individual.push_str("</main>");                
                        

    let header = match get_li_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let scripts = match get_li_groups_scripts(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack scripts".to_string()),
    };

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_li_belonging_groups_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack groups_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}


#[get("logged_in/manage_owned_groups")]
pub async fn get_logged_in_manage_owned_groups(state: Data<AppState>, session: Session) -> impl Responder{
    if session::is_logged_in(&session) == false{
        return HttpResponse::Unauthorized().json("you must be logged in to get this resource".to_string())
    }

    let mut individual = "<main>".to_string();
    let groups_buttons = match build_manage_owned_groups_buttons(state.clone(), session,"manage_owned_groups").await{
        Ok(result) => result,
        Err(error) => return error
    };

    individual.push_str(&groups_buttons);
    individual.push_str("</main>");

    let header = match get_li_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let scripts = match get_li_home_scripts(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack scripts".to_string()),
    };

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_li_manage_groups_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack groups_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}



#[get("logged_in/manage_owned_groups/{group_name}")]
pub async fn get_logged_in_manage_owned_groups_specific(state: Data<AppState>, session: Session, group_name: web::Path<String>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    let user_id = match session::get_id(&session){
        Some(result) => result,
        None => return HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    };
    let join_code = match db.send(IsUserGroupOwner{group_name: group_name.to_string(), usr_id: user_id}).await{
        Ok(true) => {
            match db.send(GetJoinCode{group_name: group_name.to_string()}).await{
                Ok(Ok(response)) => response,
                Ok(Err(_)) => return HttpResponse::NotFound().json("No code for provided data"),
                _ => return HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        Ok(false) => return HttpResponse::BadRequest().json("provided Wrong Data "),
        Err(_) => return HttpResponse::InternalServerError().json("Unable check provided data"),
    };

    let mut individual = "<main>".to_string();
    let groups_buttons = match build_manage_owned_groups_buttons_pciked(state.clone(), session,"manage_owned_groups",group_name.to_string()).await{
        Ok(result) => result,
        Err(error) => return error
    };

    let mut controls = match get_li_manage_groups_individual(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack individiual".to_string()),
    };

    let re = Regex::new(r"(join group code:)\s*").unwrap();
    controls = re.replace(&controls, format!("$1 {}",join_code)).to_string();

    individual.push_str(&groups_buttons);
    individual.push_str(&controls);
    individual.push_str("</main>");

    let header = match get_li_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let scripts = match get_li_manage_groups_scripts(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack scripts".to_string()),
    };
    info!("scripts: {}", scripts);

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_li_manage_groups_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack groups_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}

#[get("logged_in/join_group")]
pub async fn get_logged_in_join_group(state: Data<AppState>, session: Session) -> impl Responder{
    if session::is_logged_in(&session) == false{
        return HttpResponse::Unauthorized().json("you must be logged in to get this resource".to_string())
    }


    let header = match get_li_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let individual = match get_li_join_group_individual(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_body_individual".to_string()),
    };

    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let scripts = match get_li_join_group_scripts(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_li_join_group_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}


#[get("logged_in/create_group")]
pub async fn get_logged_in_create_group(state: Data<AppState>, session: Session) -> impl Responder{
    if session::is_logged_in(&session) == false{
        return HttpResponse::Unauthorized().json("you must be logged in to get this resource".to_string())
    }


    let header = match get_li_header(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack header".to_string()),
    };

    let individual = match get_li_create_group_individual(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_body_individual".to_string()),
    };

    let footer = match get_footer(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let scripts = match get_li_create_group_scripts(state.clone()).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack footer".to_string()),
    };

    let body = Body{
        header,
        individual,
        footer,
        scripts,
    };

    let head = match get_li_join_group_head(state).await {
        Ok(response) => response,
        _ => return HttpResponse::InternalServerError().json("couldn't unpack index_head".to_string()),
    };

    let page = HtmlPage {
        head,
        body
    };
    HttpResponse::Ok().body(page.synthesize())
}





// HELPERS
async fn build_my_groups_buttons(state: Data<AppState>, session: Session, path: &str) -> Result<String,HttpResponse>{
    let mut individual = 
    "   <div id=\"myGroups\" class=\"Page\">\n
            <h2>My Groups</h2>\n
            <div class=\"Buttons\">\n
    ".to_string();

    let db: Addr<DbActor> = state.as_ref().db.clone();
    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(GetBelongingGroupsName{user_id}).await{
                Ok(Ok(response)) => response,
                Ok(Err(_)) => return Err(HttpResponse::NotFound().json("No groups found")),
                _ => return Err(HttpResponse::InternalServerError().json("Unable to retrieve users")),
            }
        }
        None          => return Err(HttpResponse::InternalServerError().json("u must be logged in to get yours groups"))
    }.iter().for_each(|group_name| {
        individual.push_str("<a id=\"");
        individual.push_str(group_name);
        individual.push_str(&format!("\" class=\"notpicked groupButton grow\" href=\"/logged_in/{}/",path));
        individual.push_str(group_name);
        individual.push_str("\"> ");
        individual.push_str(group_name);
        individual.push_str("</a>\n");
    });
    Ok(individual)
}


async fn build_my_groups_buttons_picked(state: Data<AppState>, session: Session, path: &str, picked_button: String) -> Result<String,HttpResponse>{
    let mut individual = 
    "   <div id=\"myGroups\" class=\"Page\">\n
            <h2>My Groups</h2>\n
            <div class=\"Buttons\">\n
    ".to_string();

    let db: Addr<DbActor> = state.as_ref().db.clone();
    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(GetBelongingGroupsName{user_id}).await{
                Ok(Ok(response)) => response,
                Ok(Err(_)) => return Err(HttpResponse::NotFound().json("No groups found")),
                _ => return Err(HttpResponse::InternalServerError().json("Unable to retrieve users")),
            }
        }
        None          => return Err(HttpResponse::InternalServerError().json("u must be logged in to get yours groups"))
    }.iter().for_each(|group_name| {
        individual.push_str("<a id=\"");
        individual.push_str(group_name);

        if group_name == &picked_button{
            individual.push_str(&format!("\" class=\" picked groupButton grow\" href=\"/logged_in/{}/",path));
        } else{
            individual.push_str(&format!("\" class=\"notpicked groupButton grow\" href=\"/logged_in/{}/",path));
        }

        individual.push_str(group_name);
        individual.push_str("\"> ");
        individual.push_str(group_name);
        individual.push_str("</a>\n");
    });
    Ok(individual)
}




async fn build_manage_owned_groups_buttons(state: Data<AppState>, session: Session, path: &str) -> Result<String,HttpResponse>{
    let mut individual = 
    "   <div id=\"myGroups\" class=\"Page\">\n
            <h2>My Groups</h2>\n
            <div class=\"Buttons\">\n
    ".to_string();

    let db: Addr<DbActor> = state.as_ref().db.clone();
    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(GetMyGroupName{user_id}).await{
                Ok(Ok(response)) => response,
                Ok(Err(_)) => return Err(HttpResponse::NotFound().json("No groups found")),
                _ => return Err(HttpResponse::InternalServerError().json("Unable to retrieve users")),
            }
        }
        None          => return Err(HttpResponse::InternalServerError().json("u must be logged in to get yours groups"))
    }.iter().for_each(|group_name| {
        individual.push_str("<a id=\"");
        individual.push_str(group_name);
        individual.push_str(&format!("\" class=\"notpicked groupButton grow\" href=\"/logged_in/{}/",path));
        individual.push_str(group_name);
        individual.push_str("\"> ");
        individual.push_str(group_name);
        individual.push_str("</a>\n");
    });
    Ok(individual)
}

async fn build_manage_owned_groups_buttons_pciked(state: Data<AppState>, session: Session, path: &str, picked_button: String) -> Result<String,HttpResponse>{
    let mut individual = 
    "   <div id=\"myGroups\" class=\"Page\">\n
            <h2>My Groups</h2>\n
            <div class=\"Buttons\">\n
    ".to_string();

    let db: Addr<DbActor> = state.as_ref().db.clone();
    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(GetMyGroupName{user_id}).await{
                Ok(Ok(response)) => response,
                Ok(Err(_)) => return Err(HttpResponse::NotFound().json("No groups found")),
                _ => return Err(HttpResponse::InternalServerError().json("Unable to retrieve users")),
            }
        }
        None          => return Err(HttpResponse::InternalServerError().json("u must be logged in to get yours groups"))
    }.iter().for_each(|group_name| {
        individual.push_str("<a id=\"");
        individual.push_str(group_name);
        if group_name == &picked_button{
            individual.push_str(&format!("\" class=\" picked groupButton grow\" href=\"/logged_in/{}/",path));
        } else{
            individual.push_str(&format!("\" class=\"notpicked groupButton grow\" href=\"/logged_in/{}/",path));
        }
        individual.push_str(group_name);
        individual.push_str("\"> ");
        individual.push_str(group_name);
        individual.push_str("</a>\n");
    });
    Ok(individual)
}
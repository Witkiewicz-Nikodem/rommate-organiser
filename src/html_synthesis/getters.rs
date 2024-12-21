/*
    This module focus on getting HTML code from data base
    It is made only for:
        * error handling
        * getting html by function name not by element name in data base - then its easier to use those functions as VS code will propose proper functions
*/

use actix::{Addr, Response};
use actix_web::web::Data;
use log::info;

use crate::db::{messages::GetHTML, utils::{AppState, DbActor}};

pub async fn get_footer(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "Footer".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no Footer found")),
        _ => Err(info!("coudln't retrieve Footer from db")),
    }
}

pub async fn get_header(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_out_Header".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no Header found")),
        _ => Err(info!("coudln't retrieve Header from db")),
    }
}

pub async fn get_lo_index_head(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_out_index_head".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_out_index_head found")),
        _ => Err(info!("coudln't retrieve logged_out_index_head from db")),
    }
}

pub async fn get_lo_index_body_individual(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_out_index_body_individual".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_out_index_body_individual found")),
        _ => Err(info!("coudln't retrieve logged_out_index_body_individual from db")),
    }
}

pub async fn get_lo_support_body_individual(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_out_support_body_individual".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_out_support_body_individual found")),
        _ => Err(info!("coudln't retrieve logged_out_support_body_individual from db")),
    }
}

pub async fn get_lo_register_body_individual(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_out_register_body_individual".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_out_support_body_individual found")),
        _ => Err(info!("coudln't retrieve logged_out_support_body_individual from db")),
    }
}

pub async fn get_lo_register_head(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_out_form_head".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_out_form_head found")),
        _ => Err(info!("coudln't retrieve logged_out_form_head from db")),
    }
}

pub async fn get_lo_register_script(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_out_register_script".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_out_register_script found")),
        _ => Err(info!("coudln't retrieve logged_out_register_script from db")),
    }
}

pub async fn get_lo_login_body_individual(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_out_login_body_individual".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_out_login_body_individual found")),
        _ => Err(info!("coudln't retrieve logged_out_login_body_individual from db")),
    }
}

pub async fn get_lo_login_script(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_out_login_script".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_out_login_script found")),
        _ => Err(info!("coudln't retrieve logged_out_login_script from db")),
    }
}


// LOGGED IN COMMON ELEMENTS
pub async fn get_li_basic_head(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_basic_head".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_basic_head found")),
        _ => Err(info!("coudln't retrieve logged_in_basic_head from db")),
    }
}

pub async fn get_li_header(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_header".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_header found")),
        _ => Err(info!("coudln't retrieve logged_in_header from db")),
    }
}

// LOGGED IN HOME 

pub async fn get_li_home_individual(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_home_main".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_home_main found")),
        _ => Err(info!("coudln't retrieve logged_in_home_main from db")),
    }
}

pub async fn get_li_home_scripts(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_home_scripts".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_home_scripts found")),
        _ => Err(info!("coudln't retrieve logged_in_home_scripts from db")),
    }
}

// LOGGED IN support

pub async fn get_li_support_individual(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_support_main".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_support_main found")),
        _ => Err(info!("coudln't retrieve logged_in_support_main from db")),
    }
}

// LOGGED IN BELONGING GROUPS

pub async fn get_li_groups_head(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_belonging_groups_head".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_belonging_groups_head found")),
        _ => Err(info!("coudln't retrieve logged_in_belonging_groups_head from db")),
    }
}

pub async fn get_li_belonging_groups_head(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_belonging_groups_specific_head".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_belonging_groups_specific_head found")),
        _ => Err(info!("coudln't retrieve logged_in_belonging_groups_specific_head from db")),
    }
}

pub async fn get_li_groups_scripts(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_belonging_groups_scripts".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_belonging_groups_scripts found")),
        _ => Err(info!("coudln't retrieve logged_in_belonging_groups_scripts from db")),
    }
}


// LOGGED IN MANAGE GROUPS
pub async fn get_li_manage_groups_head(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_manage_groups_head".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_manage_groups_head found")),
        _ => Err(info!("coudln't retrieve logged_in_manage_groups_head from db")),
    }
}

pub async fn get_li_manage_groups_individual(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_manage_groups_individual".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_manage_groups_individual found")),
        _ => Err(info!("coudln't retrieve logged_in_manage_groups_individual from db")),
    }
}

pub async fn get_li_manage_groups_scripts(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_manage_groups_scripts".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_manage_groups_scripts found")),
        _ => Err(info!("coudln't retrieve logged_in_manage_groups_scripts from db")),
    }
}

// JOIN GROUP
pub async fn get_li_join_group_individual(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_join_group_individual".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_join_group_individual found")),
        _ => Err(info!("coudln't retrieve logged_in_join_group_individual from db")),
    }
}

pub async fn get_li_join_group_scripts(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_join_group_scripts".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_join_group_scripts found")),
        _ => Err(info!("coudln't retrieve logged_in_join_group_scripts from db")),
    }
}

pub async fn get_li_join_group_head(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_join_group_head".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_join_group_head found")),
        _ => Err(info!("coudln't retrieve logged_in_join_group_head from db")),
    }
}


// Create GROUP
pub async fn get_li_create_group_individual(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_create_group_individual".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_create_group_individual found")),
        _ => Err(info!("coudln't retrieve logged_in_create_group_individual from db")),
    }
}

pub async fn get_li_create_group_scripts(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_create_group_scripts".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_create_group_scripts found")),
        _ => Err(info!("coudln't retrieve logged_in_create_group_scripts from db")),
    }
}

pub async fn get_li_crud_expenses_controll(state: Data<AppState>) -> Result<String,()>{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let get_footer_element = GetHTML{html_object_name: "logged_in_belonging_groups_CRUD_expenses".to_string()};
    match db.send(get_footer_element).await{
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(info!("no logged_in_belonging_groups_CRUD_expenses found")),
        _ => Err(info!("coudln't retrieve logged_in_belonging_groups_CRUD_expenses from db")),
    }
}
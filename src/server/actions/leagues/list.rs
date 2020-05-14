use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LeagueListRequest {
    game_id: String
}

#[derive(Serialize)]
pub struct LeagueListResponse<'c> {
    pub leagues: Vec<LeagueListDto<'c>>
}

#[derive(Serialize)]
pub struct LeagueListDto<'c> {
    pub name: &'c str
}

pub async fn league_list_action(route_params: web::Path<LeagueListRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let simulator_data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let leagues = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .flat_map(|cn| &cn.leagues);
    
    let result = LeagueListResponse{
        leagues: leagues.map(|c| LeagueListDto {
            name: &c.name
        }).collect()
    };
    
    Ok(HttpResponse::Ok().json(result))
}

mod response;
use std::collections::BTreeMap;
use crate::config_file::model::node_cost::NodeCost;
use crate::config_file;
use crate::config_file::ConfigurationControl;
use actix_web::middleware::Logger;
use env_logger::Env;
use response::{ClusterCostResponse};
use crate::cost::*;
use crate::cluster::*;
use crate::command_control;
use crate::command_control::{CmdCtl, PredictCost};
use actix_web::{HttpRequest, Responder, HttpResponse, web, App, HttpServer};
use structopt::StructOpt;


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Daemeon {
}

impl Daemeon {
    pub fn new(config: ConfigurationControl) -> Daemeon {
        Default::default()
    }

    pub fn to_json(order: CostItem) -> ClusterCostResponse {
        let mut json = ClusterCostResponse::new();
        json.set_attributes(order);
        json
    }

    pub fn to_json_with_error(error: String) -> ClusterCostResponse {
        let mut json = ClusterCostResponse::new();
        json.set_error(error);
        json
    }

    pub async fn get_prediction(data: web::Data<ConfigurationControl>, req: HttpRequest) -> impl Responder {
        let (node_type, cpu, memory, scale): (String, f32, f32, i32) = req.match_info().load().unwrap();
        let mut options = CmdCtl::from_args();
        let mut json = ClusterCostResponse::new();
        let mut prediction = PredictCost {
            node_type,
            cpu,
            memory,
            scale
        };

        if let Ok(result) = CostRunner::run(&data, &prediction).await {
            json = Daemeon::to_json(result);
        }

        HttpResponse::Ok().json(json)
    }

    pub async fn run_as_daemeon(&mut self) -> std::io::Result<()> {
        let config = web::Data::new(config_file::ConfigurationControl::new().load());

        let options = command_control::CmdCtl::from_args();
        let host = format!("{}:{}", options.host, options.port);

        println!("Listening {:#?}", host);

        env_logger::from_env(Env::default().default_filter_or("info")).init();

        HttpServer::new(move || {
            App::new()
                .app_data(config.clone())
                .wrap(Logger::default())
                .wrap(Logger::new("%a %P %{User-Agent}i"))
                .route("/predict/{node_type}/{cpu}/{memory}/{scale}", web::get().to( Daemeon::get_prediction ))
        })
        .bind(host)?
        .run()
        .await
    }

}

use std::time::Duration;

use rusqlite::Connection;

use crate::{
    json_format::{PlayerStats, Vehicles, Weapons},
    sql::{delete_user, init_db, insert_user, query_user},
    stats_api::{get_stats, get_vehicles, get_weapons},
};

#[derive(Debug)]
pub enum StateRequest {
    GetStats { ea_id: String },
    GetVehicles { ea_id: String },
    GetWeapons { ea_id: String },
    InsertUser { user_id: String, ea_id: String },
    QueryUser { user_id: String },
    DeleteUser { user_id: String },
    Stop,
}

#[derive(Debug)]
pub enum StateResponse {
    Ok,
    Stats(PlayerStats),
    Vehicles(Vehicles),
    Weapons(Weapons),
    EaUser(String),
    DatabaseError(rusqlite::Error),
    NetworkError(reqwest::Error),
}

pub type Pipe = (
    tokio::sync::oneshot::Sender<StateResponse>,
    tokio::sync::oneshot::Receiver<StateResponse>,
);

pub type StatePipe = (StateRequest, tokio::sync::oneshot::Sender<StateResponse>);

pub type ProducerChan = tokio::sync::mpsc::Sender<StatePipe>;

pub type ConsumerChan = tokio::sync::mpsc::Receiver<StatePipe>;

pub async fn req(chan: ProducerChan, r: StateRequest) -> StateResponse {
    let (p_tx, p_rx): Pipe = tokio::sync::oneshot::channel();
    chan.send((r, p_tx)).await.unwrap();
    // TODO: error handling
    let resp = p_rx.await.unwrap();
    resp
}

pub async fn backend(mut chan: ConsumerChan) {
    // init clients
    let db_conn = Connection::open("Users.db").expect("error open db");
    init_db(&db_conn).expect("Error to create the table");
    let qwq = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()
        .expect("error create reqwest client");
    // waiting for requests
    'lp: loop {
        let (req, pipe) = if let Some(x) = chan.recv().await {
            x
        } else {
            log::error!("channel closed");
            break 'lp;
        };
        let qwq = qwq.clone();
        match req {
            StateRequest::GetWeapons { ea_id } => {
                let resp = match get_weapons(qwq, &ea_id).await {
                    Ok(x) => StateResponse::Weapons(x),
                    Err(e) => {
                        eprintln!("Network Error: {}", e);
                        StateResponse::NetworkError(e)
                    }
                };
                pipe.send(resp).unwrap();
            }

            StateRequest::GetVehicles { ea_id } => {
                let resp = match get_vehicles(qwq, &ea_id).await {
                    Ok(x) => StateResponse::Vehicles(x),
                    Err(e) => {
                        eprintln!("Network Error: {}", e);
                        StateResponse::NetworkError(e)
                    }
                };
                pipe.send(resp).unwrap();
            }
            StateRequest::GetStats { ea_id } => {
                let resp = match get_stats(qwq, &ea_id).await {
                    Ok(x) => StateResponse::Stats(x),
                    Err(e) => {
                        eprintln!("Network Error: {}", e);
                        StateResponse::NetworkError(e)
                    }
                };
                pipe.send(resp).unwrap();
            }
            StateRequest::InsertUser { user_id, ea_id } => {
                // todo: sql operations should be async
                let resp = match insert_user(&db_conn, &user_id, &ea_id) {
                    Ok(_x) => StateResponse::Ok,
                    Err(e) => {
                        eprintln!("Database Error: {}", e);
                        StateResponse::DatabaseError(e)
                    }
                };
                pipe.send(resp).unwrap();
            }
            StateRequest::QueryUser { user_id } => {
                let resp = match query_user(&db_conn, &user_id) {
                    Ok(x) => StateResponse::EaUser(x),
                    Err(e) => {
                        eprintln!("Database Error: {}", e);
                        StateResponse::DatabaseError(e)
                    }
                };
                pipe.send(resp).unwrap();
            }
            StateRequest::DeleteUser { user_id } => {
                let resp = match delete_user(&db_conn, &user_id) {
                    Ok(_x) => StateResponse::Ok,
                    Err(e) => {
                        eprintln!("Database Error: {}", e);
                        StateResponse::DatabaseError(e)
                    }
                };
                pipe.send(resp).unwrap();
            }
            StateRequest::Stop => {
                // 假设它永远会成功.jpg
                pipe.send(StateResponse::Ok).unwrap();
                log::error!("`Stop` request received");
                break 'lp;
            }
        }
    }
    log::warn!("backend exited");
}

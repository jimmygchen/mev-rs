use crate::{
    blinded_block_provider::{BlindedBlockProvider, Error},
    types::{
        BidRequest, ExecutionPayload, SignedBlindedBeaconBlock, SignedBuilderBid,
        SignedValidatorRegistration,
    },
    ExecutionPayloadBellatrix, SignedBlindedBeaconBlockBellatrix, SignedBuilderBidBellatrix,
};
use axum::http::HeaderMap;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use beacon_api_client::{ApiError, ConsensusVersion, Value};
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
};

const CONSENSUS_VERSION_HEADER: &str = "Eth-Consensus-Version";

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let message = self.to_string();
        let code = match self {
            Self::Internal(..) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };
        (code, Json(ApiError { code, message })).into_response()
    }
}

async fn handle_status_check() -> impl IntoResponse {
    tracing::debug!("status check");
    StatusCode::OK
}

async fn handle_validator_registration<B: BlindedBlockProvider>(
    Json(mut registrations): Json<Vec<SignedValidatorRegistration>>,
    Extension(builder): Extension<B>,
) -> Result<(), Error> {
    tracing::debug!("processing registrations {registrations:?}");

    builder.register_validators(&mut registrations).await.map_err(From::from)
}

async fn handle_fetch_bid<B: BlindedBlockProvider>(
    Path(bid_request): Path<BidRequest>,
    Extension(builder): Extension<B>,
    headers: HeaderMap,
) -> Result<Json<Value<impl SignedBuilderBid<dyn ExecutionPayload>>>, Error> {
    tracing::debug!("fetching best bid for block for request {bid_request:?}");

    let consensus_version = headers.get(CONSENSUS_VERSION_HEADER).map(|val| val.to_str()?);
    let signed_bid_result = match consensus_version? {
        "bellatrix" => {
            builder
                .fetch_best_bid::<ExecutionPayloadBellatrix, SignedBuilderBidBellatrix>(
                    &bid_request,
                    consensus_version,
                )
                .await
        }
        "capella" => Err("fork not supported"),
        _ => Err(format!("Missing {} header", CONSENSUS_VERSION_HEADER)),
    };

    let version = serde_json::to_value(ConsensusVersion::Bellatrix).unwrap();
    Ok(Json(Value {
        meta: HashMap::from_iter([("version".to_string(), version)]),
        data: signed_bid_result?,
    }))
}

async fn handle_open_bid<B: BlindedBlockProvider>(
    Json(mut block): Json<impl SignedBlindedBeaconBlock>,
    Extension(builder): Extension<B>,
    headers: HeaderMap,
) -> Result<Json<Value<impl ExecutionPayload>>, Error> {
    tracing::debug!("opening bid for block {block:?}");

    let consensus_version = headers.get(CONSENSUS_VERSION_HEADER).map(|val| val.to_str()?);
    let open_bid_result = match consensus_version? {
        "bellatrix" => {
            builder
                .open_bid::<ExecutionPayloadBellatrix, SignedBlindedBeaconBlockBellatrix>(
                    &mut (block as SignedBlindedBeaconBlockBellatrix),
                )
                .await?
        }
        "capella" => Err("fork not supported"),
        _ => Err(format!("Missing {} header", CONSENSUS_VERSION_HEADER)),
    };

    let version = serde_json::to_value(ConsensusVersion::Bellatrix).unwrap();
    Ok(Json(Value {
        meta: HashMap::from_iter([("version".to_string(), version)]),
        data: open_bid_result?,
    }))
}

pub struct Server<B: BlindedBlockProvider> {
    host: Ipv4Addr,
    port: u16,
    builder: B,
}

impl<B: BlindedBlockProvider + Clone + Send + Sync + 'static> Server<B> {
    pub fn new(host: Ipv4Addr, port: u16, builder: B) -> Self {
        Self { host, port, builder }
    }

    pub async fn run(&self) {
        let router = Router::new()
            .route("/eth/v1/builder/status", get(handle_status_check))
            .route("/eth/v1/builder/validators", post(handle_validator_registration::<B>))
            .route(
                "/eth/v1/builder/header/:slot/:parent_hash/:public_key",
                get(handle_fetch_bid::<B>),
            )
            .route("/eth/v1/builder/blinded_blocks", post(handle_open_bid::<B>))
            .layer(Extension(self.builder.clone()));
        let addr = SocketAddr::from((self.host, self.port));
        let server = axum::Server::bind(&addr).serve(router.into_make_service());

        tracing::info!("listening at {addr}...");
        if let Err(err) = server.await {
            tracing::error!("error while listening for incoming: {err}")
        }
    }
}

use crate::error::ApiError;
use crate::models::{ConvertQuery, ConvertResponse};
use crate::services::convert_currency;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Query, State},
};
use validator::Validate;

pub async fn convert_handler(
    State(state): State<AppState>,
    Query(params): Query<ConvertQuery>,
) -> Result<Json<ConvertResponse>, ApiError> {
    params
        .validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    let amount = params.parse_amount()?;
    let from = params.from.to_uppercase();
    let to = params.to.to_uppercase();

    let rates = state.current_rates().await?;
    let (result, rate) = convert_currency(&rates, &from, &to, amount)?;

    Ok(Json(ConvertResponse {
        from,
        to,
        amount,
        result,
        rate,
        date: rates.date,
    }))
}

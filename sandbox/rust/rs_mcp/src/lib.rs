use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::time::Duration;

pub const WEATHER_NOW_TOOL: &str = "weather_now";
pub const PUBLIC_IP_TOOL: &str = "public_ip";

pub fn handle_request(request: &Value) -> Value {
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    let method = request.get("method").and_then(Value::as_str).unwrap_or("");

    match method {
        "initialize" => success(
            id,
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "rs_mcp", "version": env!("CARGO_PKG_VERSION") }
            }),
        ),
        "tools/list" => success(id, json!({ "tools": tool_definitions() })),
        "tools/call" => call_tool(id, request),
        _ => error(id, -32601, "Method not found"),
    }
}

fn tool_definitions() -> Value {
    json!([
        {
            "name": WEATHER_NOW_TOOL,
            "description": "Fetch current weather for a city using Open-Meteo.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "city": {
                        "type": "string",
                        "description": "City name, for example Amsterdam."
                    }
                },
                "required": ["city"]
            }
        },
        {
            "name": PUBLIC_IP_TOOL,
            "description": "Fetch the public IP address seen by an external service.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }
    ])
}

fn call_tool(id: Value, request: &Value) -> Value {
    let params = request.get("params").unwrap_or(&Value::Null);
    let name = params.get("name").and_then(Value::as_str).unwrap_or("");
    let args = params.get("arguments").unwrap_or(&Value::Null);

    match name {
        WEATHER_NOW_TOOL => call_weather_now(id, args),
        PUBLIC_IP_TOOL => call_public_ip(id),
        _ => error(id, -32602, "Unknown tool"),
    }
}

fn call_weather_now(id: Value, args: &Value) -> Value {
    let Some(city) = args.get("city").and_then(Value::as_str).map(str::trim) else {
        return error(id, -32602, "Missing string argument: city");
    };
    if city.is_empty() {
        return error(id, -32602, "Missing string argument: city");
    }

    match weather_now(city) {
        Ok(result) => success(id, result),
        Err(message) => error(id, -32000, &message),
    }
}

fn call_public_ip(id: Value) -> Value {
    match public_ip() {
        Ok(result) => success(id, result),
        Err(message) => error(id, -32000, &message),
    }
}

fn weather_now(city: &str) -> Result<Value, String> {
    let client = http_client()?;
    let geocode: Value = client
        .get("https://geocoding-api.open-meteo.com/v1/search")
        .query(&[
            ("name", city),
            ("count", "1"),
            ("language", "en"),
            ("format", "json"),
        ])
        .send()
        .map_err(|err| format!("Failed to search city: {err}"))?
        .error_for_status()
        .map_err(|err| format!("City lookup failed: {err}"))?
        .json()
        .map_err(|err| format!("Failed to parse city lookup response: {err}"))?;

    let location = geocode
        .get("results")
        .and_then(Value::as_array)
        .and_then(|results| results.first())
        .ok_or_else(|| format!("No location found for city: {city}"))?;
    let latitude = location
        .get("latitude")
        .and_then(Value::as_f64)
        .ok_or("City lookup did not include latitude")?;
    let longitude = location
        .get("longitude")
        .and_then(Value::as_f64)
        .ok_or("City lookup did not include longitude")?;
    let resolved_name = location.get("name").and_then(Value::as_str).unwrap_or(city);
    let country = location
        .get("country")
        .and_then(Value::as_str)
        .unwrap_or("");

    let forecast: Value = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude", latitude.to_string()),
            ("longitude", longitude.to_string()),
            (
                "current",
                "temperature_2m,weather_code,wind_speed_10m".to_string(),
            ),
        ])
        .send()
        .map_err(|err| format!("Failed to fetch weather: {err}"))?
        .error_for_status()
        .map_err(|err| format!("Weather lookup failed: {err}"))?
        .json()
        .map_err(|err| format!("Failed to parse weather response: {err}"))?;

    let current = forecast.get("current").unwrap_or(&Value::Null);
    let weather_code = current.get("weather_code").and_then(Value::as_i64);
    let temperature = current.get("temperature_2m").and_then(Value::as_f64);
    let wind_speed = current.get("wind_speed_10m").and_then(Value::as_f64);
    let timestamp = current.get("time").and_then(Value::as_str).unwrap_or("");

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!(
                "Weather for {resolved_name}{country_suffix}: {temperature_text}, {conditions}, wind {wind_text}",
                country_suffix = if country.is_empty() { String::new() } else { format!(", {country}") },
                temperature_text = temperature.map(|value| format!("{value} °C")).unwrap_or_else(|| "temperature unavailable".to_string()),
                conditions = weather_code.map(weather_description).unwrap_or("conditions unavailable"),
                wind_text = wind_speed.map(|value| format!("{value} km/h")).unwrap_or_else(|| "unavailable".to_string())
            )
        }],
        "city": city,
        "resolved_location": {
            "name": resolved_name,
            "country": country,
            "latitude": latitude,
            "longitude": longitude
        },
        "temperature_celsius": temperature,
        "conditions": weather_code.map(weather_description),
        "weather_code": weather_code,
        "wind_speed_kmh": wind_speed,
        "timestamp": timestamp,
        "source": "Open-Meteo"
    }))
}

fn public_ip() -> Result<Value, String> {
    let client = http_client()?;
    let response: Value = client
        .get("https://api.ipify.org")
        .query(&[("format", "json")])
        .send()
        .map_err(|err| format!("Failed to fetch public IP: {err}"))?
        .error_for_status()
        .map_err(|err| format!("Public IP lookup failed: {err}"))?
        .json()
        .map_err(|err| format!("Failed to parse public IP response: {err}"))?;

    let ip = response
        .get("ip")
        .and_then(Value::as_str)
        .ok_or("Public IP response did not include ip")?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!("Public IP: {ip}")
        }],
        "ip": ip,
        "source": "ipify"
    }))
}

fn http_client() -> Result<Client, String> {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|err| format!("Failed to create HTTP client: {err}"))
}

fn weather_description(code: i64) -> &'static str {
    match code {
        0 => "clear sky",
        1 | 2 | 3 => "partly cloudy",
        45 | 48 => "fog",
        51 | 53 | 55 | 56 | 57 => "drizzle",
        61 | 63 | 65 | 66 | 67 => "rain",
        71 | 73 | 75 | 77 => "snow",
        80 | 81 | 82 => "rain showers",
        85 | 86 => "snow showers",
        95 | 96 | 99 => "thunderstorm",
        _ => "unknown conditions",
    }
}

fn success(id: Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn error(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Prize {
    pub decimo: String,
    pub prize: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JSONResponse {
    tipo_sorteo: char,
    tipo_sorteo_general: char,
    fecha_sorteo: String,
    draw_id_sorteo: String,
    importe_por_defecto: u16,
    premio_especial: PremioEspecial,
    url_listado_oficial: String,
    literal_fecha_sorteo: LiteralFechaSorteo,
    pub compruebe: Vec<Prize>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PremioEspecial {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    numero: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    fraccion: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    serie: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    premio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    fila: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    orden: Option<String>,

    show_folded: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct LiteralFechaSorteo {
    es: String,
    ca: String,
    en: String,
    eu: String,
    gl: String,
    va: String,
}

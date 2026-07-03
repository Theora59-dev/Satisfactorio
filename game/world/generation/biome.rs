use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BiomeConfig {
    pub id: String,
    pub name: String,
    pub temperature_range: [f64; 2],
    pub humidity_range: [f64; 2],
    pub terrain: BiomeTerrainParams,
    pub layers: BiomeLayers,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BiomeTerrainParams {
    pub base_height: f64,
    pub amplitude: f64,
    pub scale: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BiomeLayers {
    pub surface_block: String,
    pub subsurface_block: String,
    pub deep_block: String,
    pub subsurface_depth: i32,
}

#[derive(Deserialize, Debug)]
struct BiomeGenConfig {
    biomes: Vec<BiomeConfig>,
}

#[derive(Clone)]
pub struct BiomeRegistry {
    pub biomes: Vec<BiomeConfig>,
    default_index: usize,
}

impl BiomeRegistry {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let content =
            fs::read_to_string(path.as_ref()).unwrap_or_else(|_| panic!("Failed to read biome config {:?}", path.as_ref()));
        let config: BiomeGenConfig = serde_json::from_str(&content)
            .unwrap_or_else(|e| panic!("Failed to parse biome config {:?}: {}", path.as_ref(), e));

        let default_index = config.biomes.iter().position(|b| b.id == "plains").unwrap_or(0);

        Self {
            biomes: config.biomes,
            default_index,
        }
    }

    /// Parcourt les biomes dans l'ordre ; premier dont la température et l'humidité sont dans les ranges.
    /// Sinon, retourne le biome par défaut (plains).
    pub fn get_biome_index(&self, temperature: f64, humidity: f64) -> usize {
        for (i, biome) in self.biomes.iter().enumerate() {
            if temperature >= biome.temperature_range[0]
                && temperature <= biome.temperature_range[1]
                && humidity >= biome.humidity_range[0]
                && humidity <= biome.humidity_range[1]
            {
                return i;
            }
        }
        self.default_index
    }
}

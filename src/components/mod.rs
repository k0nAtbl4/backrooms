// Экспортируем модуль player
pub mod player;
pub mod camera;

// Реэкспортируем типы из player для удобства
pub use player::Player; 
pub use camera::MainCamera3POV;
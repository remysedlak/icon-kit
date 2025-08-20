pub enum Icon {
    CircleHeatSvgrepoCom,
    CloudRainAlt1SvgrepoCom,
    CloudRainbowSvgrepoCom,
    CloudSunSvgrepoCom,
    CloudSvgrepoCom,
    HurricaneAltSvgrepoCom,
    LinkSvgrepoCom1,
    LinkSvgrepoCom,
}
impl Icon {
    pub fn path(&self) -> &'static str {
        match self {
            Icon::CircleHeatSvgrepoCom => "assets/icons/circle-heat-svgrepo-com.svg",
            Icon::CloudRainAlt1SvgrepoCom => "assets/icons/cloud-rain-alt-1-svgrepo-com.svg",
            Icon::CloudRainbowSvgrepoCom => "assets/icons/cloud-rainbow-svgrepo-com.svg",
            Icon::CloudSunSvgrepoCom => "assets/icons/cloud-sun-svgrepo-com.svg",
            Icon::CloudSvgrepoCom => "assets/icons/cloud-svgrepo-com.svg",
            Icon::HurricaneAltSvgrepoCom => "assets/icons/hurricane-alt-svgrepo-com.svg",
            Icon::LinkSvgrepoCom1 => "assets/icons/link-svgrepo-com(1).svg",
            Icon::LinkSvgrepoCom => "assets/icons/link-svgrepo-com.svg",
        }
    }
}

use super::*;

#[derive(geng::Assets)]
pub struct Assets {
    #[asset(load_with = "load_system_shaders(geng, &base_path)")]
    pub system_shaders: SystemShaders,
}

async fn load_system_shaders(
    geng: &Geng,
    base_path: &std::path::Path,
) -> anyhow::Result<SystemShaders> {
    let base_path = base_path.join("shaders/system/");

    geng.shader_lib().add(
        "common.glsl",
        &<String as geng::LoadAsset>::load(&geng, &base_path.join("common.glsl"))
            .await
            .context(format!("Failed to load common.glsl"))?,
    );

    let json = <String as geng::LoadAsset>::load(geng, &base_path.join("config.json"))
        .await
        .context("Failed to load config.json for system shaders")?;
    let mut system_shaders: SystemShaders =
        serde_json::from_str(&json).context("Failed to parse config.json for system shaders")?;

    let path = system_shaders.field.path.clone();
    let program = <ugli::Program as geng::LoadAsset>::load(&geng, &base_path.join(path.clone()))
        .await
        .context(format!("Failed to load {path}"))?;
    system_shaders.field.program = Some(Rc::new(program));

    let path = system_shaders.flower.path.clone();
    let program = <ugli::Program as geng::LoadAsset>::load(&geng, &base_path.join(path.clone()))
        .await
        .context(format!("Failed to load {path}"))?;
    system_shaders.flower.program = Some(Rc::new(program));

    let path = system_shaders.flower_radius.path.clone();
    let program = <ugli::Program as geng::LoadAsset>::load(&geng, &base_path.join(path.clone()))
        .await
        .context(format!("Failed to load {path}"))?;
    system_shaders.flower_radius.program = Some(Rc::new(program));

    let path = system_shaders.seed.path.clone();
    let program = <ugli::Program as geng::LoadAsset>::load(&geng, &base_path.join(path.clone()))
        .await
        .context(format!("Failed to load {path}"))?;
    system_shaders.seed.program = Some(Rc::new(program));

    let path = system_shaders.flower_particles.path.clone();
    let program = <ugli::Program as geng::LoadAsset>::load(&geng, &base_path.join(path.clone()))
        .await
        .context(format!("Failed to load {path}"))?;
    system_shaders.flower_particles.program = Some(Rc::new(program));

    let path = system_shaders.bind.path.clone();
    let program = <ugli::Program as geng::LoadAsset>::load(&geng, &base_path.join(path.clone()))
        .await
        .context(format!("Failed to load {path}"))?;
    system_shaders.bind.program = Some(Rc::new(program));

    Ok::<_, anyhow::Error>(system_shaders)
}

mod editor;

fn main() -> std::io::Result<()> {
    editor::run_editor()?;
    Ok(())
}

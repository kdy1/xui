#[cfg(test)]
mod tests {
    use stretch::{geometry::Size, style::*};

    #[test]
    fn test() -> Result<(), stretch::Error> {
        let mut stretch = stretch::node::Stretch::new();
        let child = stretch.new_node(
            Style {
                size: Size {
                    width: Dimension::Percent(0.5),
                    height: Dimension::Auto,
                },
                ..Default::default()
            },
            vec![],
        )?;

        let node = stretch.new_node(
            Style {
                size: Size {
                    width: Dimension::Points(100.0),
                    height: Dimension::Points(100.0),
                },
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            vec![child],
        )?;

        stretch.compute_layout(node, Size::undefined())?;
        dbg!(stretch.layout(node)?);

        Ok(())
    }
}

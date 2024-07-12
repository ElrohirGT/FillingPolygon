use std::error::Error;
extern crate nalgebra_glm as glm;

use filling_polygon::{color::Color, framebuffer::Framebuffer};

fn main() -> Result<(), Box<dyn Error>> {
    test_renders()?;

    let mut framebuffer = Framebuffer::new(450, 450);
    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let points = vec![
        glm::Vec3::new(165.0, 380.0, 0.0),
        glm::Vec3::new(185.0, 360.0, 0.0),
        glm::Vec3::new(180.0, 330.0, 0.0),
        glm::Vec3::new(207.0, 345.0, 0.0),
        glm::Vec3::new(233.0, 330.0, 0.0),
        glm::Vec3::new(230.0, 360.0, 0.0),
        glm::Vec3::new(250.0, 380.0, 0.0),
        glm::Vec3::new(220.0, 385.0, 0.0),
        glm::Vec3::new(205.0, 410.0, 0.0),
        glm::Vec3::new(193.0, 383.0, 0.0),
    ];
    // framebuffer.polygon(points, 0xe5de00).paint()?;
    framebuffer.paint_filled_polygon(points, 0xe5de00, 0xffffff)?;
    framebuffer.save("poligon1.bmp")?;

    Ok(())
}

fn test_renders() -> Result<(), Box<dyn Error>> {
    let mut framebuffer = Framebuffer::new(2, 2);

    framebuffer.set_background_color(Color::black());
    framebuffer.clear();

    framebuffer.set_current_color(0xff0000);
    framebuffer.paint_point(glm::Vec3::new(0.0, 0.0, 0.0))?;

    framebuffer.set_current_color(0xffffff);
    framebuffer.paint_point(glm::Vec3::new(1.0, 0.0, 0.0))?;

    framebuffer.set_current_color(0x0000ff);
    framebuffer.paint_point(glm::Vec3::new(0.0, 1.0, 0.0))?;

    framebuffer.set_current_color(0x00ff00);
    framebuffer.paint_point(glm::Vec3::new(1.0, 1.0, 0.0))?;
    framebuffer.save("test_grid.bmp")?;

    let mut framebuffer = Framebuffer::new(3, 3);

    framebuffer.set_background_color(Color::black());
    framebuffer.clear();

    framebuffer.set_current_color(0xff0000);
    framebuffer.paint_point(glm::Vec3::new(0.0, 0.0, 0.0))?;

    framebuffer.set_current_color(0xffffff);
    framebuffer.paint_point(glm::Vec3::new(1.0, 0.0, 0.0))?;

    framebuffer.set_current_color(0x0000ff);
    framebuffer.paint_point(glm::Vec3::new(0.0, 1.0, 0.0))?;

    framebuffer.set_current_color(0x00ff00);
    framebuffer.paint_point(glm::Vec3::new(1.0, 1.0, 0.0))?;
    framebuffer.save("bigger_test_grid.bmp")?;

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(0xADD8E6);
    framebuffer.clear();

    framebuffer.set_current_color(0xFF0000);
    framebuffer.paint_point(glm::Vec3::new(400.0, 300.0, 0.0))?;
    framebuffer.paint_point(glm::Vec3::new(401.0, 300.0, 0.0))?;
    framebuffer.paint_point(glm::Vec3::new(400.0, 301.0, 0.0))?;
    framebuffer.paint_point(glm::Vec3::new(401.0, 301.0, 0.0))?;

    framebuffer.set_current_color(0x00FF00);
    framebuffer.paint_point(glm::Vec3::new(200.0, 150.0, 0.0))?;
    framebuffer.paint_point(glm::Vec3::new(201.0, 150.0, 0.0))?;
    framebuffer.paint_point(glm::Vec3::new(200.0, 151.0, 0.0))?;
    framebuffer.paint_point(glm::Vec3::new(201.0, 151.0, 0.0))?;

    framebuffer.set_current_color(0x0000FF);
    framebuffer.paint_point(glm::Vec3::new(600.0, 450.0, 0.0))?;
    framebuffer.paint_point(glm::Vec3::new(601.0, 450.0, 0.0))?;
    framebuffer.paint_point(glm::Vec3::new(600.0, 451.0, 0.0))?;
    framebuffer.paint_point(glm::Vec3::new(601.0, 451.0, 0.0))?;

    framebuffer.save("canva_test.bmp")?;

    let mut framebuffer = Framebuffer::new(70, 70);
    framebuffer.clear();

    framebuffer.set_background_color(Color::black());

    framebuffer
        .line(
            glm::Vec3::new(0.0, 0.0, 0.0),
            glm::Vec3::new(2.0, 30.0, 0.0),
        )
        .paint()?;

    framebuffer.save("line_test.bmp")?;

    let mut framebuffer = Framebuffer::new(20, 20);
    framebuffer.clear();

    framebuffer
        .line(
            glm::Vec3::new(5.0, 5.0, 0.0),
            glm::Vec3::new(15.0, 5.0, 0.0),
        )
        .paint()?;
    framebuffer
        .line(
            glm::Vec3::new(15.0, 5.0, 0.0),
            glm::Vec3::new(15.0, 15.0, 0.0),
        )
        .paint()?;
    framebuffer
        .line(
            glm::Vec3::new(15.0, 15.0, 0.0),
            glm::Vec3::new(5.0, 15.0, 0.0),
        )
        .paint()?;
    framebuffer
        .line(
            glm::Vec3::new(5.0, 15.0, 0.0),
            glm::Vec3::new(5.0, 5.0, 0.0),
        )
        .paint()?;

    framebuffer.save("square_test.bmp")?;

    let mut framebuffer = Framebuffer::new(300, 300);
    framebuffer.clear();

    framebuffer
        .polygon(vec![
            glm::Vec3::new(50.0, 5.0, 0.0),
            glm::Vec3::new(20.0, 20.0, 0.0),
            glm::Vec3::new(70.0, 7.0, 0.0),
            glm::Vec3::new(150.0, 200.0, 0.0),
        ])
        .paint()?;
    framebuffer.save("polygon_test.bmp")?;

    Ok(())
}

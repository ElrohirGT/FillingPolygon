use std::error::Error;
extern crate nalgebra_glm as glm;

use filling_polygon::{
    color::Color,
    framebuffer::{self, Framebuffer},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut framebuffer = Framebuffer::new(800, 800);
    framebuffer.clear();

    test_renders()?;
    draw_star(&mut framebuffer)?;
    draw_square(&mut framebuffer)?;
    draw_triangle(&mut framebuffer)?;
    draw_teapot(&mut framebuffer)?;
    draw_teapothole(&mut framebuffer)?;

    Ok(())
}

fn draw_teapothole(framebuffer: &mut Framebuffer) -> Result<(), Box<dyn Error>> {
    let points = vec![
        glm::Vec3::new(413.0, 177.0, 0.0),
        glm::Vec3::new(448.0, 159.0, 0.0),
        glm::Vec3::new(502.0, 88.0, 0.0),
        glm::Vec3::new(553.0, 53.0, 0.0),
        glm::Vec3::new(535.0, 36.0, 0.0),
        glm::Vec3::new(676.0, 37.0, 0.0),
        glm::Vec3::new(660.0, 52.0, 0.0),
        glm::Vec3::new(750.0, 145.0, 0.0),
        glm::Vec3::new(761.0, 179.0, 0.0),
        glm::Vec3::new(672.0, 192.0, 0.0),
        glm::Vec3::new(659.0, 214.0, 0.0),
        glm::Vec3::new(615.0, 214.0, 0.0),
        glm::Vec3::new(632.0, 230.0, 0.0),
        glm::Vec3::new(580.0, 230.0, 0.0),
        glm::Vec3::new(597.0, 215.0, 0.0),
        glm::Vec3::new(552.0, 214.0, 0.0),
        glm::Vec3::new(517.0, 144.0, 0.0),
        glm::Vec3::new(466.0, 180.0, 0.0),
    ];

    framebuffer.paint_filled_polygon(points, 0x0a5c36, 0xffffff)?;

    let hole_points = vec![
        glm::Vec3::new(682.0, 175.0, 0.0),
        glm::Vec3::new(708.0, 120.0, 0.0),
        glm::Vec3::new(735.0, 148.0, 0.0),
        glm::Vec3::new(739.0, 170.0, 0.0),
    ];
    framebuffer.paint_filled_polygon(hole_points, 0x000000, 0xffffff)?;

    framebuffer.save("poligon5.bmp")?;

    Ok(())
}

fn draw_teapot(framebuffer: &mut Framebuffer) -> Result<(), Box<dyn Error>> {
    let points = vec![
        glm::Vec3::new(413.0, 177.0, 0.0),
        glm::Vec3::new(448.0, 159.0, 0.0),
        glm::Vec3::new(502.0, 88.0, 0.0),
        glm::Vec3::new(553.0, 53.0, 0.0),
        glm::Vec3::new(535.0, 36.0, 0.0),
        glm::Vec3::new(676.0, 37.0, 0.0),
        glm::Vec3::new(660.0, 52.0, 0.0),
        glm::Vec3::new(750.0, 145.0, 0.0),
        glm::Vec3::new(761.0, 179.0, 0.0),
        glm::Vec3::new(672.0, 192.0, 0.0),
        glm::Vec3::new(659.0, 214.0, 0.0),
        glm::Vec3::new(615.0, 214.0, 0.0),
        glm::Vec3::new(632.0, 230.0, 0.0),
        glm::Vec3::new(580.0, 230.0, 0.0),
        glm::Vec3::new(597.0, 215.0, 0.0),
        glm::Vec3::new(552.0, 214.0, 0.0),
        glm::Vec3::new(517.0, 144.0, 0.0),
        glm::Vec3::new(466.0, 180.0, 0.0),
    ];

    framebuffer.paint_filled_polygon(points, 0x0a5c36, 0xffffff)?;
    framebuffer.save("poligon4.bmp")?;

    Ok(())
}

fn draw_triangle(framebuffer: &mut Framebuffer) -> Result<(), Box<dyn Error>> {
    let points = vec![
        glm::Vec3::new(377.0, 249.0, 0.0),
        glm::Vec3::new(411.0, 197.0, 0.0),
        glm::Vec3::new(436.0, 249.0, 0.0),
    ];
    framebuffer.paint_filled_polygon(points, 0xff0000, 0xffffff)?;
    framebuffer.save("poligon3.bmp")?;

    Ok(())
}

fn draw_square(framebuffer: &mut Framebuffer) -> Result<(), Box<dyn Error>> {
    let points = vec![
        glm::Vec3::new(321.0, 335.0, 0.0),
        glm::Vec3::new(288.0, 286.0, 0.0),
        glm::Vec3::new(339.0, 251.0, 0.0),
        glm::Vec3::new(374.0, 302.0, 0.0),
    ];
    framebuffer.paint_filled_polygon(points, 0x0000ff, 0xffffff)?;
    framebuffer.save("poligon2.bmp")?;

    Ok(())
}

fn draw_star(framebuffer: &mut Framebuffer) -> Result<(), Box<dyn Error>> {
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

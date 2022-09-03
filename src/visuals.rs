use colored::Colorize;

pub fn draw_line(color: impl Into<Option<(u8, u8, u8)>>){
    let color = color.into().unwrap_or((247, 78, 0));
    println!("{}", format!("------------").truecolor(color.0, color.1, color.2));
}

pub fn draw_blocks(){
    let colors = [(0,0,0),(128,0,0),(0,128,0),(128,128,0),
                                    (0,0,128), (128,0,128),(0,128,128),(192,192,192),
                                    (128,128,128),(255,0,0),(0,255,0),(255,255,0),
                                    (0,0,255),(255,0,255),(0,255,0),(0,255,255),
                                    (255,255,255)];
    for c in colors{
        print!("{}",format!("\u{2588}").truecolor(c.0, c.1, c.2));
    }
    println!()
}
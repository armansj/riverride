use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    event,
    
};

fn main() -> std::io::Result<()> {
    

    // or using functions
    let mut sc = stdout();
    
    sc.execute(MoveTo(6,4));
    sc.execute(Print("Styled text here."))?;
    
    
    Ok(())
    
}
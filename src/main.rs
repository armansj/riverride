use std::{io::{stdout, Stdout, Write}, time::Duration};

use crossterm::{
    cursor::MoveTo, event::{self, poll, read, Event, KeyCode}, execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType}, ExecutableCommand, QueueableCommand
    
};



struct World{

    player_C : u16,
    player_l : u16,

}

fn draw(mut sc: &Stdout,world : &World) -> std::io::Result<()> {
    sc.queue(Clear(ClearType::All))?;

    sc.queue(MoveTo(world.player_C,world.player_l))?;
    sc.queue(Print("Styled text here."))?;

    sc.flush();
    Ok(())
}

fn main() -> std::io::Result<()> {


    // or using functions
    let mut sc = stdout(); 
    let (maxc,maxl)=size().unwrap();


    enable_raw_mode()?;
 

    // init the game
    let mut world :World = World{
        player_C : maxc /2 ,
        player_l : maxl - 1,

    };



    loop {

        if poll(Duration::from_millis(10))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            let key = read().unwrap();
            match key {
                Event::Key(event) => {
                    match  event.code {

                        KeyCode::Char('q')=>{
                            break;
                        }
                        KeyCode::Char('w')=>{
                        if(world.player_l > 1 ){
                            world.player_l -=1
                        };
                        }
                        KeyCode::Char('s')=>{
                        if(world.player_l < maxl-1 ){
                            world.player_l +=1
                            };
                            }
                        KeyCode::Char('a')=>{
                        if(world.player_C > 1 ){
                            world.player_C -=1
                                };
                                }
                        KeyCode::Char('d')=>{
                        if(world.player_C < maxc-1 ){
                            world.player_C +=1
                             };
                                    }
                        _=> {},
                        
                    }
                }, 
                _=>{},     
            }
        } else {
            // Timeout expired and no `Event` is available
        }

        // physics()


        draw(&sc,&world)?;
    }
    
    disable_raw_mode();
    Ok(())
    
}
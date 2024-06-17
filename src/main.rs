use std::{io::{stdout, Result, Stdout, Write}, num::ParseIntError, time::Duration};
use std::{thread,time};
use rand::{thread_rng, Rng};
use material_icons::{Icon, icon_to_char};
use crossterm::{
    cursor::{Hide, MoveTo, Show}, event::{self, poll, read, Event, KeyCode}, execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType}, ExecutableCommand, QueueableCommand
    
};

struct World{

    player_C : u16,
    player_l : u16,
    maxc:u16,
    maxl:u16,
    map: Vec<(u16,u16)>,
    died : bool,
    next_start: u16,
    next_end: u16,

}

fn draw(mut sc: &Stdout,world : &World) -> std::io::Result<()> {


    sc.queue(Clear(ClearType::All))?;

    //draw the map

    for l in 0..world.map.len(){

        sc.queue(MoveTo(0,l as u16))?;
        sc.queue(Print("+".repeat(world.map[l].0 as usize)))?;


        sc.queue(MoveTo(world.map[l].1,l as u16))?;
        sc.queue(Print("+".repeat((world.maxc - world.map[l].1) as usize)))?;

    }

    let martini_emoji = '\u{1F6EC}';
    // draw the player
    sc.queue(MoveTo(world.player_C,world.player_l))?;
  
    sc.queue(Print(martini_emoji))?;
    

    sc.flush();
    Ok(())
}



fn physics(mut world : World) -> Result<World> {

    //check if player died
    if (world.player_C <= world.map[world.player_l as usize].0 ||  world.player_C >= world.map[world.player_l as usize].1){
        world.died = true;
    }

    for l in (0..world.map.len()-1).rev(){
        world.map[l+1] = world.map[l];
    }
    if(world.next_end < world.map[0].1){
        world.map[0].1 -=1;
    }
    if(world.next_end>world.map[0].1){
        world.map[0].1 +=1;
    }
    if(world.next_start < world.map[0].0){
        world.map[0].0 -=1;
    }
    if(world.next_start>world.map[0].0){
        world.map[0].0 +=1;
    }
    let mut rng = thread_rng();

    if  rng.gen_range(0..10)>7{
        
  

    // TODO : possible bug might comes out 
    if world.next_start == world.map[0].0 && world.next_end == world.map[0].1{

        world.next_start = rng.gen_range(world.map[0].0 - 5..world.map[0].1 - 5);
        world.next_end = rng.gen_range(world.map[0].0 + 5..world.map[0].1 + 5);
        if world.next_end - world.next_start <=3{
            world.next_start -=3;
        }


       }
    }

    
    
    
    Ok(world)
    
}

fn main() -> std::io::Result<()> {


    // or using functions
    let mut sc = stdout(); 
    let (maxc,maxl)=size().unwrap();
    sc.execute(Hide);


    enable_raw_mode()?;
 

    // init the game
    let mut world :World = World{
        player_C : maxc /2 ,
        player_l : maxl - 1,
        maxc:maxc,
        maxl:maxl,
        map: vec![((maxc/2)-5,(maxc/2)+5);maxl as usize],
        died:false,
        next_end: maxc/2 +10,
        next_start : maxc/2 -10,

    };



    while  !world.died{

        if poll(Duration::from_millis(10))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            let key = read().unwrap();
            while poll(Duration::from_millis(0)).unwrap() {
                let _ = read();
            }
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
        world = physics(world).unwrap();


        draw(&sc,&world)?;
        thread::sleep(time::Duration::from_millis(100));
    }
    // TODO: check for die and show a message
    sc.execute(Show);
    disable_raw_mode();
    sc.execute(Clear(ClearType::All))?;
    sc.execute(Print("Thanks for playing!"))?;

    Ok(())
    
}
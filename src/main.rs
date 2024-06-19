use std::{io::{stdout, Result, Stdout, Write}, num::ParseIntError, time::Duration};
use std::{thread,time};
use rand::{thread_rng, Rng};
use material_icons::{Icon, icon_to_char};
use crossterm::{
    cursor::{Hide, MoveTo, Show}, event::{self, poll, read, Event, KeyCode}, execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType}, ExecutableCommand, QueueableCommand
    
};


struct  Enemy{
    l:u16,
    c:u16,
    
}

struct Bullet{
    l:u16,
    c:u16,
    energy:u16,
}


struct World{

    player_C : u16,
    player_l : u16,
    maxc:u16,
    maxl:u16,
    map: Vec<(u16,u16)>,
    died : bool,
    next_start: u16,
    next_end: u16,
    enemy :Vec<Enemy>,
    bullet:Vec<Bullet>,

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

    // drwa the enemy

    for e in &world.enemy{

        sc.queue(MoveTo(e.c, e.l as u16))?;
        sc.queue(Print("E"))?;

    }


    // draw bullet

    for b in &world.bullet{

        sc.queue(MoveTo(b.c, b.l as u16))?;
        sc.queue(Print("."))?;

    }






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





    // if enemy hit something

    for i in (0..world.enemy.len()).rev() {

        if world.enemy[i].l == world.player_l && world.enemy[i].c == world.player_C{

            // palyer hit enemy
            world.died = true;

        }
        for j in  (0..world.bullet.len()).rev(){
            if(world.bullet[j].l == world.enemy[i].l && world.bullet[j].c == world.enemy[i].c){
              world.enemy.remove(i);
            }


        }





       }




       // move the bullet

       for i in  (0..world.bullet.len()).rev(){

        if(world.bullet[i].energy == 0 || world.bullet[i].l<2){
            world.bullet.remove(i);
        }else {
            world.bullet[i].energy -=1;
            world.bullet[i].l -=2;
            if(world.bullet[i].l<1){world.bullet[i].energy=0}
        }

       }



    let mut rng = thread_rng();

    if  rng.gen_range(0..10)>7{
        
  

    // TODO : possible bug might comes out 
    if world.next_start == world.map[0].0 && world.next_end == world.map[0].1{

        world.next_start = rng.gen_range(world.map[0].0 - 5..world.map[0].1 - 4);
        world.next_end = rng.gen_range(world.map[0].0 + 5..world.map[0].1 + 4);
        if world.next_end - world.next_start <=3{
            world.next_start -=3;
        }


       }


       for i in (0..world.enemy.len()).rev() {

        if world.enemy[i].l < world.maxl{
            world.enemy[i].l+=1;

        }else {
            world.enemy.remove(i);
        }


       }

       if(rng.gen_range(0..10)>=9){
        let new_c =rng.gen_range(world.map[0].0..world.map[1].1);
        world.enemy.push(
            Enemy { l: 0, c: new_c }
        )
       }



    }

    
    
    
    Ok(world)
    
}



// MOVE and add enemy

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
        enemy:vec![],
        bullet:vec![],

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

                        KeyCode::Char(' ')=>{
                            if(world.bullet.len() == 0 ){
                                world.bullet.push(Bullet{
                                    l:world.player_l+1,
                                    c:world.player_C,
                                    energy:10,
                                })
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
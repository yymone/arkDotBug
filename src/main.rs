use ark_std::{end_timer, start_timer};
fn main() {
    for _ in 0..10 {
        let timer1 = start_timer!(|| "test panic 1");

        let handle = std::thread::spawn(move || {
            let timer2 = start_timer!(|| "test panic 2");
            let timer3 = start_timer!(|| "test panic 3");
            let timer4 = start_timer!(|| "test panic 4");
            let timer5 = start_timer!(|| "test panic 5");
            let timer6 = start_timer!(|| "test panic 6");
            end_timer!(timer6);
            //Simulator do task and panic
            if true {
                panic!();
            }

            end_timer!(timer2);
            end_timer!(timer3);
            end_timer!(timer4);
            end_timer!(timer5);
            
        });

        let _ = match handle.join() {
            Ok(_) => {
                println!("Ok")
            }
            Err(e) => {
                let msg = if let Some(msg) = e.downcast_ref::<&'static str>() {
                    format!("Test panic {:?}", msg.to_string())
                } else if let Some(msg) = e.downcast_ref::<String>() {
                    format!("Test panic {:?}", msg.clone())
                } else {
                    format!("Test panic {:?}", e)
                };
                println!("Ignore panic error in spawn thread. {:?}", msg);
            }
        };

        end_timer!(timer1);
    }
}
